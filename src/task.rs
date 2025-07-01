use serde::{Deserialize, Serialize};
use strum::VariantNames;
use strum_macros::{Display, EnumIter, EnumString};

use crate::colors::style_msg;

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

#[derive(
    EnumString,
    Display,
    EnumIter,
    VariantNames,
    Serialize,
    Deserialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Copy,
)]
#[strum(serialize_all = "PascalCase", ascii_case_insensitive)]
pub enum Priority {
    #[strum(serialize = "high")]
    High,
    #[strum(serialize = "normal")]
    Normal,
    #[strum(serialize = "low")]
    Low,
}

impl Priority {
    pub fn visualise(&self) -> String {
        match self {
            Priority::Low => style_msg(
                "Low",
                (91, 152, 70),
                owo_colors::DynColors::Ansi(owo_colors::AnsiColors::BrightGreen),
            ),
            Priority::Normal => style_msg(
                "Normal",
                (244, 199, 42),
                owo_colors::DynColors::Ansi(owo_colors::AnsiColors::BrightYellow),
            ),
            Priority::High => style_msg(
                "High",
                (210, 68, 99),
                owo_colors::DynColors::Ansi(owo_colors::AnsiColors::BrightRed),
            ),
        }
    }

    pub fn decrease(&self) -> Self {
        Priority::from((*self as usize).saturating_add(1))
    }

    pub fn increase(&self) -> Self {
        Priority::from((*self as usize).saturating_sub(1))
    }
}

impl From<usize> for Priority {
    fn from(value: usize) -> Self {
        match value {
            0 => Priority::High,
            1 => Priority::Normal,
            _ => Priority::Low,
        }
    }
}
