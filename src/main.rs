mod client;
mod constants;
mod download;
mod io;
mod log;
mod magnet;
mod peers;
mod torrentfile;
mod utils;

use clap::Parser;
use log::{error, info};
use magnet::Magnet;
use std::path::Path;
use std::{fs::File, io::Read};
use torrentfile::bencode::Decoder;
use torrentfile::torrent::Torrent;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(flatten)]
    source: Source,
    // ~/Downloads used to work fine idk why it changed
    #[arg(short, long, default_value = "/home/ilyes/Downloads")]
    download_dir: String,
}
#[derive(Parser, Debug)]
#[group(required = true, multiple = false)]
pub struct Source {
    #[arg(short, long)]
    torrent_file: Option<String>,
    #[arg(short, long)]
    magnet_url: Option<String>,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    // download directory checking
    info(format!("download directory: {}", args.download_dir));
    if !Path::new(&args.download_dir).exists() {
        error(format!("the provided directory does not exist"));
        std::process::exit(0);
    }

    let peer_id = utils::new_peer_id();
    // get torrent data torrent_file or magnet_url
    let res = if args.source.magnet_url == None {
        info(format!(
            "starting downloade for torrent: {}",
            args.source.torrent_file.clone().unwrap()
        ));
        let path = &args.source.torrent_file.unwrap();
        let mut file = File::open(path).map_err(|e| e.to_string()).unwrap();
        let mut buf = vec![];
        file.read_to_end(&mut buf)
            .map_err(|e| e.to_string())
            .unwrap();
        buf
    } else {
        let magnet_data = Magnet::new(&args.source.magnet_url.unwrap());
        info(format!("magnet data: {:?}", magnet_data));
        std::process::exit(0);
        Vec::new()
    };
    // reading torrent file
    //maybe we need a static PeerId

    // execution
    let bencode_data = Decoder::new(&res).start().unwrap();
    let torrent_data = Torrent::new(bencode_data, peer_id).unwrap();
    let peers = peers::get_peers(&torrent_data, peer_id).unwrap();
    download::start(torrent_data, peers.peers, args.download_dir).unwrap();

    Ok(())
}
