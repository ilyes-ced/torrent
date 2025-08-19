use std::time::{Duration, Instant};

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

use crate::ui::files::draw_files_tab;
use crate::ui::info::draw_info;
use crate::ui::peers::draw_peers_tab;
use crate::ui::progress::draw_progress;
use crate::{app::App, ui::download::draw_download_tab};

mod download;
mod files;
mod info;
mod peers;
mod progress;

/*

enum AppEvent {
    NewPeer(PeerInfo),
    PieceDownloaded { index: u32, bytes: Vec<u8> },
    DownloadLog(String),
    ConnectionLog(String),
}
*/

pub fn start_tui() {
    let mut terminal = ratatui::init();
    let mut app = App::new("Crossterm Demo", true);
    let mut last_tick = Instant::now();

    loop {
        /*
        tokio::select! {
            Some(event) = rx.recv() => {
                match event {
                    AppEvent::Log(msg) => app_state.logs.push(msg),
                    AppEvent::NewPeer(peer) => app_state.peers.push(peer),
                    AppEvent::Tick => { /* maybe update progress */ },
                    _ => {}
                }
            }

            _ = tick_interval.tick() => {
                // Regular redraws or tick events
                app_state.should_redraw = true;
            }
        }
        */

        terminal
            .draw(|frame| draw(frame, &mut app))
            .expect("failed to draw frame");

        let timeout = Duration::from_millis(100).saturating_sub(last_tick.elapsed());
        if !event::poll(timeout).unwrap() {
            app.on_tick();
            last_tick = Instant::now();
            continue;
        }

        match event::read().expect("failed to read event") {
            Event::FocusGained => todo!(),
            Event::FocusLost => todo!(),
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Left => app.on_left(),
                KeyCode::Right => app.on_right(),
                KeyCode::Tab => app.on_right(),
                _ => {}
            },
            Event::Mouse(mouse_event) => todo!(),
            Event::Paste(_) => todo!(),
            Event::Resize(_, _) => todo!(),
        }
    }
    ratatui::restore();
}

pub fn draw(frame: &mut Frame, app: &mut App) {
    let vertical = Layout::vertical([Length(4), Min(0), Length(4)]);
    let [title_area, main_area, progress_area] = vertical.areas(frame.area());

    draw_info(frame, title_area, app);
    draw_tabs(frame, main_area, app);
    draw_progress(frame, progress_area, app);
}

pub fn draw_tabs(frame: &mut Frame, main_area: Rect, app: &mut App) {
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
    // let tabs = Tabs::new(titles)
    //     .style(Style::default().white())
    //     .highlight_style(Style::default().red())
    //     .select(app.tabs.index)
    //     .divider(block::FULL)
    //     .padding(" ", " ");

    let tabs = app
        .tabs
        .titles
        .iter()
        .map(|t| text::Line::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect::<Tabs>()
        .highlight_style(Style::default().red())
        .divider(block::FULL)
        .padding(" ", " ")
        .select(app.tabs.index);
    frame.render_widget(tabs, tabs_area);

    match app.tabs.index {
        0 => draw_download_tab(frame, content_area, app),
        1 => draw_peers_tab(frame, content_area, app),
        2 => draw_files_tab(frame, content_area, app),
        _ => {}
    };

    // draw_download_tab(frame, content_area);
    // draw_peers_tab(frame, content_area);
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
