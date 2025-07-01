use clap::{Parser, Subcommand};

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

    /// Remove a task by ID or full title. Example: rtask remove 2 or rtask remove Buy milk
    Remove {
        /// remove all done tasks, if parsed rtask will ignore the option you will provide
        /// e.g. rtask remove -r Buy Milk... will lead to Buy Milk... to be ignored
        #[arg(short, long)]
        done: bool,
        option: Vec<String>,
    },

    /// Mark a task as done or undone by ID or title. Optionally remove it right after.
    Mark {
        option: Vec<String>,
        /// Also remove the task after marking it
        #[arg(short, long)]
        remove: bool,
    },

    /// Change the priority of a task by ID or title. Example: rtask priority high Buy milk
    Priority {
        /// New priority value (e.g. low, normal, high)
        priority: String,
        option: Vec<String>,
    },

    /// Show the current task list
    List {
        /// list only done items
        #[arg(short, long)]
        done: bool,

        /// list sorted items by their priority
        #[arg(short, long)]
        sort: bool,
    },
    /// sort and save
    Sort {
        #[arg(short, long)]
        reverse: bool,
    },

    /// Start the interactive TUI mode
    Tui,
}
