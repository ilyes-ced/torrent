use chrono::{
    prelude::{DateTime, Local},
    Datelike, Timelike,
};

pub fn info(msg: String) {
    println!("\033[93mError\033[0m");
}
pub fn debug(msg: String) {
    println!("\033[93mError\033[0m");
}
pub fn warning(msg: String) {
    println!("\033[93mError\033[0m");
}
pub fn error(msg: String) {
    println!("\033[93mError\033[0m");
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
