use clap::Parser;

use anyhow::{Context, Result};
use crate::app_state::AppState;
use crate::cli::{Args, Commands};
use console::style;


pub fn commander(app: &AppState) -> Result<()>{
    let cli = Args::parse();

    match &cli.command {
        Some(Commands::Add { title }) => {
            app.db
                .add_task(title.join(" "))
                .expect("Something went wrong when tried to add a new task");
        }
    
        Some(Commands::Remove { title }) => {
            app.db
                .remove_task(title.join(" ")).context(style("error: something went wrong!").red())?;
        }
        Some(Commands::List) => {
            app.db
                .list_tasks()?;
        }
        Some(Commands::Mark { title, remove }) => {
             app.db.mark_task(title.join(" ")).context(style("error: something went wrong!").red())?;
            match *remove{
                true => {                  
                                app.db.remove_task(title.join(" ")).context(style("error: something went wrong!").red())?;
                            },
                _ => {}
            }   
        }
        None => {

            println!("{}", style("Wrong command, exiting").yellow());
        }
    }

    
    Ok(())
}