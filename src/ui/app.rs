use std::{fs::File, io::Read};

use rand::{
    distr::{Distribution, Uniform},
    rngs::ThreadRng,
};
use ratatui::widgets::ListState;

use crate::{
    download,
    peers::{self, PeersResult},
    torrentfile::{bencode::Decoder, torrent::Torrent},
    utils, Cli,
};

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}
impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> Self {
        Self {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub enhanced_graphics: bool,
    pub peer_id: [u8; 20],
    pub torrent_path: String,
    pub download_dir: String,

    pub connections_logs: StatefulList<(&'a str, &'a str)>,
    pub downloads_logs: StatefulList<(&'a str, &'a str)>,
    pub torrent: Torrent,
    pub peers: PeersResult,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, cli: Cli) -> Self {
        App {
            title,
            should_quit: false,
            enhanced_graphics: cli.enhanced_graphics,
            peer_id: utils::new_peer_id(),
            torrent_path: cli.torrent_path,
            download_dir: cli.download_dir,
            connections_logs: StatefulList::with_items(vec![]),
            downloads_logs: StatefulList::with_items(vec![]),
            torrent: Torrent::default(),
            peers: PeersResult {
                peers: vec![],
                _interval: 0,
            },
        }
    }

    pub fn on_up(&mut self) {}

    pub fn on_down(&mut self) {}

    pub fn on_right(&mut self) {}

    pub fn on_left(&mut self) {}

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {}
}
