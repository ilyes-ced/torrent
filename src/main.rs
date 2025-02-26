mod bencode;
mod client;
mod constants;
mod download;
mod error;
mod log;
mod peers;
mod torrent;
mod utils;
mod writer;

use bencode::Decoder;
use log::info;
use std::{fs::File, io::Read};
use torrent::Torrent;

fn main() -> std::io::Result<()> {
    //maybe we need a static PeerId
    let peer_id = utils::new_peer_id();
    //let path = "debian.torrent";
    let path = "tests/torrents/many_files.torrent";
    let mut file = File::open(path)?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    // add error handling here maybe for all of those function calls
    let bencode_data = Decoder::new(&buf).start().unwrap();
    let torrent_data = Torrent::new(bencode_data, peer_id).unwrap();
    let peers = peers::get_peers(&torrent_data, peer_id).unwrap();
    // peers.interval can use it later
    info(format!("{:?}", peers));
    let _download = download::start(torrent_data, peers.peers).unwrap();

    Ok(())
}
