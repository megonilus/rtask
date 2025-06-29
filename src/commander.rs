use crate::app_state::AppState;
use crate::backend::Backend;
use crate::cli::{Args, Commands};
use crate::colors::{error_msg, success_msg, warning_msg};
use crate::task::Priority;
use crate::task_option::TaskOption;
use crate::tui::init;
use anyhow::{Context, Result};
use clap::Parser;

pub fn commander(app: &mut AppState, backend: &mut Backend) -> Result<()> {
    let cli = Args::parse();

    match &cli.command {
        Some(Commands::Add { title }) => {
            backend
                .add_task(&title.join(" "))
                .expect("Something went wrong when tried to add a new task");
        }

        Some(Commands::Remove { title }) => {
            if backend
                .remove_task(TaskOption::Title(title.join(" ")))
                .context(error_msg("error: something went wrong!"))?
            {
                println!("{}", success_msg("Removed!"))
            } else {
                println!("{}", warning_msg("task not found"))
            }
        }
        Some(Commands::List) => {
            backend.print_tasks();
        }
        Some(Commands::Mark { title, remove }) => {
            println!(
                "{}",
                backend
                    .mark_task(TaskOption::Title(title.join(" ")))
                    .context(error_msg("error: something went wrong!"))?
            );
            if *remove {
                backend
                    .remove_task(TaskOption::Title(title.join(" ")))
                    .context(error_msg("Error: something went wrong!"))?;
            }
        }
        Some(Commands::Tui) => {
            init(app, backend).expect("Failed to init tui!");
        }
        Some(Commands::Priority { title, priority }) => {
            let p = Priority::from_str(priority);

            match p {
                Ok(priority) => {
                    println!(
                        "{}",
                        backend.edit_priority(TaskOption::Title(title.join(" ")), priority)
                    )
                }
                Err(e) => {
                    println!("{}", error_msg(e.as_str()))
                }
            };
        }
        None => {
            println!("{}", warning_msg("Wrong command, exiting"));
        }
    }

    Ok(())
}
