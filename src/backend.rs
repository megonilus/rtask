use crate::{
    colors::{error_msg, success_msg, warning_msg},
    task::{Priority, Task},
    task_option::TaskOption,
};
use anyhow::{Context, Ok, Result};
use dirs::config_dir;
use owo_colors::{OwoColorize, colors::*};
use serde_json::Result as SerdeResult;
use std::{
    cell::RefCell,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    rc::Rc,
};

#[derive(Debug)]
pub struct Backend {
    pub path: PathBuf,
    pub items: Rc<RefCell<Vec<Task>>>,
}

impl Backend {
    pub fn new(path: &str, items: Rc<RefCell<Vec<Task>>>) -> Result<Self> {
        let mut base_path = config_dir().unwrap_or_else(|| PathBuf::from("."));
        base_path.push("rtask");
        fs::create_dir_all(&base_path).context("Failed to create rtask config dir".fg::<Red>())?;

        base_path.push(path);

        if !base_path.exists() {
            let _ = File::create(&base_path)?.write_all(b"[]");
        }
        Ok(Self {
            path: base_path,
            items,
        })
    }

    pub fn add_task(&self, title: &String) -> Result<&str> {
        if *title == *"" {
            return Ok("Empty input!");
        }
        // ! check for an existing task
        let task = Task::new(title);

        self.items.borrow_mut().push(task);
        Ok("Added!")
    }

    pub fn print_tasks(&self) {
        for task in self.items.borrow().iter() {
            println!(
                "name: {}, state: {}, priority: {}",
                task.title,
                if task.done {
                    success_msg("Done")
                } else {
                    warning_msg("In progress")
                },
                Priority::visualise(&task.priority)
            );
        }
    }
    pub fn mark_task(&mut self, opt: TaskOption) -> Option<String> {
        let mut items = self.items.borrow_mut();

        match opt {
            TaskOption::Id(id) => {
                if id == 0 || id > items.len() {
                    return Some(error_msg("Wrong index!"));
                }
                if let Some(task) = items.get_mut(id - 1) {
                    task.done = !task.done;
                    Some(success_msg("Marked!"))
                } else {
                    Some(warning_msg("Not marked!"))
                }
            }
            TaskOption::Title(title) => {
                let mut found = false;
                for t in items.iter_mut() {
                    if t.title == *title {
                        t.done = !t.done;
                        found = true
                    }
                }
                Some(if found {
                    success_msg("Marked!")
                } else {
                    warning_msg("Not found with such title!")
                })
            }
        }
    }

    pub fn edit_priority(&self, opt: TaskOption, prior: Priority) -> &str {
        match opt {
            TaskOption::Id(id) => {
                let mut items = self.items.borrow_mut();
                if id == 0 || id > items.len() {
                    return "Wrong id!";
                }
                if let Some(task) = items.get_mut(id - 1) {
                    task.priority = prior;
                    "Edited!"
                } else {
                    "Not found!!"
                }
            }
            TaskOption::Title(title) => {
                if title == *"" {
                    return "Empty input!";
                }
                let mut items = self.items.borrow_mut();
                let mut found = false;

                for t in items.iter_mut() {
                    if t.title == title {
                        t.priority = prior.clone();
                        found = true
                    }
                }
                if found { "Edited!" } else { "Not found!" }
            }
        }
    }

    pub fn substract_priority(&mut self, opt: TaskOption, increase: bool) {
        let mut items = self.items.borrow_mut();
        match opt {
            TaskOption::Id(index) => {
                if index != 0 && index <= items.len() {
                    if let Some(task) = items.get_mut(index - 1) {
                        task.priority = if increase {
                            task.priority.increase()
                        } else {
                            task.priority.decrease()
                        }
                    }
                }
            }
            TaskOption::Title(title) => {
                for t in items.iter_mut() {
                    if t.title == title {
                        t.priority = if increase {
                            t.priority.increase()
                        } else {
                            t.priority.decrease()
                        }
                    }
                }
            }
        }
    }

    // TODO: this
    pub fn remove_task(&self, opt: TaskOption) -> Result<bool> {
        let mut items = self.items.borrow_mut();
        match opt {
            TaskOption::Id(i) => {
                items.swap_remove(i - 1);
            }
            TaskOption::Title(title) => {
                items.retain(|t| t.title != title);
            }
        }

        Ok(true)
    }
    pub fn update(&mut self) -> Result<()> {
        *self.items.borrow_mut() = serde_json::from_str(fs::read_to_string(&self.path)?.as_str())?;
        Ok(())
    }

    pub fn save(&mut self) -> SerdeResult<()> {
        if let Err(e) = fs::write(
            &self.path,
            &serde_json::to_string_pretty(&*self.items.borrow())?,
        ) {
            println!("An error ocurred when tried to save tasks: {e}")
        };

        SerdeResult::Ok(())
    }
}
