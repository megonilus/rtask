use clap::{Parser, Subcommand};

use crate::task_option::TaskOption;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]

// TODO: list --done
// TODO: list --search
// TODO: deadlines(optional for task creating)
// TODO: sorting by priority
pub enum Commands {
    /// Adds to tasklist a todo with a name
    Add {
        title: Vec<String>,
    },
    /// Remove from tasklist a task by id or title
    Remove {
        option: TaskOption,
    },
    /// mark task as done(not deleting it) by id or title
    Mark {
        option: TaskOption,
        #[arg(short, long)]
        remove: bool,
    },
    /// change priority of task by id or title
    Priority {
        priority: String,
        option: TaskOption,
    },
    /// print tasklist
    List,
    /// enter tui mode instead of cli
    Tui,
}
