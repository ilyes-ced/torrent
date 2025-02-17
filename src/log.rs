use chrono::{
    prelude::{DateTime, Local},
    Datelike, Timelike,
};

enum Color {
    Red,
    Green,
    Yellow,
    Blue,
}

pub fn info(msg: String) {
    println!(
        "{} {} {}",
        color(time_date(), Color::Blue),
        color("[ INFO ]".to_string(), Color::Blue),
        color(msg, Color::Blue),
    );
}
pub fn debug(msg: String) {
    println!(
        "{} {} {}",
        color(time_date(), Color::Green),
        color("[ DEBUG ]".to_string(), Color::Green),
        color(msg, Color::Green),
    );
}
pub fn warning(msg: String) {
    println!(
        "{} {} {}",
        color(time_date(), Color::Yellow),
        color("[ WARNING ]".to_string(), Color::Yellow),
        color(msg, Color::Yellow),
    );
}
pub fn error(msg: String) {
    println!(
        "{} {} {}",
        color(time_date(), Color::Red),
        color("[ ERROR ]".to_string(), Color::Red),
        color(msg, Color::Red),
    );
}

fn color(txt: String, color: Color) -> String {
    match color {
        Color::Red => return format!("\x1b[31m{}\x1b[0m", txt),
        Color::Green => return format!("\x1b[32m{}\x1b[0m", txt),
        Color::Yellow => return format!("\x1b[33m{}\x1b[0m", txt),
        Color::Blue => return format!("\x1b[34m{}\x1b[0m", txt),
    }
}

fn time_date() -> String {
    // e.g. `2014-11-28T21:45:59.324310806+09:00`}
    let local: DateTime<Local> = Local::now();
    format!(
        "{}-{}-{} {}:{}:{}",
        local.year(),
        local.month(),
        local.day(),
        local.hour(),
        local.minute(),
        local.second()
    )
}
