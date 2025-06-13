use rusqlite::{Result, Row};

// TODO: add removing task by id too(not uuid from bd)
pub struct Task {
    pub id: String,
    pub title: String,
    pub done: bool,
}

impl Task {
    pub fn from_row(row: &Row) -> Result<Self> {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            done: row.get(2)?,
        })
    }
}
