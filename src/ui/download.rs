use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{self, Span},
    widgets::{Block, List, ListItem, Paragraph, Wrap},
    Frame,
};
use Constraint::{Fill, Length, Min, Percentage};

use crate::app::App;

pub fn draw_download_tab(frame: &mut Frame, content_area: Rect, app: &mut App) {
    let info_style = Style::default().fg(Color::Blue);
    let warning_style = Style::default().fg(Color::Yellow);
    let error_style = Style::default().fg(Color::Magenta);
    let critical_style = Style::default().fg(Color::Red);

    let mut text = vec![];
    for log in &app.connections_logs.items {
        let color = match &log.0 {
            &"INFO" => Color::Blue,
            &"DEBUG" => Color::Green,
            &"WARNING" => Color::Yellow,
            &"ERROR" => Color::Red,
            _ => Color::White,
        };
        text.push(text::Line::from(vec![
            Span::styled(
                format!("dd-mm-yyyy hh-mm-ss-mmm [{}] ", log.0),
                Style::default().fg(color),
            ),
            Span::from(log.1),
        ]));
    }
    let list_items: Vec<ListItem> = text
        .iter()
        .map(|line| ListItem::new(line.clone()))
        .collect();
    let connections_logs_widget =
        List::new(list_items).block(Block::bordered().title("Connections Logs"));
    //? removed the highlight things because we dont need them
    // .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    // .highlight_symbol("> ")

    let mut text = vec![];
    for log in &app.download_logs.items {
        let color = match &log.0 {
            &"INFO" => Color::Blue,
            &"DEBUG" => Color::Green,
            &"WARNING" => Color::Yellow,
            &"ERROR" => Color::Red,
            _ => Color::White,
        };
        text.push(text::Line::from(vec![
            Span::styled(
                format!("dd-mm-yyyy hh-mm-ss-mmm [{}] ", log.0),
                Style::default().fg(color),
            ),
            Span::from(log.1),
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

    frame.render_stateful_widget(download_logs_widget, left, &mut app.download_logs.state);
    frame.render_stateful_widget(
        connections_logs_widget,
        right,
        &mut app.connections_logs.state,
    );
    // frame.render_widget(log, top_right);
    // frame.render_widget(Block::bordered().title("Bottom Left"), bottom_left);
    // frame.render_widget(Block::bordered().title("Bottom Right"), bottom_right);
}
