use ratatui::widgets::ListState;

use crate::{
    torrent::Torrent,
    tracker::Peer,
    ui::{Log, LogType},
    Source,
};
pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}
impl<'a> TabsState<'a> {
    pub const fn new(titles: Vec<&'a str>) -> Self {
        Self { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}
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
    pub fn end(&mut self) {
        if self.items.len() > 0 {
            self.state.select(Some(self.items.len() - 1));
        }
    }
}

pub struct App<'a> {
    pub torrent_name: String,
    pub download_dir: String,
    pub info_hash: [u8; 20],
    pub size: usize,

    pub pieces: usize,
    pub downloaded_pieces: usize,

    pub download_logs: StatefulList<Log>,
    pub events_logs: StatefulList<Log>,

    pub peers: StatefulList<Peer>,

    pub tabs: TabsState<'a>,

    pub peerId: (),
    // .ttorrent or magnet url
    pub torrent_type: (),
    // path of .torrent or magnet url
    pub torrent_type_value: (),
}

impl<'a> App<'a> {
    pub fn new(torrent_data: Torrent, download_dir: String) -> Self {
        // todo: the size conversion, files tree ......
        App {
            torrent_name: torrent_data.info.name,
            download_dir: download_dir,
            info_hash: torrent_data.info_hash,
            size: 9999,

            pieces: 0,
            downloaded_pieces: 0,

            download_logs: StatefulList::with_items(Vec::new()),
            events_logs: StatefulList::with_items(Vec::new()),

            peers: StatefulList::with_items(vec![]),

            tabs: TabsState::new(vec!["Download", "Peers", "Files"]),

            peerId: (),
            torrent_type: (),
            torrent_type_value: (),
        }
    }
    ///////////////////////////////////////////////////////////////////////
    pub fn set_size(&mut self, size: usize) {
        //todo: here we get size in bytes and transform it to MiB or GiB
    }
    pub fn set_source(&mut self, source: Source) {
        //todo: here set source of .torrent or magnet url
    }
    pub fn set_infohash(&mut self, infohash: [u8; 20]) {
        //todo: here set infohash as array of bytes or as a hex string
    }
    ///////////////////////////////////////////////////////////////////////
    pub fn add_peer(&mut self, peer: Peer) {
        //todo: add peer to list
    }
    pub fn remove_peer(&mut self, peer: Peer) {
        //todo: remove peer to list
    }
    pub fn add_piece_downloaded(&mut self, index: u32, peer: Peer, size: usize) {
        //todo: add to progress and pieces downloaded, also add data to data downloaded by each peer
    }
    ///////////////////////////////////////////////////////////////////////
    pub fn add_download_logs(&mut self, log: Log) {
        self.download_logs.items.push(log);
    }
    pub fn add_event_logs(&mut self, log: Log) {
        self.events_logs.items.push(log);
    }
    ///////////////////////////////////////////////////////////////////////
    pub fn set_files(&mut self) {
        //todo: not yet
    }
    ///////////////////////////////////////////////////////////////////////
    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            _ => {}
        }
    }
    ///////////////////////////////////////////////////////////////////////
    pub fn on_tick(&mut self) {
        // self.download_logs.items.push(("ERROR", "new test values"));
        self.download_logs.end();
        // self.events_logs
        //     .items
        //     .push(("INFO", "new test values"));
        self.events_logs.end();
    }
}
