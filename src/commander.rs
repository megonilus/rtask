use crate::app_state::AppState;
use crate::backend::Backend;
use crate::cli::{Args, Commands};
use crate::colors::{error_msg, success_msg, warning_msg};
use crate::error::AppError;
use crate::task::Priority;
use crate::tui::init;
use clap::Parser;
use std::result::Result::Ok;

pub fn commander(app: &mut AppState, backend: &mut Backend) -> Result<(), AppError> {
    let cli = Args::parse();
    let mut is_need_to_print = false;

    match cli.command {
        Some(Commands::Add { title }) => {
            backend.add_task(&title.join(" "))?;
            is_need_to_print = true;
        }

        Some(Commands::Remove { option }) => {
            backend.remove_task(&option)?;
            is_need_to_print = true;
        }
        Some(Commands::List) => {
            backend.print_tasks();
        }
        Some(Commands::Mark { option, remove }) => {
            if remove {
                backend.remove_task(&option)?;
                return Ok(());
            }
            backend.mark_task(option)?;
            is_need_to_print = true;
        }
        Some(Commands::Tui) => {
            init(app, backend).expect("Failed to init tui!");
        }
        Some(Commands::Priority { option, priority }) => {
            let p = Priority::from_str(&priority);

            match p {
                Ok(priority) => {
                    backend.edit_priority(option, priority)?;
                }
                Err(e) => {
                    eprintln!("Wrong priority! entered: {}", error_msg(e.as_str()));
                }
            };
        }
        None => {
            println!("{}", warning_msg("Wrong command, exiting"));
        }
    }
    if is_need_to_print == true {
        backend.print_tasks();
    }
    Ok(())
}
