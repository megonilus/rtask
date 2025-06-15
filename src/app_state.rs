use anyhow::{Ok, Result};
use ratatui::widgets::ListState;

use crate::{db::Db, task::Task};

#[derive(Debug)]
// TODO: global app name variable?
pub struct AppState {
    pub db: Db,
    pub items: Vec<Task>,
    pub list_state: ListState,
    pub is_add_new: bool,
    pub input_string: String,
    // ! TODO: pub app_name: String
}

impl AppState{
    pub fn new(title: &str) -> Self{
        let mut app_state = AppState {
            db: Db::new(title).expect("Failed to init DB"),
            items: vec![],
            list_state: ListState::default(),
            is_add_new: false,
            input_string: String::new(),
        };
        // TODO: fix this
        let _ = app_state.update();
        app_state
    }
    pub fn update(&mut self) -> Result<()>{
        self.items = self.db.get_tasks()?;
        Ok(())
    }
}

impl Default for AppState {
     fn default() -> Self {
        AppState {
            db: Db::new("rtask.db").expect("Failed to init DB"),
            items: vec![],
            list_state: ListState::default(),
            is_add_new: false,
            input_string: String::new(),
        }
    }
}