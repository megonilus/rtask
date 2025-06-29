pub enum TaskOption {
    Id(usize),
    Title(String),
}

impl From<String> for TaskOption {
    fn from(input: String) -> Self {
        if let Ok(id) = input.parse::<usize>() {
            TaskOption::Id(id.saturating_sub(1))
        } else {
            TaskOption::Title(input)
        }
    }
}

impl From<&String> for TaskOption {
    fn from(input: &String) -> Self {
        TaskOption::from(input.clone())
    }
}
