use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{self, Span},
    widgets::{Block, List, ListItem},
    Frame,
};
use Constraint::{Fill, Length};

use crate::{
    app::App,
    ui::{info::draw_info, LogType},
};

pub fn draw_download_tab(frame: &mut Frame, content_area: Rect, app: &mut App) {
    let mut text = vec![];
    for log in &app.events_logs.items {
        let (fg, log_type) = match log.log_type {
            LogType::Info => (Color::Blue, "INFO"),
            LogType::Debug => (Color::Green, "DEBUG"),
            LogType::Warning => (Color::Yellow, "WARNING"),
            LogType::Error => (Color::Red, "ERROR"),
            LogType::Critical => todo!(),
        };

        text.push(text::Line::from(vec![
            Span::styled(
                format!("{} [{}] ", log.timestamp, log_type),
                Style::default().fg(fg),
            ),
            Span::from(&log.msg),
        ]));
    }
    let list_items: Vec<ListItem> = text
        .iter()
        .map(|line| ListItem::new(line.clone()))
        .collect();
    let events_logs_widget =
        List::new(list_items).block(Block::bordered().title("Connections Logs"));

    //? removed the highlight things because we dont need them
    // .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    // .highlight_symbol("> ")

    let mut text = vec![];
    for log in &app.download_logs.items {
        let (fg, log_type) = match log.log_type {
            LogType::Info => (Color::Blue, "INFO"),
            LogType::Debug => (Color::Green, "DEBUG"),
            LogType::Warning => (Color::Yellow, "WARNING"),
            LogType::Error => (Color::Red, "ERROR"),
            LogType::Critical => todo!(),
        };

        text.push(text::Line::from(vec![
            Span::styled(
                format!("{} [{}] ", log.timestamp, log_type),
                Style::default().fg(fg).bg(Color::Red),
            ),
            Span::from(&log.msg),
        ]));
    }
    let list_items: Vec<ListItem> = text
        .iter()
        .map(|line| ListItem::new(line.clone()))
        .collect();
    let download_logs_widget =
        List::new(list_items).block(Block::bordered().title("Download Logs"));

    // "dd-mm-yyyy hh-mm-ss-mmm [INFO] lorem ipsum lorem ipsum"

    let main_tabs = Layout::horizontal([Fill(1), Fill(1)]);
    let [left, right] = main_tabs.areas(content_area);

    let left_area = Layout::vertical([Length(5), Fill(1)]);
    let [info_area, download_area] = left_area.areas(left);

    draw_info(frame, info_area, app);

    frame.render_stateful_widget(
        download_logs_widget,
        download_area,
        &mut app.download_logs.state,
    );
    frame.render_stateful_widget(events_logs_widget, right, &mut app.events_logs.state);
    // frame.render_widget(log, top_right);
    // frame.render_widget(Block::bordered().title("Bottom Left"), bottom_left);
    // frame.render_widget(Block::bordered().title("Bottom Right"), bottom_right);
}
