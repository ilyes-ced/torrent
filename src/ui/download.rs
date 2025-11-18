use std::path::Prefix;

use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{self, Line, Span},
    widgets::{Block, List, ListItem, Paragraph},
    Frame,
};
use Constraint::{Fill, Length, Percentage};

use crate::{
    app::{ActiveBlock, App},
    ui::{info::draw_info, LogType},
};

pub fn draw_download_tab(frame: &mut Frame, content_area: Rect, app: &mut App) {
    ////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////

    let main_tabs = Layout::horizontal([Percentage(30), Percentage(70)]);
    let [left, right] = main_tabs.areas(content_area);

    let left_area = Layout::vertical([Length(7), Fill(1)]);
    let [info_area, download_area] = left_area.areas(left);

    draw_info(frame, info_area, app);

    let events_logs_widget = log_blocks(right, app, ActiveBlock::EventLog);
    let download_logs_widget = log_blocks(download_area, app, ActiveBlock::DownloadLog);

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

fn log_blocks(content_area: Rect, app: &mut App, active_block: ActiveBlock) -> List<'static> {
    let logs = log_lines(content_area, app, &active_block);

    let name = if active_block == ActiveBlock::EventLog {
        "Event Log"
    } else if active_block == ActiveBlock::DownloadLog {
        "Downlaod Log"
    } else {
        panic!("should never happen")
    };

    let events_block = if app.active_block == active_block {
        Block::bordered()
            .border_style(Style::new().blue().bold())
            .title(name)
    } else {
        Block::bordered().title(name)
    };

    let list_items: Vec<ListItem> = logs
        .iter()
        .map(|line| ListItem::new(line.clone()))
        .collect();
    List::new(list_items)
        .block(events_block)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ")
}

fn log_lines(content_area: Rect, app: &mut App, active_block: &ActiveBlock) -> Vec<Line<'static>> {
    let mut text = vec![];

    let logs = if active_block == &ActiveBlock::EventLog {
        &app.events_logs.items
    } else if active_block == &ActiveBlock::DownloadLog {
        &app.download_logs.items
    } else {
        panic!("should never happen")
    };

    for log in logs {
        let (fg, bg, log_type) = match log.log_type {
            LogType::Info => (Color::Blue, Color::Reset, "INFO"),
            LogType::Debug => (Color::Green, Color::Reset, "DEBUG"),
            LogType::Warning => (Color::Yellow, Color::Reset, "WARNING"),
            LogType::Error => (Color::Red, Color::Reset, "ERROR"),
            LogType::Critical => (Color::Reset, Color::Red, "ERROR"),
        };

        let log_prefix = format!("{} [{}]", log.timestamp, log_type);

        let (first_line, lines) =
            truncate_lines(content_area.width as usize, log_prefix.len(), &log.msg);

        text.push(text::Line::from(vec![
            Span::styled(log_prefix, Style::default().fg(fg).bg(bg)),
            Span::from(format!(" {}", first_line)),
        ]));

        for line in lines {
            text.push(text::Line::from(vec![Span::from(format!(" {}", line))]));
        }
    }
    text
}

fn truncate_lines(
    widget_width: usize,
    log_prefix_len: usize,
    log_msg: &str,
) -> (String, Vec<String>) {
    let mut lines: Vec<String> = Vec::new();
    let mut line = String::new();
    let mut first_line = String::new();
    let mut used_words = 0;

    let words: Vec<&str> = log_msg.split_whitespace().collect();

    for word in words.iter() {
        // -2 for the borders
        if (first_line.len() + word.len()) >= (widget_width - log_prefix_len - 6) {
            break;
        }
        first_line.push_str(word);
        first_line.push(' ');
        used_words += 1;
    }

    for word in &words[used_words..] {
        if line.len() + word.len() > widget_width - 2 {
            lines.push(line.trim_end().to_string());
            line = String::new();
        }
        line.push_str(word);
        line.push(' ');
    }
    // add last line which is shorter than full line
    if line.len() > 0 {
        lines.push(line.trim_end().to_string());
    }

    (first_line, lines)
}

//fn wrrite_to_file(txt: String) {
//    use std::io::Write;
//    let path = "results.txt";
//    let mut output = std::fs::OpenOptions::new()
//        .create(true)
//        .append(true)
//        .open(path)
//        .expect("Failed to open file");
//    writeln!(output, "{txt}").expect("Failed to write to file");
//}
