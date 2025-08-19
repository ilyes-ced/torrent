use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::{self, block},
    text::{self, Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Tabs, Wrap},
    Frame,
};
use Constraint::{Fill, Length, Min, Percentage};

use crate::ui::download::draw_download_tab;
use crate::ui::files::draw_files_tab;
use crate::ui::info::draw_info;
use crate::ui::peers::draw_peers_tab;
use crate::ui::progress::draw_progress;

mod download;
mod files;
mod info;
mod peers;
mod progress;

pub fn start_tui() {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw).expect("failed to draw frame");
        match event::read().expect("failed to read event") {
            Event::FocusGained => todo!(),
            Event::FocusLost => todo!(),
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Left => {}
                KeyCode::Right => {}
                _ => {}
            },
            Event::Mouse(mouse_event) => todo!(),
            Event::Paste(_) => todo!(),
            Event::Resize(_, _) => todo!(),
        }
    }
    ratatui::restore();
}

pub fn draw(frame: &mut Frame) {
    let vertical = Layout::vertical([Length(4), Min(0), Length(4)]);
    let [title_area, main_area, progress_area] = vertical.areas(frame.area());

    draw_info(frame, title_area);
    draw_tabs(frame, main_area);
    draw_progress(frame, progress_area);
}

pub fn draw_tabs(frame: &mut Frame, main_area: Rect) {
    let main_tabs = Layout::vertical([Length(1), Fill(1)]);
    let [tabs_area, content_area] = main_tabs.areas(main_area);

    let titles: Vec<Line> = vec!["Download", "Peers", "Files"]
        .iter()
        .map(|t| {
            let title = format!(" {} ", t); // add padding
            Line::from(Span::styled(
                title,
                Style::default().fg(Color::White).bg(Color::Black), // simulate block background
            ))
        })
        .collect();
    let tabs = Tabs::new(titles)
        .style(Style::default().white())
        .highlight_style(Style::default().red())
        .select(1)
        .divider(block::FULL)
        .padding(" ", " ");

    frame.render_widget(tabs, tabs_area);

    //draw_download_tab(frame, content_area);
    draw_peers_tab(frame, content_area);
    // draw_files_tab(frame, content_area);
}

/*
to avoid overloading the RAM with logs
use VecDeque for logs storage

fn update_logs(logs: &mut VecDeque<String>, new_log: String) {
    logs.push_back(new_log);
    if logs.len() > MAX_LOG_LINES {
        logs.pop_front();
    }
}
*/
