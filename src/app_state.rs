use ratatui::widgets::ListState;

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
pub struct AppState {
    pub list_state: ListState,
    pub tui_state: TuiState,
    pub showing_help: bool,
}
impl AppState {
    pub fn new() -> Self {
        Self {
            list_state: ListState::default(),
            tui_state: TuiState::Normal,
            showing_help: false,
        }
    }
}
