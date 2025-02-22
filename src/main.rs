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
};
use torrent::Torrent;

fn main() -> std::io::Result<()> {
    //  //maybe we need a static PeerId
    //  let peer_id = utils::new_peer_id();
    //  //let path = "debian.torrent";
    //  let path = "tests/torrents/zip.torrent";
    //  let mut file = File::open(path)?;
    //  let mut buf = vec![];
    //  file.read_to_end(&mut buf)?;

    //  // add error handling here maybe for all of those function calls
    //  let bencode_data = Decoder::new(&buf).start().unwrap();
    //  let torrent_data = Torrent::new(bencode_data, peer_id).unwrap();
    //  let peers = peers::get_peers(&torrent_data, peer_id).unwrap();
    //  // peers.interval can use it later
    //  info(format!("{:?}", peers));
    //  let _download = download::start(torrent_data, peers.peers).unwrap();

    let torrent = Torrent {
        info: torrent::TorrentInfo {
            name: "POLEN23E.zip".to_string(),
            piece_length: 4190208,
            pieces: Default::default(),
            files: torrent::FileInfo::Single(34561190),
        },
        announce: Default::default(),
        announce_list: Default::default(),
        comment: Default::default(),
        creation_date: Default::default(),
        created_by: Default::default(),
        info_hash: Default::default(),
        peer_id: Default::default(),
    };

    let mut files = [
        (File::open("piece_7.txt")?, 7),
        (File::open("piece_8.txt")?, 8),
        (File::open("piece_5.txt")?, 5),
        (File::open("piece_2.txt")?, 2),
        (File::open("piece_1.txt")?, 1),
        (File::open("piece_3.txt")?, 3),
        (File::open("piece_0.txt")?, 0),
        (File::open("piece_4.txt")?, 4),
        (File::open("piece_6.txt")?, 6),
    ];

    for ind in 0..files.len() {
        info(format!("for piece: {}", files[ind].1));
        let metadata = files[files[ind].1]
            .0
            .metadata()
            .expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        files[files[ind].1]
            .0
            .read(&mut buffer)
            .expect("buffer overflow");
        let res = writer::write_single_file(&torrent, files[ind].1.try_into().unwrap(), buffer);
        // info(format!("{:?}", res));
    }

    Ok(())
}
