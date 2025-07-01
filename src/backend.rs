use crate::{
    colors::{success_msg, warning_msg},
    error::AppError,
    task::{Priority, Task},
    task_option::TaskOption,
};
use dirs::config_dir;
use owo_colors::{colors::*, OwoColorize};
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

#[derive(Debug)]
pub struct Backend {
    pub path: PathBuf,
    pub items: Vec<Task>,
}

impl Backend {
    pub fn new(path: &str) -> Result<Self, AppError> {
        let mut base_path = config_dir().unwrap_or_else(|| PathBuf::from("."));
        base_path.push("rtask");
        if let Err(e) = fs::create_dir_all(&base_path) {
            eprintln!(
                "error: {e}, {}",
                "Failed to create rtask config dir".fg::<Red>()
            );
        }

        base_path.push(path);

        if !base_path.exists() {
            let _ = File::create(&base_path)?.write_all(b"[]");
        }
        Ok(Self {
            path: base_path,
            items: vec![],
        })
    }

    pub fn add_task(&mut self, title: &str) -> Result<(), AppError> {
        if title.is_empty() {
            return Err(AppError::EmptyInput);
        }

        if Option::is_some(&self.items.iter().find(|t| t.title == title)) {
            return Err(AppError::AlreadyExists(title.to_string()));
        }

        // ! check for an existing task
        let task = Task::new(title);

        self.items.push(task);
        Ok(())
    }

    pub fn print_tasks(&self, sort: bool, done: bool) {
        let mut items = self.items.clone();
        if sort {
            items =Self::sort_tasks(&items,false);
        }

        for (index, task) in items.iter().enumerate() {
            if !done || task.done {
                println!(
                    "№{} name: {}, state: {}, priority: {}",
                    index + 1,
                    task.title,
                    if task.done {
                        success_msg("Done")
                    } else {
                        warning_msg("In progress")
                    },
                    task.priority.visualise()
                )
            }
        }
    }
    pub fn mark_task(&mut self, opt: TaskOption) -> Result<(), AppError> {
        let items = &mut self.items;

        match opt {
            TaskOption::Id(id) => {
                if id == 0 || id > items.len() {
                    return Err(AppError::TooBigIndex(id, items.len()));
                }
                if let Some(task) = items.get_mut(id - 1) {
                    task.done = !task.done;
                    return Ok(());
                }
                Err(AppError::TaskNotFound(opt))
            }
            TaskOption::Title(title) => {
                let mut found = false;
                for t in items.iter_mut() {
                    if t.title == title.join(" ") {
                        t.done = !t.done;
                        found = true
                    }
                }
                if found {
                    Ok(())
                } else {
                    Err(AppError::TaskNotFound(TaskOption::Title(title)))
                }
            }
        }
    }

    pub fn edit_priority(&mut self, opt: TaskOption, prior: Priority) -> Result<(), AppError> {
        match opt {
            TaskOption::Id(id) => {
                let items = &mut self.items;
                if id == 0 || id > items.len() {
                    return Err(AppError::TooBigIndex(id, items.len()));
                }
                if let Some(task) = items.get_mut(id - 1) {
                    task.priority = prior;
                    Ok(())
                } else {
                    Err(AppError::TaskNotFound(opt))
                }
            }
            TaskOption::Title(title) => {
                if title.join(" ").is_empty() {
                    return Err(AppError::EmptyInput);
                }
                let items = &mut self.items;
                let mut found = false;

                for t in items.iter_mut() {
                    if t.title == title.join(" ") {
                        t.priority = prior;
                        found = true
                    }
                }
                if found {
                    Ok(())
                } else {
                    Err(AppError::TaskNotFound(TaskOption::Title(title)))
                }
            }
        }
    }

    pub fn substract_priority(&mut self, opt: TaskOption, increase: bool) -> Result<(), AppError> {
        let items = &mut self.items;
        match opt {
            TaskOption::Id(index) => {
                if index != 0 || index < items.len() {
                    if let Some(task) = items.get_mut(index) {
                        task.priority = if increase {
                            task.priority.increase()
                        } else {
                            task.priority.decrease()
                        };
                        return Ok(());
                    }
                    return Err(AppError::TaskNotFound(TaskOption::Id(index)));
                }
                Err(AppError::TooBigIndex(index, items.len()))
            }
            TaskOption::Title(title) => {
                for t in items.iter_mut() {
                    if t.title == title.join(" ") {
                        t.priority = if increase {
                            t.priority.increase()
                        } else {
                            t.priority.decrease()
                        }
                    }
                }
                Ok(())
            }
        }
    }

    pub fn remove_task(&mut self, opt: &TaskOption) -> Result<(), AppError> {
        match opt {
            TaskOption::Id(i) => {
                if *i == 0 || *i > self.items.len() {
                    return Err(AppError::TooBigIndex(*i, self.items.len()));
                }
                self.items.remove(*i - 1);
            }
            TaskOption::Title(title) => {
                if title.join(" ").is_empty() {
                    return Err(AppError::EmptyInput);
                }
                let old_len = self.items.len();
                self.items.retain(|t| t.title != title.join(" "));
                if self.items.len() == old_len {
                    return Err(AppError::TaskNotFound(TaskOption::Title(title.clone())));
                }
            }
        }
        Ok(())
    }

    pub fn sort(&mut self, reversed: bool) -> Result<(), AppError> {
        self.items = Self::sort_tasks(&self.items, reversed);
        Ok(())
    }

    pub fn sort_tasks(items: &Vec<Task>, reversed: bool) -> Vec<Task> {
        let mut sorted = items.to_owned();
        sorted.sort_by_key(|t| {if reversed{return Priority::Low as usize - t.priority as usize} t.priority as usize});
        
        sorted
    }

    pub fn update(&mut self) -> Result<(), AppError> {
        self.items = serde_json::from_str(fs::read_to_string(&self.path)?.as_str())?;
        Ok(())
    }

    pub fn save(&mut self) -> Result<(), AppError> {
        if let Err(e) = fs::write(&self.path, &serde_json::to_string_pretty(&*self.items)?) {
            println!("An AppError ocurred when tried to save tasks: {e}")
        };

        Ok(())
    }
}
