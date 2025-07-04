use crate::{backend::Backend, error::AppError};
use app_state::AppState;
use commander::commander;

mod app_state;
mod backend;
mod cli;
mod colors;
mod commander;
mod error;
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

fn main() -> Result<(), AppError> {
    let result: Result<(), AppError> = (|| {
        let mut backend = Backend::new("tasks.json")?;
        backend.update()?;

        let mut app = AppState::new();
        commander(&mut app, &mut backend)?;

        backend.save().ok();

        Ok(())
    })();

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }

    Ok(())
}
