use std::{io, };
use thiserror::Error;

use crate::task_option::TaskOption;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),

    #[error("Json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("NotFound error: task with title {0} not found ")]
    TaskNotFound(TaskOption),

    #[error("Empty input")]
    EmptyInput,

    #[error("TooBigIndex error: entered {0}, when max is: {1}")]
    TooBigIndex(usize, usize),

    #[error("Invalid priority: {0}")]
    InvalidPriority(String),

    #[error("Task with title: {0} already exists")]
    AlreadyExists(String),

    #[error("Error: {0}")]
    ColorEyreError(#[from] color_eyre::eyre::Error),
}
