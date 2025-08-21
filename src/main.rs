mod app;
mod bencode;
mod client;
mod constants;
mod dht;
mod download;
mod io;
mod log;
mod magnet;
mod torrentfile;
mod tracker;
mod ui;
mod utils;

use clap::Parser;
use dht::Dht;
use log::{error, info};
use magnet::Magnet;
use ratatui::crossterm::event::{self, Event, KeyCode};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{self, Span, Text};
use ratatui::widgets::{Block, BorderType, Borders, Gauge, Paragraph, Tabs, Wrap};
use serde_json::Value;
use std::fmt::format;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use std::{fs::File, io::Read};
use tokio::sync::mpsc::{self, Sender};
use torrentfile::torrent::Torrent;

use tokio;

use crate::bencode::decoder::Decoder;
use crate::tracker::Peer;
use crate::ui::start_tui;

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

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // infohash
    // 6fcf7ef136e73f0fb6186b30fe67d741cc260c5c

    ////################################################################################
    //* DHT protocol (DHT thread)
    // let mut dht = Dht::new().await.unwrap();
    // error("***********************************************************".to_string());
    // // error(format!("dht:: {:#?}", dht));
    // error("***********************************************************".to_string());
    // dht.bootstrap().await.unwrap();
    // dht.lookup().await.unwrap();
    // error("***********************************************************".to_string());
    // //error(format!("dht:: {:#?}", dht));
    // error("***********************************************************".to_string());
    //################################################################################
    let peer_id = utils::new_peer_id();
    //std::thread::spawn(move || {
    //    // Start TUI loop here
    //    start_tui();
    //});

    //* parsing args (main thread)
    let args = Args::parse();
    // download directory checking
    info(format!("download directory: {}", args.download_dir));
    if !Path::new(&args.download_dir).exists() {
        error(format!("the provided directory does not exist"));
        std::process::exit(0);
    }

    // get torrent data torrent_file or magnet_url
    // these ones execute fast so no need for async + the data is required by all other components so no need for extra async logic and channels
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
        //TODO: here it is supposed to do the get_metadata from the peers we get from dht nodes
        todo!();
        Vec::new()
    };
    let bencode_data = Decoder::new(&res).start().unwrap();
    let torrent_data = Torrent::new(bencode_data, peer_id).unwrap();
    let torrent_data2 = torrent_data.clone();

    let (tx_peers, rx_peers) = mpsc::channel::<Peer>(128);

    // THESE 2 are async

    tokio::spawn(async move {
        let _ = get_peers_from_tracker(torrent_data, peer_id, tx_peers).await;
    });

    tokio::spawn(async move {
        let _ = download::start(torrent_data2, rx_peers, args.download_dir).await;
    });
    // start_tui();

    loop {}
    Ok(())
}

async fn get_peers_from_dht() -> Result<(), ()> {
    Ok(())
}
async fn get_peers_from_tracker(
    torrent_data: Torrent,
    peer_id: [u8; 20],
    tx_peers: Sender<Peer>,
) -> Result<(), ()> {
    tokio::spawn(async move {
        loop {
            let peers = tracker::get_peers(&torrent_data, &peer_id).await.unwrap();
            info(format!("tracker result: {:?}", peers));
            for peer in peers.peers {
                info(format!("sending tracker peer: {:?}", peer));
                tx_peers.send(peer).await.unwrap();
            }
            tokio::time::sleep(Duration::from_secs(peers.interval)).await;
        }
    });

    Ok(())
}
