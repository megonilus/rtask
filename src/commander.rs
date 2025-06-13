use clap::Parser;
use crate::app_state::AppState;
use crate::cli::{Args, Commands};



pub fn commander(app: &AppState) {
    let cli = Args::parse();

    match &cli.command {
        Some(Commands::Add { title }) => {
            app.db
                .add_task(title.join(" "))
                .expect("Something went wrong when tried to add a new task");
        }
        Some(Commands::Remove { title }) => {
            app.db
                .remove_task(title.join(" "))
                .expect("Something went wrong when tried to remove tasks");
        }
        Some(Commands::List) => {
            app.db
                .list_tasks()
                .expect("Something went wrong when tried to list tasks");
        }
        Some(Commands::Mark { title, remove }) => {
             app.db.mark_task(title.join(" ")).expect("failed to mark task with a given title");
            match *remove{
                true => {                  
                                app.db.remove_task(title.join(" ")).expect("failed to remove task with a given title");
                            },
                _ => {}
            }   
        }
        None => {
            println!("Wrong command, exiting");
        }
    }
}