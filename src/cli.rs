use clap::{Parser, Subcommand};

use crate::task_option::TaskOption;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a task to the list. Example: rtask add Buy milk
    Add {
        /// Title of the task (multiple words allowed)
        title: Vec<String>,
    },

    /// Remove a task by ID or full title. Example: rtask remove 2 or rtask remove "Buy milk"
    Remove { option: TaskOption },

    /// Mark a task as done or undone by ID or title. Optionally remove it right after.
    Mark {
        option: TaskOption,
        /// Also remove the task after marking it
        #[arg(short, long)]
        remove: bool,
    },

    /// Change the priority of a task by ID or title. Example: rtask priority high "Buy milk"
    Priority {
        /// New priority value (e.g. low, normal, high)
        priority: String,
        option: TaskOption,
    },

    /// Show the current task list
    List,

    /// Start the interactive TUI mode
    Tui,
}
