mod client;
mod constants;
mod download;
mod io;
mod log;
mod peers;
mod torrentfile;
mod utils;

use clap::Parser;
use log::info;
use std::{env, fs::File, io::Read};
use torrentfile::bencode::Decoder;
use torrentfile::torrent::Torrent;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    torrent_file: String,

    #[arg(short, long, default_value = "~/Downloads")]
    download_dir: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    info(format!(
        "starting downloade for torrent: {}",
        args.torrent_file
    ));
    info(format!("download directory: {}", args.download_dir));

    //maybe we need a static PeerId
    let peer_id = utils::new_peer_id();
    //let path = "debian.torrent";
    let path = &args.torrent_file;
    let mut file = File::open(path).map_err(|e| e.to_string()).unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf)
        .map_err(|e| e.to_string())
        .unwrap();

    let bencode_data = Decoder::new(&buf).start().unwrap();
    let torrent_data = Torrent::new(bencode_data, peer_id).unwrap();
    let peers = peers::get_peers(&torrent_data, peer_id).unwrap();
    download::start(torrent_data, peers.peers).unwrap();

    Ok(())
}
