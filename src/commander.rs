use clap::Parser;
use anyhow::{Context, Result};
use crate::app_state::AppState;
use crate::cli::{Args, Commands};
use crate::tui::init;
use console::style;


pub fn commander(app: &mut AppState) -> Result<()>{
    let cli = Args::parse();

    match &cli.command {
        Some(Commands::Add { title }) => {
            app.db
                .add_task(&title.join(" "))
                .expect("Something went wrong when tried to add a new task");
        }
    
        Some(Commands::Remove { title }) => {
            if app.db.remove_task(&title.join(" ")).context(style("error: something went wrong!").red())?{
                println!("{}", style("Removed!").yellow())
             }
             else {println!("{}", style("task not found").yellow())}
        }
        Some(Commands::List) => {
            app.db
                .list_tasks()?;
        }
        Some(Commands::Mark { title, remove }) => {
             if app.db.mark_task(&title.join(" ")).context(style("error: something went wrong!").red())?{
                println!("{}", style("Marked!").yellow())
             }
             else {println!("{}", style("task not found").yellow())}
            match *remove{
                true => {                  
                                app.db.remove_task(&title.join(" ")).context(style("error: something went wrong!").red())?;
                            },
                _ => {}
            }   
        }
        Some(Commands::Tui) => {
            init(app).expect("failed to init tui!  ");
        }
        None => {

            println!("{}", style("Wrong command, exiting").yellow());
        }

    }

    
    Ok(())
}