use owo_colors::OwoColorize;

pub fn success_msg(msg: &str) -> String {
    msg.green().to_string()
}

pub fn error_msg(msg: &str) -> String {
    msg.red().to_string()
}

pub fn warning_msg(msg: &str) -> String {
    msg.yellow().to_string()
}
