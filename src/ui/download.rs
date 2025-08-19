use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{self, Span},
    widgets::{Block, List, ListItem, Paragraph, Wrap},
    Frame,
};
use Constraint::{Fill, Length, Min, Percentage};

pub fn draw_download_tab(frame: &mut Frame, content_area: Rect) {
    let download_logs: Vec<(&str, &str)> = vec![
        ("INFO", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("INFO", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("INFO", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("ERROR", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("ERROR", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("ERROR", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("ERROR", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
    ];

    let mut text = vec![];
    for log in download_logs {
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

    let download_logs_widget = Paragraph::new(text)
        .block(Block::bordered().title("Connections Log"))
        .wrap(Wrap { trim: false });

    let connections_logs: Vec<(&str, &str)> = vec![
        ("INFO", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("INFO", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("INFO", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("ERROR", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("ERROR", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("ERROR", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("ERROR", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
    ];

    let mut text = vec![];
    for log in connections_logs {
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
    let connections_logs_widget = List::new(list_items)
        .block(Block::bordered().title("Download Logs (List)"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    // "dd-mm-yyyy hh-mm-ss-mmm [INFO] lorem ipsum lorem ipsum"

    let main_tabs = Layout::horizontal([Fill(1), Fill(1)]);
    let [left, right] = main_tabs.areas(content_area);

    frame.render_widget(download_logs_widget, left);
    frame.render_widget(connections_logs_widget, right);
    // frame.render_widget(log, top_right);
    // frame.render_widget(Block::bordered().title("Bottom Left"), bottom_left);
    // frame.render_widget(Block::bordered().title("Bottom Right"), bottom_right);
}
