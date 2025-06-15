use color_eyre::eyre::{Ok, Result};
use ratatui::{
    crossterm::event::{self, Event, KeyEvent},
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    text::{Line, ToSpan},
    widgets::{Block, BorderType, List, ListItem, Padding, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use crate::app_state::AppState;

// TODO: split in different files
// TODO: better UI
// TODO: custom colors
enum FormAction {
    None,
    Submit,
    Escape,
}

pub fn init(state: &mut AppState) -> Result<()> {
    // TODO: fix this
    let _ = state.update();
    color_eyre::install()?;

    let terminal = ratatui::init();

    let result = run(terminal, state);

    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    // loop where we are rendering and handling the input
    loop {
        // Rendering
        // TODO: разобраться с этим
        terminal.draw(|f| render(f, app_state))?;

        // Input handling
        if let Event::Key(key) = event::read()? {
            if app_state.is_add_new {
                match handle_new_todo(key, app_state) {
                    FormAction::None => {}
                    FormAction::Submit => {
                        app_state.is_add_new = false;
                        // TODO: fix this
                        let _ = app_state.db.add_task(&app_state.input_string);
                        let _ = app_state.update();
                        app_state.input_string.clear();
                    }
                    FormAction::Escape => {
                        app_state.is_add_new = false;
                        app_state.input_string.clear();
                    }
                }
            } else {
                if handle_key(key, app_state) {
                    break;
                }
            }
        }
    }

    Ok(())
}

fn handle_new_todo(key: KeyEvent, app_state: &mut AppState) -> FormAction {
    match key.code {
        event::KeyCode::Char(char) => {
            app_state.input_string.push(char);
        }
        event::KeyCode::Backspace => {
            app_state.input_string.pop();
        }
        event::KeyCode::Esc => {
            return FormAction::Escape;
        }
        event::KeyCode::Enter => {
            return FormAction::Submit;
        }

        _ => {}
    }

    FormAction::None
}

fn handle_key(key: KeyEvent, app_state: &mut AppState) -> bool {
    match key.code {
        event::KeyCode::Esc => {
            // * exiting the program with esc keybinding
            return true;
        }
        event::KeyCode::Enter => {
            if let Some(index) = app_state.list_state.selected() {
                if let Some(item) = app_state.items.get_mut(index) {
                    let _ = app_state.db.mark_task(&item.title);
                    item.done = !item.done;
                }
            }
        }
        event::KeyCode::Backspace => {
            if let Some(index) = app_state.list_state.selected() {
                if let Some(task) = app_state.items.get(index) {
                    // TODO: error handling
                    let _ = app_state.db.remove_task(&task.title);
                    app_state.items.remove(index);
                }
            }
        }
        event::KeyCode::Char(char) => match char {
            'c' => {
                app_state.is_add_new = true;
            }
            'k' => {
                app_state.list_state.select_previous();
            }
            'j' => {
                app_state.list_state.select_next();
            }
            _ => {}
        },

        _ => {}
    }

    false
}

// function for rendering
fn render(frame: &mut Frame, app_state: &mut AppState) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());
    if app_state.is_add_new {
        let main_instructions = Line::from(vec![
            " Submit ".into(),
            "<Enter>".blue().bold(),
            " Quit ".into(),
            "<Esc>".blue().bold(),
        ])
        .centered();
        Paragraph::new(app_state.input_string.as_str())
            .block(
                Block::bordered()
                    .title("Input description".to_span().into_centered_line())
                    .title_bottom(main_instructions)
                    .fg(Color::Yellow)
                    .padding(Padding::uniform(1)),
            )
            .render(border_area, frame.buffer_mut())
    } else {
        let [inner_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(border_area);

        let main_bindings = Line::from(vec![
            " Move selection up ".into(),
            "<J>".blue().bold(),
            " Move selection down ".into(),
            "<K>".blue().bold(),
            " Add a task ".into(),
            "<C>".blue().bold(),
            " Mark task ".into(),
            "<Enter> ".blue().bold(),
            " Remove task ".into(),
            "<Backspace> ".blue().bold(),
            " Quit ".into(),
            "<Esc> ".blue().bold(),
        ])
        .centered();

        Block::bordered()
            .border_type(BorderType::Rounded)
            .fg(Color::LightGreen)
            .title("rtask tui".to_span().into_centered_line())
            .title_bottom(main_bindings)
            .render(border_area, frame.buffer_mut());

        let list = List::new(app_state.items.iter().map(|task| {
            let value = if task.done {
                task.title.to_span().crossed_out()
            } else {
                task.title.to_span()
            };
            ListItem::from(value)
        }))
        .fg(Color::Blue)
        .highlight_style(Style::default().fg(Color::Green))
        .highlight_symbol(">");

        frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);
    }
}
