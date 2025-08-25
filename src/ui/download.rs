use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{self, Span},
    widgets::{Block, List, ListItem},
    Frame,
};
use Constraint::{Fill, Length};

use crate::{
    app::{ActiveBlock, App},
    ui::{info::draw_info, LogType},
};

pub fn draw_download_tab(frame: &mut Frame, content_area: Rect, app: &mut App) {
    let mut text = vec![];
    for log in &app.events_logs.items {
        let (fg, bg, log_type) = match log.log_type {
            LogType::Info => (Color::Blue, Color::Reset, "INFO"),
            LogType::Debug => (Color::Green, Color::Reset, "DEBUG"),
            LogType::Warning => (Color::Yellow, Color::Reset, "WARNING"),
            LogType::Error => (Color::Red, Color::Reset, "ERROR"),
            LogType::Critical => (Color::Black, Color::Red, "ERROR"),
        };

        text.push(text::Line::from(vec![
            Span::styled(
                format!("{} [{}]", log.timestamp, log_type),
                Style::default().fg(fg).bg(bg),
            ),
            Span::from(format!(" {}", &log.msg)),
        ]));
    }

    let events_block = if app.active_block == ActiveBlock::EventLog {
        Block::bordered()
            .border_style(Style::new().blue().bold())
            .title("Events Logs")
    } else {
        Block::bordered().title("Events Logs")
    };

    let list_items: Vec<ListItem> = text
        .iter()
        .map(|line| ListItem::new(line.clone()))
        .collect();
    let events_logs_widget = List::new(list_items)
        .block(events_block)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    let mut text = vec![];
    for log in &app.download_logs.items {
        let (fg, bg, log_type) = match log.log_type {
            LogType::Info => (Color::Blue, Color::Reset, "INFO"),
            LogType::Debug => (Color::Green, Color::Reset, "DEBUG"),
            LogType::Warning => (Color::Yellow, Color::Reset, "WARNING"),
            LogType::Error => (Color::Red, Color::Reset, "ERROR"),
            LogType::Critical => (Color::Reset, Color::Red, "ERROR"),
        };

        text.push(text::Line::from(vec![
            Span::styled(
                format!("{} [{}] ", log.timestamp, log_type),
                Style::default().fg(fg).bg(bg),
            ),
            Span::from(format!(" {}", &log.msg)),
        ]));
    }
    let list_items: Vec<ListItem> = text
        .iter()
        .map(|line| ListItem::new(line.clone()))
        .collect();

    let downnload_block = if app.active_block == ActiveBlock::DownloadLog {
        Block::bordered()
            .border_style(Style::new().blue().bold())
            .title("Download Logs")
    } else {
        Block::bordered().title("Download Logs")
    };

    let download_logs_widget = List::new(list_items)
        .block(downnload_block)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

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
