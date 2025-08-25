use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::block,
    text::{self, Line, Span},
    widgets::Tabs,
    Frame,
};
use tokio::sync::mpsc::Receiver;
use Constraint::{Fill, Length, Min};

use crate::ui::progress::draw_progress;
use crate::Source;
use crate::{app::App, ui::download::draw_download_tab};
use crate::{torrent::Torrent, ui::peers::draw_peers_tab};
use crate::{tracker::Peer, ui::files::draw_files_tab};

mod download;
mod files;
mod info;
mod peers;
mod progress;

pub enum LogType {
    Error,
    Debug,
    Info,
    Warning,
    Critical,
}

pub struct Log {
    //? (timestamp, logtype, msg)
    pub timestamp: String,
    pub log_type: LogType,
    pub msg: String,
}
pub enum AppEvent {
    TorrentName(String),
    Size(usize),
    DownloadDir(String),
    Infohash([u8; 20]),

    // .torrent OR magnet_url
    DownloadType(Source),

    Files(Vec<String>),

    NewPeer(Peer),
    RemovePeer(Peer),

    // peer and size to keep track of how much is downloaded from each peer
    PieceDownloaded { index: u32, peer: Peer, size: usize },
    DownloadLog(Log),
    EventLog(Log),
    // add dht events later
}

pub fn start_tui(
    mut rx_app: Receiver<AppEvent>,
    torrent_data: Torrent,
    download_dir: String,
    peer_id: [u8; 20],
) {
    let mut terminal = ratatui::init();

    let mut app = App::new(torrent_data, download_dir, peer_id);
    let mut last_tick = Instant::now();

    loop {
        while let Ok(event) = rx_app.try_recv() {
            match event {
                AppEvent::TorrentName(name) => app.torrent_name = name,
                AppEvent::Size(size) => app.set_size(size),
                AppEvent::DownloadDir(dir) => app.download_dir = dir,
                AppEvent::Infohash(infohash) => app.set_infohash(infohash),
                AppEvent::DownloadType(source) => app.set_source(source),
                AppEvent::Files(items) => app.set_files(),
                AppEvent::NewPeer(peer) => app.add_peer(peer),
                AppEvent::RemovePeer(peer) => app.remove_peer(peer),
                AppEvent::PieceDownloaded { index, peer, size } => {
                    app.add_piece_downloaded(index, peer, size)
                }
                AppEvent::DownloadLog(log) => app.add_download_logs(log),
                AppEvent::EventLog(log) => app.add_event_logs(log),
            }
        }

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
                KeyCode::Down => app.on_down(),
                KeyCode::Up => app.on_up(),

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
    let vertical = Layout::vertical([Min(0), Length(4)]);
    let [main_area, progress_area] = vertical.areas(frame.area());

    // draw_info(frame, title_area, app);
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
