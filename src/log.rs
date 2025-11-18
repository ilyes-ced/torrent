//? no longer in use because of the TUI change

use crate::ui::{
    AppEvent::{self, EventLog},
    Log,
    LogType::{self, Critical, Debug, Error, Info, Warning},
};
use chrono::prelude::{DateTime, Local};
use tokio::sync::mpsc::Sender;

//enum Color {
//    Red,
//    Green,
//    Yellow,
//    Blue,
//}
//
//pub fn info(msg: String) {
//    println!(
//        "{} {} {}",
//        color(time_date(), Color::Blue),
//        color("[ INFO ]".to_string(), Color::Blue),
//        color(msg, Color::Blue),
//    );
//}
//pub fn debug(msg: String) {
//    println!(
//        "{} {} {}",
//        color(time_date(), Color::Green),
//        color("[ DEBUG ]".to_string(), Color::Green),
//        color(msg, Color::Green),
//    );
//}
//pub fn warning(msg: String) {
//    println!(
//        "{} {} {}",
//        color(time_date(), Color::Yellow),
//        color("[ WARNING ]".to_string(), Color::Yellow),
//        color(msg, Color::Yellow),
//    );
//}
//pub fn error(msg: String) {
//    println!(
//        "{} {} {}",
//        color(time_date(), Color::Red),
//        color("[ ERROR ]".to_string(), Color::Red),
//        color(msg, Color::Red),
//    );
//}
//
//fn color(txt: String, color: Color) -> String {
//    match color {
//        Color::Red => format!("\x1b[31m{}\x1b[0m", txt),
//        Color::Green => format!("\x1b[32m{}\x1b[0m", txt),
//        Color::Yellow => format!("\x1b[33m{}\x1b[0m", txt),
//        Color::Blue => format!("\x1b[34m{}\x1b[0m", txt),
//    }
//}
//
pub enum LogTarget {
    Event,
    Download,
}

const DEBUG: bool = false;

pub async fn info(msg: String, tx_tui: &Sender<AppEvent>) {
    log_message(msg, LogType::Info, tx_tui, LogTarget::Event).await;
}

pub async fn info_download(msg: String, tx_tui: &Sender<AppEvent>) {
    log_message(msg, LogType::Info, tx_tui, LogTarget::Download).await;
}

pub async fn debug(msg: String, tx_tui: &Sender<AppEvent>) {
    if DEBUG {
        log_message(msg, LogType::Debug, tx_tui, LogTarget::Event).await;
    }
}

pub async fn warning(msg: String, tx_tui: &Sender<AppEvent>) {
    log_message(msg, LogType::Warning, tx_tui, LogTarget::Event).await;
}

pub async fn error(msg: String, tx_tui: &Sender<AppEvent>) {
    log_message(msg, LogType::Error, tx_tui, LogTarget::Event).await;
}

pub async fn critical(msg: String, tx_tui: &Sender<AppEvent>) {
    log_message(msg, LogType::Critical, tx_tui, LogTarget::Event).await;
}

pub async fn log_message(
    msg: String,
    log_type: LogType,
    tx_tui: &Sender<AppEvent>,
    target: LogTarget,
) {
    let log = Log {
        timestamp: time_date(),
        log_type,
        msg,
    };

    let event = match target {
        LogTarget::Event => AppEvent::EventLog(log),
        LogTarget::Download => AppEvent::DownloadLog(log),
    };

    let _ = tx_tui.send(event).await;
}

fn time_date() -> String {
    // e.g. `2014-11-28T21:45:59.324310806+09:00`
    Local::now().format("%Y-%m-%d %H:%M:%S.%3f").to_string()
}
