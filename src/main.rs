mod bencode;
mod constants;
mod download;
mod error;
mod log;
mod peers;
mod torrent;
mod utils;

use bencode::Decoder;
use download::writer;
use log::info;
use std::{
    fs::{self, File},
    io::Read,
    time::Duration,
};
use torrent::Torrent;

fn main() -> std::io::Result<()> {
    //maybe we need a static PeerId
    let peer_id = utils::new_peer_id();
    //let path = "debian.torrent";
    let path = "tests/torrents/yome.torrent";
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

    // let file = File::options()
    //     .read(true)
    //     .write(true)
    //     .open("test.txt")
    //     .map_err(|e| e.to_string())
    //     .unwrap();

    // let test = std::os::unix::fs::FileExt::write_at(
    //     &file,
    //     &[
    //         150, 239, 21, 88, 47, 62, 122, 139, 184, 00, 00, 00, 00, 00, 00, 00, 00,
    //     ],
    //     0,
    // )
    // .map_err(|e| e.to_string());

    Ok(())
}
