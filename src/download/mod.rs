use crate::client::Client;
use crate::io::writer;
use crate::log::{error, info, info_download};
use crate::torrent::Torrent;
use crate::tracker::Peer;
use crate::ui::AppEvent;

pub(crate) mod download;
use download::PieceResult;
use tokio::sync::mpsc::{self, Receiver, Sender};
use writer::write_file;

pub async fn start(
    torrent: Torrent,
    mut rx_peers: Receiver<Peer>,
    download_dir: String,
    tx_tui: &Sender<AppEvent>,
) -> Result<(), String> {
    info("starting download\n".to_string(), &tx_tui).await;
    let (tx_pieces, rx_pieces) = mpsc::channel::<Option<PieceResult>>(128);
    let (tx_clients, rx_clients) = mpsc::channel::<Client>(128);

    //? starting the thread listinign for downloaded pieces
    // fix the cloning issue
    writer_listener(torrent.clone(), download_dir.clone(), rx_pieces, tx_tui);
    download::start_download(
        torrent.clone(),
        download_dir.clone(),
        rx_clients,
        tx_pieces,
        tx_tui,
    );

    // start threads here for new clients
    while let Some(peer) = rx_peers.recv().await {
        let torrent = torrent.clone();
        let tx_clients = tx_clients.clone();

        let tx_tui_clone = tx_tui.clone();
        let _ = tx_tui;

        tokio::spawn(async move {
            info(format!("starting client: {:?}", peer), &tx_tui_clone).await;
            let client = match get_client(&torrent, &peer, &tx_tui_clone).await {
                Ok(client) => client,
                // kill the thread
                Err(err) => {
                    error(
                        format!(
                            "connection with peer {:?} was dropped | cause: {}",
                            peer, err
                        ),
                        &tx_tui_clone,
                    )
                    .await;
                    return;
                }
            };

            let _ = tx_clients.send(client).await;
        });
    }

    Ok(())
}

async fn get_client(
    torrent: &Torrent,
    peer: &Peer,
    tx_tui: &Sender<AppEvent>,
) -> Result<Client, String> {
    match Client::new(&torrent, peer, tx_tui).await {
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
    mut rx_pieces: Receiver<Option<PieceResult>>,
    tx_tui: &Sender<AppEvent>,
) {
    let tx_tui_clone = tx_tui.clone();
    let _ = tx_tui;

    // here we write data to file
    tokio::spawn(async move {
        while let Some(piece) = rx_pieces.recv().await {
            // error(
            //     format!("================================== recieved piece"),
            //     &tx_tui_clone.clone(),
            // )
            // .await;
            match piece {
                Some(finished_piece) => {
                    write_file(
                        &torrent,
                        finished_piece.clone(),
                        &download_dir,
                        &tx_tui_clone,
                    )
                    .await
                    .unwrap();

                    info(
                        format!("=============================================================================================================================="),
                        &tx_tui_clone.clone(),
                    )
                    .await;

                    info_download(
                        format!("piece {} successfully downloaded", finished_piece.index),
                        &tx_tui_clone.clone(),
                    )
                    .await;
                }
                None => {
                    break;
                }
            }
        }
        info_download(
            format!("Download finished. press <q> to exit the application"),
            &tx_tui_clone.clone(),
        )
        .await;
    });
}
