use clap::{Parser, Subcommand};

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
    // Adds to tasklist a todo with a name
    Add {
        title: Vec<String>,
    },
    // Remove from tasklist a task
    Remove {
        title: Vec<String>,
    },
    // mark task as done(not deleting it)
    // syntax: rtask mark title
    Mark {
        title: Vec<String>,
        #[arg(short, long)]
        remove: bool,
    },
    Priority {
        priority: String,
        title: Vec<String>,
    },
    // print tasklist
    List,
    // TODO: flags?
    Tui,
}
