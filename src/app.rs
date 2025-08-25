use std::collections::VecDeque;

use ratatui::widgets::ListState;
const LOGS_MAX_LEN: usize = 1000;
use crate::{
    torrent::Torrent,
    tracker::Peer,
    ui::{Log, LogType},
    utils::readable_size,
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
    pub items: VecDeque<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: VecDeque<T>) -> Self {
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
#[derive(PartialEq)]
pub enum ActiveBlock {
    DownloadLog,
    EventLog,
    Peers,
    DHT,
    Files,
}

pub struct App<'a> {
    pub torrent_name: String,
    pub download_dir: String,
    pub info_hash: [u8; 20],
    pub size: String,

    pub pieces: usize,
    pub downloaded_pieces: usize,

    pub download_logs: StatefulList<Log>,
    pub events_logs: StatefulList<Log>,

    pub peers: StatefulList<Peer>,

    pub active_block: ActiveBlock,
    pub tabs: TabsState<'a>,

    pub peer_id: String,
    // .ttorrent or magnet url
    pub torrent_type: (),
    // path of .torrent or magnet url
    pub torrent_type_value: (),
}

impl<'a> App<'a> {
    pub fn new(torrent_data: Torrent, download_dir: String, peer_id: [u8; 20]) -> Self {
        // todo: the size conversion, files tree ......

        let size = match torrent_data.info.files {
            crate::torrent::FileInfo::Single(file) => file,
            crate::torrent::FileInfo::Multiple(files) => files.iter().map(|f| f.length).sum(),
        } as f64;

        let readable_size = readable_size(size);

        let num_pieces = (size / torrent_data.info.piece_length as f64).ceil() as usize;

        App {
            torrent_name: torrent_data.info.name,
            download_dir: download_dir,
            info_hash: torrent_data.info_hash,
            size: readable_size,

            pieces: num_pieces,
            downloaded_pieces: 0,

            download_logs: StatefulList::with_items(VecDeque::new()),
            events_logs: StatefulList::with_items(VecDeque::new()),

            peers: StatefulList::with_items(VecDeque::new()),

            active_block: ActiveBlock::DownloadLog,
            tabs: TabsState::new(vec!["Download", "Peers", "Files"]),

            peer_id: peer_id
                .iter()
                .map(|byte| format!("{:02X}", byte))
                .collect::<Vec<String>>()
                .join(""),
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
    //? these 2 are limited to 1000 logs because if there is too many logs it might overflow the RAM
    pub fn add_download_logs(&mut self, log: Log) {
        if self.download_logs.items.len() == LOGS_MAX_LEN {
            self.download_logs.items.pop_front();
        }
        self.download_logs.items.push_back(log);
        //? should this exist because as it is when new logs appear when we are scolling it will force scroll down
        self.download_logs.end();
    }
    pub fn add_event_logs(&mut self, log: Log) {
        if self.events_logs.items.len() == LOGS_MAX_LEN {
            self.events_logs.items.pop_front();
        }
        self.events_logs.items.push_back(log);
        //? should this exist because as it is when new logs appear when we are scolling it will force scroll down
        self.events_logs.end();
    }
    ///////////////////////////////////////////////////////////////////////
    pub fn set_files(&mut self) {
        //todo: not yet
    }
    ///////////////////////////////////////////////////////////////////////
    // todo: change these to change active blocks, if active blocks is last to right change tab and change active block to first block in the new tab
    pub fn on_right(&mut self) {
        match self.active_block {
            ActiveBlock::DownloadLog => self.active_block = ActiveBlock::EventLog,
            ActiveBlock::EventLog => {
                self.tabs.next();
                self.active_block = ActiveBlock::Peers
            }
            ActiveBlock::Peers => self.active_block = ActiveBlock::DHT,
            ActiveBlock::DHT => {
                self.tabs.next();
                self.active_block = ActiveBlock::Files
            }
            ActiveBlock::Files => {
                self.tabs.next();
                self.active_block = ActiveBlock::DownloadLog
            }
        }
    }
    pub fn on_left(&mut self) {
        match self.active_block {
            ActiveBlock::DownloadLog => {
                self.tabs.previous();
                self.active_block = ActiveBlock::Files
            }
            ActiveBlock::EventLog => self.active_block = ActiveBlock::DownloadLog,
            ActiveBlock::Peers => {
                self.tabs.previous();
                self.active_block = ActiveBlock::EventLog
            }
            ActiveBlock::DHT => self.active_block = ActiveBlock::Peers,
            ActiveBlock::Files => {
                self.tabs.previous();
                self.active_block = ActiveBlock::DHT
            }
        }
    }

    ///////////////////////////////////////////////////////////////////////
    pub fn on_down(&mut self) {
        match self.active_block {
            ActiveBlock::DownloadLog => self.download_logs.next(),
            ActiveBlock::EventLog => self.events_logs.next(),
            ActiveBlock::Peers => self.peers.next(),
            ActiveBlock::DHT => todo!(),
            ActiveBlock::Files => todo!(),
        }
    }
    pub fn on_up(&mut self) {
        match self.active_block {
            ActiveBlock::DownloadLog => self.download_logs.previous(),
            ActiveBlock::EventLog => self.events_logs.previous(),
            ActiveBlock::Peers => self.peers.previous(),
            ActiveBlock::DHT => todo!(),
            ActiveBlock::Files => todo!(),
        }
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            _ => {}
        }
    }
    ///////////////////////////////////////////////////////////////////////
    pub fn on_tick(&mut self) {}
}
