use anyhow::Result;
use app_state::AppState;
use commander::commander;

mod task; 
mod commander;
mod app_state;
mod db;
mod cli; 
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
    let mut app = AppState::new("tasks.db");


    commander(&mut app)?;
    Ok(())
}