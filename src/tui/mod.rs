use ratatui::{
    crossterm::event::{self, Event, KeyEvent, KeyEventKind},
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, ToSpan},
    widgets::{Block, BorderType, List, ListItem, Padding, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use crate::{
    app_state::{AppState, TuiState},
    backend::Backend,
    error::AppError,
    task_option::TaskOption,
};

enum FormAction {
    None,
    Submit,
    Escape,
}

pub fn init(state: &mut AppState, backend: &mut Backend) -> Result<(), AppError> {
    let _ = backend.update();

    color_eyre::install()?;

    let terminal = ratatui::init();

    let result = run(terminal, state, backend);

    ratatui::restore();
    result
}

fn run(
    mut terminal: DefaultTerminal,
    app_state: &mut AppState,
    backend: &mut Backend,
) -> Result<(), AppError> {
    // loop where we are rendering and handling the input
    loop {
        // Rendering
        terminal.draw(|f| render(f, app_state, backend))?;

        // Input handling
        if let Event::Key(key) = event::read()? {
            match app_state.tui_state {
                TuiState::Add(_) => match handle_new_todo(key, app_state) {
                    FormAction::None => {}
                    FormAction::Submit => {
                        if let Some(input_string) = app_state.tui_state.get_input_string() {
                            let _ = backend.add_task(input_string);
                            let _ = backend.save();

                            input_string.clear();
                        }
                        app_state.tui_state = TuiState::Normal;
                    }
                    FormAction::Escape => {
                        app_state.tui_state = TuiState::Normal;
                    }
                },
                _ => {
                    if handle_key(key, app_state, backend)? {
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}

fn handle_new_todo(key: KeyEvent, app_state: &mut AppState) -> FormAction {
    match key.code {
        event::KeyCode::Char(char) => {
            if let Some(input_string) = app_state.tui_state.get_input_string() {
                input_string.push(char);
            }
        }
        event::KeyCode::Backspace => {
            if let Some(input_string) = app_state.tui_state.get_input_string() {
                input_string.pop();
            }
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

fn handle_key(
    key: KeyEvent,
    app_state: &mut AppState,
    backend: &mut Backend,
) -> Result<bool, AppError> {
    if key.kind == KeyEventKind::Press {
        match key.code {
            event::KeyCode::Esc => {
                // * exiting the program with esc keybinding
                if app_state.showing_help {
                    app_state.showing_help = false;
                    return Ok(false);
                }
                return Ok(false);
            }
            event::KeyCode::Enter => {
                if let Some(index) = app_state.list_state.selected() {
                    backend.mark_task(TaskOption::Id(index + 1))?;
                }
            }
            event::KeyCode::Backspace => {
                if let Some(index) = app_state.list_state.selected() {
                    backend.remove_task(&TaskOption::Id(index + 1))?;
                }
            }
            event::KeyCode::Char(char) => match char {
                'c' => {
                    app_state.tui_state = TuiState::Add("".to_string());
                }
                'k' => {
                    app_state.list_state.select_previous();
                }
                'j' => {
                    app_state.list_state.select_next();
                }
                'h' => {
                    app_state.showing_help = !app_state.showing_help;
                }
                'e' => {
                    if let Some(i) = app_state.list_state.selected() {
                        backend.substract_priority(TaskOption::Id(i), true)?;
                        let _ = backend.save();
                    }
                }
                'r' => {
                    if let Some(i) = app_state.list_state.selected() {
                        backend.substract_priority(TaskOption::Id(i), false)?;
                        let _ = backend.save();
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
    Ok(false)
}

// function for rendering
fn render(frame: &mut Frame, app_state: &mut AppState, backend: &Backend) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());
    let items = &backend.items;
    match &app_state.tui_state {
        TuiState::Normal => {
            if app_state.showing_help {
                let help_box_area =
                    center(border_area, Constraint::Length(60), Constraint::Length(12));

                let help_text = Paragraph::new(vec![
                    Line::from(vec!["j / k".cyan().bold(), " - Move up/down".into()]),
                    Line::from(vec!["c".cyan().bold(), " - Create task".into()]),
                    Line::from(vec!["Enter".cyan().bold(), " - Toggle done".into()]),
                    Line::from(vec!["e / r".cyan().bold(), " - Priority +/-".into()]),
                    Line::from(vec!["Backspace".cyan().bold(), " - Remove task".into()]),
                    Line::from(vec!["Esc".cyan().bold(), " - Exit / Hide help".into()]),
                    Line::from(vec!["h".cyan().bold(), " - Show / Hide help".into()]),
                ])
                .block(
                    Block::bordered()
                        .title("Help".to_span().into_centered_line())
                        .border_type(BorderType::Rounded)
                        .fg(Color::Cyan)
                        .padding(Padding::uniform(1)),
                )
                .alignment(Alignment::Left);

                help_text.render(help_box_area, frame.buffer_mut());
            }

            let [inner_area] = Layout::vertical([Constraint::Fill(1)])
                .margin(1)
                .areas(border_area);
            Block::bordered()
                .border_type(BorderType::Rounded)
                .fg(Color::LightGreen)
                .title("rtask tui".to_span().into_centered_line())
                .title_bottom("Help - <h> ".to_span().into_centered_line())
                .render(border_area, frame.buffer_mut());

            let list = List::new(items.iter().map(|task| {
                let value = Line::from(vec![
                    if task.done {
                        task.title.to_span().crossed_out()
                    } else {
                        task.title.to_span()
                    },
                    " | ".into(),
                    task.priority.to_str().into(),
                ]);

                ListItem::from(value)
            }))
            .fg(Color::Blue)
            .highlight_style(Style::default().fg(Color::Green))
            .highlight_symbol(">");

            frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);
        }
        TuiState::Add(input_string) => {
            let add_binds = Line::from(vec![
                " Submit ".into(),
                "<Enter>".blue().bold(),
                " Quit ".into(),
                "<Esc>".blue().bold(),
            ])
            .centered();
            Paragraph::new(input_string.as_str())
                .block(
                    Block::bordered()
                        .title("Input description".to_span().into_centered_line())
                        .title_bottom(add_binds)
                        .fg(Color::Yellow)
                        .padding(Padding::uniform(1)),
                )
                .render(border_area, frame.buffer_mut())
        }
    }
}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
