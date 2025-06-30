use std::{cell::RefCell, rc::Rc};

use ratatui::widgets::ListState;

use crate::task::Task;

#[derive(Debug, PartialEq, Eq)]
pub enum TuiState {
    Normal,
    Add(String),
}
impl TuiState {
    pub fn get_input_string(&mut self) -> Option<&mut String> {
        if let TuiState::Add(s) = self {
            return Some(s);
        }
        None
    }
}
#[derive(Debug)]
// TODO: global app name variable?
pub struct AppState {
    pub list_state: ListState,
    pub tui_state: TuiState,
    pub showing_help: bool,
}
impl AppState {
    pub fn new() -> Self {
        Self {
            list_state: ListState::default(),
            // is_add_new: false,
            // input_string: String::new(),
            // is_help: false,
            tui_state: TuiState::Normal,
            showing_help: false,
        }
    }
}
