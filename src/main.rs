mod client;
mod constants;
mod download;
mod io;
mod log;
mod peers;
mod torrentfile;
mod ui;
mod utils;

use std::{error::Error, fs::File, io::Read, time::Duration};
use torrentfile::bencode::Decoder;
use torrentfile::torrent::Torrent;
use ui::{app, crossterm};

use argh::FromArgs;

/// Demo
#[derive(Debug, FromArgs)]
struct Cli {
    /// time in ms between two ticks.
    #[argh(option, default = "250")]
    tick_rate: u64,
    /// whether unicode symbols are used to improve the overall look of the app
    #[argh(option, default = "true")]
    enhanced_graphics: bool,

    #[argh(option, description = "torrent file path", short = 't')]
    torrent_path: String,
    #[argh(option, description = "download directory path", short = 'd')]
    download_dir: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = argh::from_env();
    let tick_rate = Duration::from_millis(cli.tick_rate);
    crate::crossterm::run(tick_rate, cli.enhanced_graphics).unwrap();

    Ok(())
}

fn start_torrent() -> Result<(), String> {
    //maybe we need a static PeerId
    let peer_id = utils::new_peer_id();
    //let path = "debian.torrent";
    let path = "tests/torrents/many_files.torrent";
    let mut file = File::open(path).map_err(|e| e.to_string())?;
    let mut buf = vec![];
    file.read_to_end(&mut buf).map_err(|e| e.to_string())?;

    let bencode_data = Decoder::new(&buf).start().unwrap();
    let torrent_data = Torrent::new(bencode_data, peer_id).unwrap();
    let peers = peers::get_peers(&torrent_data, peer_id).unwrap();
    download::start(torrent_data, peers.peers).unwrap();

    Ok(())
}
