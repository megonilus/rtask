use serde::{Deserialize, Serialize};

use crate::colors::{error_msg, success_msg, warning_msg};

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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Priority {
    Low,
    Normal,
    High,
}

impl Priority {
    const ORDER: [Priority; 3] = [Priority::Low, Priority::Normal, Priority::High];

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "low" => Ok(Priority::Low),
            "normal" => Ok(Priority::Normal),
            "high" => Ok(Priority::High),
            _ => Err(format!(
                "Wrong format! Should be Low/Normal/High(case-insensitive), entered {s}"
            )),
        }
    }

    pub fn to_str(&self) -> &str {
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
            "High" => error_msg("HIgh"),
            _ => "".to_string(),
        }
    }

    pub fn decrease(&self) -> Self {
        let index = Self::ORDER.iter().position(|p| p == self).unwrap_or(1);
        Self::ORDER.get(index.saturating_sub(1)).unwrap().clone()
    }

    pub fn increase(&self) -> Self {
        let index = Self::ORDER.iter().position(|p| p == self).unwrap_or(1);
        Self::ORDER
            .get((index + 1).min(Self::ORDER.len() - 1))
            .unwrap()
            .clone()
    }
}
