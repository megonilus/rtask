use owo_colors::OwoColorize;

pub fn success_msg(msg: &str) -> String {
    msg.truecolor(91, 152, 70).to_string()
}

pub fn error_msg(msg: &str) -> String {
    msg.truecolor(210, 68, 99).to_string()
}

pub fn warning_msg(msg: &str) -> String {
    msg.truecolor(244, 199, 42).to_string()
}
