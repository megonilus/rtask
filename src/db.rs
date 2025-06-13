use std::{fs, path::PathBuf};
use crate::task::Task;
use rusqlite::{Connection, OptionalExtension, Result, Statement};
use uuid::Uuid;
use dirs;

pub struct Db {
    pub conn: Connection,
}

impl Db {
    pub fn new(path: &str) -> Result<Self> {
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

    pub fn add_task(&self, title: String) -> Result<()> {

        let existing: Option<String> = self
        .conn
        .query_row(
            "SELECT title FROM tasks WHERE title = ?1",
            [&title],
            |row| row.get(0),
        )
        .optional()?; 

    if let Some(_) = existing {
        println!("Task with name \"{title}\" already exists.");
        return Ok(());
    }

        let id = Uuid::new_v4().to_string();
        self.conn.execute(
            "INSERT INTO tasks (id, title, done) VALUES (?1, ?2, 0)",
            &[&id, &title],
        )?;
        Ok(())
    }
    pub fn list_tasks(&self) -> Result<()> {
        let mut prep = self.prep_query("SELECT id, title, done FROM tasks")?;

        let tasks: Vec<Task> = prep
            .query_map([], Task::from_row)?
            .collect::<Result<Vec<_>, _>>()?;

        self.print_tasks(tasks);

        Ok(())
    }
    pub fn print_tasks(&self, tasks: Vec<Task>) {
        for task in tasks {
            println!(
                "name: {}, state: {}",
                task.title,
                if task.done { "done" } else { "in progress" }
            );
        }
    }

  pub  fn mark_task(&self, title: String) -> Result<()> {


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

    pub fn remove_task(&self, title: String) -> Result<()> {
        let mut prep = self.prep_query("DELETE FROM tasks WHERE title=(?1)")?;
        let deleted = prep.execute(&[&title])?;

        match deleted {
            0 => println!("Not found a task with such a title"),
            _ => println!("Removed"),
        }

        Ok(())
    }

    // TODO: for what thus fn exist?
    pub fn prep_query(&self, query: &str) -> Result<Statement<'_>> {
        let prep = self.conn.prepare(query)?;
        Ok(prep)
    }
}