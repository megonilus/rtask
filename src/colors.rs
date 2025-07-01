use owo_colors::{DynColors, OwoColorize};

fn supports_truecolor() -> bool {
    match std::env::var("COLORTERM") {
        Ok(val) => val.to_lowercase().contains("truecolor") || val.to_lowercase().contains("24bit"),
        Err(_) => false,
    }
}

pub fn success_msg(msg: &str) -> String {
    msg.truecolor(91, 152, 70).to_string()
}

// pub fn error_msg(msg: &str) -> String {
//     msg.truecolor(210, 68, 99).to_string()
// }

pub fn warning_msg(msg: &str) -> String {
    msg.truecolor(244, 199, 42).to_string()
}

pub fn style_msg(msg: &str, rgb: (u8, u8, u8), fallback: DynColors) -> String {
    if supports_truecolor() {
        msg.truecolor(rgb.0, rgb.1, rgb.2).to_string()
    } else {
        format!("{}", msg.color(fallback))
    }
}
