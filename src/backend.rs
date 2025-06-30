use crate::{
    colors::{error_msg, success_msg, warning_msg},
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
    sync::mpsc::TryIter,
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
            eprintln!("{}", "Failed to create rtask config dir".fg::<Red>());
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

    pub fn add_task(&mut self, title: &String) -> Result<(), AppError> {
        if title == "" {
            return Err(AppError::EmptyInput);
        }
        // ! check for an existing task
        let task = Task::new(title);

        self.items.push(task);
        Ok(())
    }

    pub fn print_tasks(&self) {
        for (index, task) in self.items.iter().enumerate() {
            println!(
                "№{} name: {}, state: {}, priority: {}",
                index + 1,
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
                if title.join(" ") == "" {
                    return Err(AppError::EmptyInput);
                }
                let items = &mut self.items;
                let mut found = false;

                for t in items.iter_mut() {
                    if t.title == title.join(" ") {
                        t.priority = prior.clone();
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
                return Err(AppError::TooBigIndex(index, items.len()));
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

    // TODO: this
    pub fn remove_task(&mut self, opt: &TaskOption) -> Result<(), AppError> {
        let items = &mut self.items;
        match opt {
            TaskOption::Id(i) => {
                if *i == 0  || *i > items.len() {
                    return Err(AppError::TooBigIndex(*i, items.len()));
                }
                items.remove(*i - 1);
            }
            TaskOption::Title(title) => {
                if title.join(" ") == "" {
                    return Err(AppError::EmptyInput);
                }
                items.retain(|t| t.title != title.join(" "));
            }
        }

        Ok(())
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
