use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use app_state::AppState;
use commander::commander;

use crate::{backend::Backend, task::Task};

mod app_state;
mod backend;
mod cli;
mod colors;
mod commander;
mod task;
mod task_option;
mod tui;
// TODO: add  features like --verbose
// TODO: add more features and meaning to the project
// TODO: fix all cargo warnings
// TODO: remove unused dependecies
// TODO: nerd font icons
// TODO: install.sh and install.bat(or other)
// TODO: utils.rs for utililes functions
// TODO: let  the user decide for db name and choose them in tui mode and cli mode
// TODO: separate file for handling output(handler.rs)

fn main() -> Result<()> {
    let shared_tasks: Rc<RefCell<Vec<Task>>> = Rc::new(RefCell::new(vec![]));

    let mut backend = Backend::new("tasks.json", Rc::clone(&shared_tasks))?;

    backend.update()?;

    let mut app = AppState::new(shared_tasks);

    commander(&mut app, &mut backend)?;

    let _ = backend.save();
    Ok(())
}
