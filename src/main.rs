use clap::{Parser, Subcommand};
use dirs;
use rusqlite::{Connection, Result, Row, Statement};
use std::{
    fs,
    path::{Path, PathBuf},
};
use uuid::Uuid; // for the db save path

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Subcommand, Debug)]
enum Commands {
    // Adds to tasklist a todo with a name
    Add { title: Vec<String> },
    // Remove from tasklist a task
    Remove { title: Vec<String> },
    // mark task as done(not deleting it)
    // syntax: rtask mark title
    Mark { 
        title: Vec<String>,
        #[arg(short, long)]
        remove: bool
     },
    // print tasklist
    List,
}

enum TaskState {
    Done(String),
    InProgress(String),
}
#[derive(Debug)]
struct Task {
    id: String,
    title: String,
    done: bool,
}

impl Task {
    fn from_row(row: &Row) -> Result<Self> {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            done: row.get(2)?,
        })
    }
}

struct Db {
    conn: Connection,
}

impl Db {
    fn new(path: &str) -> Result<Self> {
        let mut base_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        base_path.push("rtask");

        fs::create_dir_all(&base_path).expect("Failed to create rtask directory");

        base_path.push(path);
        let conn = Connection::open(base_path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                done BOOLEAN NOT NULL DEFAULT 0
            )",
            (),
        )
        .expect("SQL gone wrong...");
        Ok(Self { conn })
    }

    fn add_task(&self, title: String) -> Result<()> {
        //  TODO: make a check for an existing task
        let id = Uuid::new_v4().to_string();
        self.conn.execute(
            "INSERT INTO tasks (id, title, done) VALUES (?1, ?2, 0)",
            &[&id, &title],
        )?;
        Ok(())
    }
    fn list_tasks(&self) -> Result<()> {
        let mut prep = self.prep_query("SELECT id, title, done FROM tasks")?;

        let tasks: Vec<Task> = prep
            .query_map([], Task::from_row)?
            .collect::<Result<Vec<_>, _>>()?;

        self.print_tasks(tasks);

        Ok(())
    }
    fn print_tasks(&self, tasks: Vec<Task>) {
        for task in tasks {
            println!(
                "name: {}, state: {}",
                task.title,
                if task.done { "done" } else { "in progress" }
            );
        }
    }

    fn mark_task(&self, title: String) -> Result<()> {


        let mut stmt = self.prep_query("SELECT * FROM tasks WHERE title= ?1")?;
        let mut rows = stmt.query_map(&[&title], Task::from_row)?;
        

        let done = match rows.next() {
            Some(Ok(task)) => {
                if !task.done {
                    1
                } else {
                    0
                }
            }
            Some(Err(err)) => return Err(err.into()),
            None => {
                println!("task not found");
                return Ok(());
            }
        };

        let mut prep = self.prep_query("UPDATE  tasks SET done=?2 WHERE title=?1 ")?;
        let marked = prep.execute(rusqlite::params![&title, &done])?;

        match marked {
            0 => println!("Not found a task with such a title"),
            _ => println!("Marked"),
        }

        Ok(())
    }

    fn remove_task(&self, title: String) -> Result<()> {
        let mut prep = self.prep_query("DELETE FROM tasks WHERE title=(?1)")?;
        let deleted = prep.execute(&[&title])?;

        match deleted {
            0 => println!("Not found a task with such a title"),
            _ => println!("Removed"),
        }

        Ok(())
    }

    fn prep_query(&self, query: &str) -> Result<Statement<'_>> {
        let prep = self.conn.prepare(query)?;
        Ok(prep)
    }
}


// TODO: add  features like --verbose
// TODO: add feature rtask mark --remove/-r "title" to mark & remove task
// TODO: add removing task by id too(not uuid from bd)
// TODO: add more features and meaning to the project
// TODO: fix all cargo warnings
// TODO: split monolith file into multiple monolith files :D


//TODO: 
// TODO: think about this struct.....
struct AppState {
    db: Db,
}

fn commander(app: &AppState) {
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

fn main() -> Result<()> {
    let app = AppState {
        db: Db::new("tasks.db")?,
    };

    commander(&app);

    Ok(())
}
