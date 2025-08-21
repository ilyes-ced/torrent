use ratatui::widgets::ListState;

use crate::tracker::Peer;
const LOGS: [(&str, &str); 23] = [
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
        self.state.select(Some(self.items.len() - 1));
    }
}

pub struct App<'a> {
    pub torrent_name: &'a str,
    pub download_dir: &'a str,
    pub info_hash: &'a str,
    pub size: &'a str,

    pub pieces: usize,
    pub downloaded_pieces: usize,

    pub download_logs: StatefulList<(&'a str, &'a str)>,
    pub connections_logs: StatefulList<(&'a str, &'a str)>,

    pub peers: StatefulList<Peer>,

    pub tabs: TabsState<'a>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> Self {
        App {
            torrent_name: "torrent_name",
            download_dir: "download_dir",
            info_hash: "infohash",
            size: "size GiB",

            pieces: 1500,
            downloaded_pieces: 250,

            download_logs: StatefulList::with_items(LOGS.to_vec()),
            connections_logs: StatefulList::with_items(LOGS.to_vec()),

            peers: StatefulList::with_items(vec![]),

            tabs: TabsState::new(vec!["Download", "Peers", "Files"]),
        }
    }

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

    pub fn on_tick(&mut self) {
        self.downloaded_pieces += 1;

        // self.download_logs.items.push(("ERROR", "new test values"));
        self.download_logs.end();
        // self.connections_logs
        //     .items
        //     .push(("INFO", "new test values"));
        self.connections_logs.end();
    }
}
