use anyhow::Result;
use db::Db;
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
// TODO: split monolith file into multiple monolith files :D
// TODO: colored output
// TODO: think about this struct.....
// TODO: remove unused dependecies
// TODO: nerd font icons
// TODO: install.sh and install.bat(or other)
// TODO: utils.rs for utililes functions

fn main() -> Result<()> {
    let app = AppState {
        db: Db::new("tasks.db")?,
    };
    commander(&app)?;
    Ok(())
}