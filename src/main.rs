use rusqlite::Result;

mod task; // * task struct
mod commander;
mod app_state;
use app_state::AppState;
mod db;
mod cli; 

use db::Db;
use commander::commander;

// TODO: add  features like --verbose
// TODO: add more features and meaning to the project
// TODO: fix all cargo warnings
// TODO: split monolith file into multiple monolith files :D
// TODO: colored output
// TODO: think about this struct.....
// TODO: remove unused dependecies
// TODO: nerd font icons
// TODO: install.sh and install.bat(or other)

fn main() -> Result<()> {
    let app = AppState {
        db: Db::new("tasks.db")?,
    };

    commander(&app);

    Ok(())
}
