mod app;
mod bencode;
mod client;
mod constants;
// mod dht;
mod download;
mod io;
mod log;
mod magnet;
mod torrent;
mod tracker;
mod ui;
mod utils;

use clap::Parser;
// use dht::Dht;
use magnet::Magnet;
use std::path::Path;
use std::time::Duration;
use std::{fs::File, io::Read};
use tokio::sync::mpsc::{self, Sender};
use torrent::Torrent;

use tokio;

use crate::bencode::decoder::Decoder;
use crate::log::{error, info};
use crate::tracker::Peer;
use crate::ui::{
    start_tui, AppEvent,
    AppEvent::EventLog,
    LogType::{Error, Info},
};

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
    let (tx_tui, rx_tui) = mpsc::channel::<AppEvent>(128);
    let peer_id = utils::new_peer_id();

    //* parsing args (main thread)
    let args = Args::parse();
    // download directory checking
    // info(format!("download directory: {}", args.download_dir)).await;
    info(
        format!("download directory: {}", args.download_dir),
        &tx_tui,
    )
    .await;
    if !Path::new(&args.download_dir).exists() {
        //error(format!("the provided directory does not exist"));
        error("the provided directory does not exist".to_string(), &tx_tui).await;
        // todo: maybe replace this by telling the use to close the application, because it cant proceed
        std::process::exit(0);
    }

    // get torrent data torrent_file or magnet_url
    // these ones execute fast so no need for async + the data is required by all other components so no need for extra async logic and channels
    let res = if args.source.magnet_url == None {
        //info(format!(
        //    "starting download for torrent: {}",
        //    args.source.torrent_file.clone().unwrap()
        //)).await;

        info(
            format!(
                "starting download for torrent: {}",
                args.source.torrent_file.clone().unwrap()
            ),
            &tx_tui,
        )
        .await;

        let path = &args.source.torrent_file.unwrap();
        let mut file = File::open(path).map_err(|e| e.to_string()).unwrap();
        let mut buf = vec![];
        file.read_to_end(&mut buf)
            .map_err(|e| e.to_string())
            .unwrap();
        buf
    } else {
        let magnet_data = Magnet::new(&args.source.magnet_url.unwrap());
        // info(format!("magnet data: {:?}", magnet_data)).await;

        info(format!("magnet data: {:?}", magnet_data), &tx_tui).await;

        //TODO: here it is supposed to do the get_metadata from the peers we get from dht nodes
        todo!();
    };

    let bencode_data = Decoder::new(&res).start().unwrap();
    let torrent_data = Torrent::new(bencode_data, peer_id).unwrap();
    let torrent_data1 = torrent_data.clone();
    let torrent_data2 = torrent_data.clone();

    let (tx_peers, rx_peers) = mpsc::channel::<Peer>(128);

    let download_dir2 = args.download_dir.clone();
    let tx_tui1 = tx_tui.clone();

    tokio::spawn(async move {
        let _ = get_peers_from_tracker(torrent_data1, peer_id, tx_peers, tx_tui1.clone()).await;
    });

    tokio::spawn(async move {
        let _ = download::start(torrent_data2, rx_peers, args.download_dir, &tx_tui).await;
    });

    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });

    // this being first will block the rest
    // so we need to start everything below first maybe in a thread in another function
    start_tui(rx_tui, torrent_data, download_dir2, peer_id);

    Ok(())
}

async fn get_peers_from_tracker(
    torrent_data: Torrent,
    peer_id: [u8; 20],
    tx_peers: Sender<Peer>,
    tx_tui: Sender<AppEvent>,
) -> Result<(), ()> {
    tokio::spawn(async move {
        loop {
            let peers = tracker::get_peers(&torrent_data, &peer_id, tx_tui.clone())
                .await
                .unwrap();

            info(format!("tracker result: {:?}", peers), &tx_tui).await;

            for peer in peers.peers {
                tx_peers.send(peer).await.unwrap();
            }
            tokio::time::sleep(Duration::from_secs(peers.interval)).await;
            info(
                format!("tracker now sleeping for: {:?} seconds", peers.interval),
                &tx_tui,
            )
            .await;
        }
    });

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////

async fn get_peers_from_dht() -> Result<(), ()> {
    // infohash
    // 6fcf7ef136e73f0fb6186b30fe67d741cc260c5c
    //TODO: finish implementing DHT

    //////################################################################################
    ////* DHT protocol (DHT thread)
    //let mut dht = Dht::new().await.unwrap();
    //error("***********************************************************".to_string());
    //// error(format!("dht:: {:#?}", dht));
    //error("***********************************************************".to_string());
    //dht.bootstrap().await.unwrap();
    //dht.lookup().await.unwrap();
    //error("***********************************************************".to_string());
    ////error(format!("dht:: {:#?}", dht));
    //error("***********************************************************".to_string());
    ////################################################################################

    Ok(())
}
