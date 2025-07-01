use serde::{Deserialize, Serialize};

use crate::{
    colors::{error_msg, success_msg, warning_msg},
    error::AppError,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub title: String,
    pub done: bool,
    pub priority: Priority,
}

impl Task {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            done: false,
            priority: Priority::Normal,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Copy)]
pub enum Priority {
    High = 0,
    Normal = 1,
    Low = 2,
}


impl Priority {
    pub fn from_str(s: &str) -> Result<Self, AppError> {
        match s.to_lowercase().as_str() {
            "low" => Ok(Priority::Low),
            "normal" => Ok(Priority::Normal),
            "high" => Ok(Priority::High),
            _ => Err(AppError::InvalidPriority(format!(
                "Wrong format! Should be Low/Normal/High(case-insensitive), entered {s}"
            ))),
        }
    }

    pub fn to_str(self) -> &'static str {
        match &self {
            Priority::Low => "Low",
            Priority::Normal => "Normal",
            Priority::High => "High",
        }
    }

    pub fn visualise(&self) -> String {
        match self.to_str() {
            "Low" => success_msg("Low"),
            "Normal" => warning_msg("Normal"),
            "High" => error_msg("High"),
            _ => "".to_string(),
        }
    }

    pub fn decrease(&self) -> Self {
        Priority::from((*self as usize).saturating_add(1))
        
    }

    pub fn increase(&self) -> Self {
        Priority::from((*self as usize).saturating_sub(1))
    }
}

impl From<usize> for Priority{
    fn from(value: usize) -> Self {
        match value {
            0 => Priority::High,
            1 => Priority::Normal,
            2 => Priority::Low,
            _ => Priority::Normal
        }
    }
}
