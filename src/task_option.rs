use std::fmt::{Display, Formatter};


#[derive(Debug, Clone)]
pub enum TaskOption {
    Id(usize),
    Title(Vec<String>),
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
            TaskOption::Id(id) => write!(f, "id: {}", id),
            TaskOption::Title(title) => write!(f, "title: \"{}\"", title.join(" ")),
        }
    }
}