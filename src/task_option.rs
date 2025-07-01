use std::{fmt::{Display, Formatter}, str::FromStr};

use strum::{EnumIter, EnumString};

use crate::{error::AppError, task::Priority};

#[derive(Debug, Clone, EnumIter, EnumString)]
pub enum TaskOption {
    Id(usize),
    Title(Vec<String>),
}

impl TryFrom<Vec<String>> for TaskOption{
    type Error = AppError;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
         if let Some(com) = value.first(){
            if Priority::from_str(com).is_ok(){
                return Err(AppError::TaskOptionException("".to_string()))
            }

            return Ok(TaskOption::from(value.join(" ")))
        } 
        Err(AppError::EmptyInput)
    }
}
impl From<String> for TaskOption {
    fn from(input: String) -> Self {
        if let Ok(id) = input.parse::<usize>() {
            TaskOption::Id(id)
        } else {
            TaskOption::Title(vec![input])
        }
    }
}

impl From<&String> for TaskOption {
    fn from(input: &String) -> Self {
        TaskOption::from(input.clone())
    }
}

impl Display for TaskOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskOption::Id(id) => write!(f, "{id}",),
            TaskOption::Title(title) => write!(f, "\"{}\"", title.join(" ")),
        }
    }
}
