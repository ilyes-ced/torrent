use crate::client::Client;
use crate::io::writer;
use crate::log::{error, info, warning};
use crate::torrentfile::torrent::Torrent;
use crate::tracker::Peer;

pub(crate) mod download;
use download::PieceResult;
use tokio::sync::mpsc::{self, Receiver};
use writer::write_file;

pub async fn start(
    torrent: Torrent,
    mut rx_peers: Receiver<Peer>,
    download_dir: String,
) -> Result<(), String> {
    info("starting download\n".to_string());
    let (tx_pieces, rx_pieces) = mpsc::channel::<(Option<PieceResult>, f64)>(128);
    let (tx_clients, rx_clients) = mpsc::channel::<Client>(128);

    //? starting the thread listinign for downloaded pieces
    // fix the cloning issue
    writer_listener(torrent.clone(), download_dir.clone(), rx_pieces);
    download::start_download(torrent.clone(), download_dir.clone(), rx_clients, tx_pieces);

    // start threads here for new clients
    while let Some(peer) = rx_peers.recv().await {
        let torrent = torrent.clone();
        let tx_clients = tx_clients.clone();

        tokio::spawn(async move {
            info(format!("starting client: {:?}", peer));
            let client = match get_client(&torrent, &peer) {
                Ok(client) => client,
                // kill the thread
                Err(err) => {
                    error(format!(
                        "connection with peer {:?} was dropped | cause: {}",
                        peer, err
                    ));
                    return;
                }
            };

            let res = tx_clients.send(client).await;
        });
    }

    Ok(())
}

fn get_client(torrent: &Torrent, peer: &Peer) -> Result<Client, String> {
    match Client::new(&torrent, peer) {
        Ok(client) => Ok(client),
        Err(err) => Err(format!(
            "connection with peer {:?} was dropped | cause: {}",
            peer, err
        )),
    }
}

fn writer_listener(
    torrent: Torrent,
    download_dir: String,
    mut rx_pieces: Receiver<(Option<PieceResult>, f64)>,
) {
    // here we write data to file
    tokio::spawn(async move {
        info("===================================================".to_string());
        info("===================================================".to_string());
        info("===================================================".to_string());
        info("===================================================".to_string());
        info("===================================================".to_string());
        info("===================================================".to_string());
        info("===================================================".to_string());
        info("===================================================".to_string());
        info("===================================================".to_string());
        info("===================================================".to_string());
        info("===================================================".to_string());
        info("===================================================".to_string());
        while let Some((piece, prog)) = rx_pieces.recv().await {
            info("===================================================".to_string());
            info("===================================================".to_string());
            info("===================================================".to_string());
            info("===================================================".to_string());
            info(format!(
                "*********** recieved pieces: {:?}, {}",
                piece.clone().unwrap().index,
                prog
            ));
            info("===================================================".to_string());
            info("===================================================".to_string());
            info("===================================================".to_string());
            info("===================================================".to_string());
            match piece {
                Some(finished_piece) => {
                    warning(format!(
                        "!!!!!!!!!!!!!!!!!!!!! recieved downloaded piece: {:?}",
                        finished_piece.index
                    ));
                    write_file(&torrent, finished_piece.clone(), &download_dir).unwrap();
                    info("-------------------------------------------".to_string());
                    info(format!(
                        "piece {} successfully downloaded",
                        finished_piece.index
                    ));
                    info(format!("download progress {:.3}%", prog));
                    info("-------------------------------------------".to_string());
                }
                None => {}
            }
        }
    });
}
