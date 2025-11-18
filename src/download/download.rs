use tokio::sync::mpsc::{Receiver, Sender};

use crate::log::critical;
use crate::tracker::Peer;
use crate::ui::AppEvent;
use crate::utils::readable_size;
use crate::{
    client::Client,
    constants::{MsgId, MAX_BACKLOG, MAX_BLOCK_SIZE},
    io::reader::read_file,
    log::{debug, error, info, warning},
    torrent::{FileInfo, Torrent},
    utils::check_integrity,
};
use std::{sync::Arc, thread, time::Duration};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct PieceResult {
    pub index: u32,
    pub buf: Vec<u8>,
}
#[derive(Debug, Clone)]
pub struct PieceWork {
    pub index: u32,
    pub hash: [u8; 20],
    pub length: usize,
}

#[derive(Debug)]
pub struct PieceProgress<'a> {
    pub index: u32,
    pub buf: Vec<u8>,
    pub client: &'a mut Client,
    pub downloaded: usize,
    pub requested: usize,
    pub backlog: usize,
}

pub fn start_download(
    torrent: Torrent,
    download_dir: String,
    mut rx_clients: Receiver<Client>,
    tx_pieces: Sender<Option<(PieceResult, Peer)>>,
    tx_tui: &Sender<AppEvent>,
) {
    let tx_tui = tx_tui.clone();

    tokio::spawn(async move {
        info(
            "checking pre downloaded pieces, please be patient . . . .".to_string(),
            &tx_tui,
        )
        .await;

        let pieces = pieces_workers(&torrent);
        let all_pieces_len = pieces.len();

        // chack already downloaded pieces
        let already_downloaded = match read_file(&pieces, &torrent, &download_dir) {
            Ok(res) => res,
            Err(e) => {
                debug(
                    format!("reading file for already downloaded pieces: {}", e),
                    &tx_tui,
                )
                .await;
                Vec::new()
            }
        };

        info(
            format!("Already downloaded {} pieces", already_downloaded.len()),
            &tx_tui,
        )
        .await;
        info(
            format!(
                "Already downloaded : {:?}",
                readable_size(already_downloaded.len() as f64 * torrent.info.piece_length as f64)
            ),
            &tx_tui,
        )
        .await;

        // replace pieces by a new array (new_array = old_pieces_array.remove(already_downloaded))
        let pieces: Vec<PieceWork> = pieces
            .into_iter()
            .filter(|piece| !already_downloaded.contains(&piece.index))
            .collect();

        let num_pieces = Arc::new(all_pieces_len);
        let workers = Arc::new(RwLock::new(pieces));
        let results_counter = Arc::new(RwLock::new(already_downloaded.len()));
        let tx_pieces = Arc::new(tx_pieces);

        //? we add to progress the pieces already downloaded
        for piece in already_downloaded {
            let _ = tx_tui
                .send(AppEvent::PieceDownloaded {
                    index: piece,
                    peer: None,
                    size: 0,
                })
                .await;
        }

        while let Some(mut client) = rx_clients.recv().await {
            // error(format!("recieved client: {:?}", client), &tx_tui).await;

            let workers_clone = Arc::clone(&workers);
            let results_counter_clone = Arc::clone(&results_counter);
            let num_pieces_clone = Arc::clone(&num_pieces);
            let tx_pieces_clone = Arc::clone(&tx_pieces);

            let tx_tui_clone = tx_tui.clone();
            tokio::spawn(async move {
                warning(
                    format!("starting new download trhead for client {:?}", client.peer),
                    &tx_tui_clone,
                )
                .await;

                match init_client(&mut client) {
                    Ok(_) => {
                        // Create a new client for the peer
                        //? not sure about this blocking thing
                        //tokio::task::spawn_blocking(move || {});
                        let _ = client_download(
                            &mut client,
                            workers_clone,
                            results_counter_clone,
                            num_pieces_clone,
                            tx_pieces_clone,
                            &tx_tui_clone,
                        )
                        .await;
                    }
                    Err(e) => {
                        error(
                            format!("error occured in the download thread, innit client: {}", e),
                            &tx_tui_clone,
                        )
                        .await;
                    }
                };
            });
        }

        //client_download(
        //    &mut client,
        //    workers_clone,
        //    results_counter_clone,
        //    num_pieces_clone,
        //    tx_pieces_clone,
        //);

        // Display results
        let results_lock = results_counter.read().await;
        debug(format!("Results len(): {}", results_lock), &tx_tui).await;
        let workers_lock = workers.read().await;
        debug(format!("workers len(): {}", workers_lock.len()), &tx_tui).await;
    });
}

fn init_client(client: &mut Client) -> Result<(), String> {
    client.send_msg_id(MsgId::Unchoke, None)?;
    client.send_msg_id(MsgId::Interested, None)?;
    Ok(())
}

async fn client_download(
    client: &mut Client,
    workers: Arc<RwLock<Vec<PieceWork>>>,
    results_counter: Arc<RwLock<usize>>,
    num_pieces: Arc<usize>,
    tx_pieces: Arc<Sender<Option<(PieceResult, Peer)>>>,
    tx_tui: &Sender<AppEvent>,
) {
    info(format!("client {:?} thread starts", client.peer), tx_tui).await;

    loop {
        let results_counter_lock = results_counter.read().await;
        if *results_counter_lock == *num_pieces {
            info(
                format!(
                    "all pieces are downloaded | client {:?} is finished",
                    client.peer
                ),
                tx_tui,
            )
            .await;
            let _ = tx_pieces.send(None).await;
            break;
        }
        drop(results_counter_lock);

        // this loop was replaced by
        // if workers_count > 0 {}
        // if the count is 0 means other pieces are occupied take a break and retry
        // change it with
        let mut workers_lock = workers.write().await;
        if workers_lock.len() < 1 {
            debug(
                format!("number of workers left: {}", workers_lock.len()),
                tx_tui,
            )
            .await;
            tokio::time::sleep(Duration::from_millis(100)).await;
            continue;
        }

        let piece = workers_lock.remove(0);
        drop(workers_lock);

        if client.bitfield.has_piece(piece.index as usize) {
            info(
                format!("client {:?} has piece {}", client.peer, piece.index),
                tx_tui,
            )
            .await;

            match prepare_download(client, piece) {
                Ok(piece) => {
                    // todo: better debugging

                    let mut results_counter_lock = results_counter.write().await;
                    *results_counter_lock += 1;

                    //info(format!(
                    //    "download progress: {}/{} pieces {:.3}%",
                    //    *results_counter_lock,
                    //    *num_pieces,
                    //    (*results_counter_lock as f64 / *num_pieces as f64) * 100.0,
                    //));

                    let _ = tx_pieces
                        .send(Some((
                            PieceResult {
                                index: piece.index,
                                buf: piece.buf,
                            },
                            client.peer.clone(),
                        )))
                        .await;
                    // increment results_counter
                    drop(results_counter_lock);
                    // when downloading a piece reset error count
                    client.err_co = 0;
                }
                Err(err) => {
                    error(err.1.clone(), tx_tui).await;
                    if err.1 == "Resource temporarily unavailable (os error 11)"
                        || err.1 == "failed to fill whole buffer"
                        || err.1 == "Broken pipe (os error 32)"
                    {
                        client.err_co += 1;
                        warning(
                            format!("increased error counter for client {:?} ", client.peer),
                            tx_tui,
                        )
                        .await;
                        'outer: loop {
                            match client.restart_con(tx_tui).await {
                                Ok(_) => break 'outer,
                                Err(_) => {}
                            }
                        }
                        if client.err_co > 3 {
                            error(format!(
                                    "client {:?} restarted 3 times and it didnt work client will be dropped",
                                    client.peer
                                ),tx_tui).await;

                            // break client loop here (ending connection with the client)
                            break;
                        }
                    }

                    let mut workers_lock = workers.write().await;
                    // return this later
                    // workers_lock.insert(0, err.0);
                    workers_lock.push(err.0);
                    drop(workers_lock);
                }
            };
        } else {
            // put piece back in queue
            error(
                format!(
                    "client {:?} does not have piece {}",
                    client.peer, piece.index
                ),
                tx_tui,
            )
            .await;
            let mut workers_lock = workers.write().await;
            workers_lock.push(piece);
            drop(workers_lock);
            // todo:
            // to sleep this thread so others would take the piece (temporary solution)
            // maybe not temporary but there could be a better solution
            thread::sleep(std::time::Duration::from_millis(1000));
        }

        // if workers_count > 0 {}
        // else {
        //     // we dont end loop here because even tho the workers array is empty there could be one that is being processed
        //     drop(workers_lock);
        // }
    }
}

fn prepare_download(
    client: &mut Client,
    piece: PieceWork,
) -> Result<PieceResult, (PieceWork, String)> {
    let progress = PieceProgress {
        index: piece.index,
        buf: vec![0; piece.length],
        client,
        downloaded: 0,
        requested: 0,
        backlog: 0,
    };

    // download
    let piece_result = match download(progress, &piece) {
        Ok(piece) => piece,
        Err(err) => {
            return Err((piece, err));
        }
    };

    // check integrity
    if !check_integrity(&piece_result.buf, piece.hash).map_err(|e| (piece.clone(), e))? {
        return Err((piece, String::from("integrity check failed")));
    }

    Ok(PieceResult {
        index: piece.index,
        buf: piece_result.buf,
    })
}

fn download<'a>(
    mut progress: PieceProgress<'a>,
    piece: &'a PieceWork,
) -> Result<PieceProgress<'a>, String> {
    while progress.downloaded < piece.length {
        if !progress.client.choked {
            while progress.backlog < MAX_BACKLOG as usize && progress.requested < piece.length {
                let mut block_size = MAX_BLOCK_SIZE as usize;
                //* last block could be smalle than the rest so we change block size
                if (piece.length - progress.requested) < block_size {
                    block_size = piece.length - progress.requested
                }

                let mut payload: [u8; 12] = [0; 12];
                payload[0..4].copy_from_slice(&piece.index.to_be_bytes());
                payload[4..8].copy_from_slice(&(progress.requested as u32).to_be_bytes());
                payload[8..12].copy_from_slice(&(block_size as u32).to_be_bytes());

                // ! error handling
                progress
                    .client
                    .send_msg_id(MsgId::Request, Some(payload.to_vec()))
                    .map_err(|e| e.to_string())?;

                progress.backlog += 1;
                progress.requested += block_size;
            }
        }

        match progress.client.read_msg() {
            Ok(msg) => match msg.id {
                0 => progress.client.choked = true,
                1 => progress.client.choked = false,
                4 => {
                    // have
                    match msg.have() {
                        Ok(index) => {
                            progress.client.bitfield.set_piece(index as usize);
                        }
                        Err(_) => {
                            // maybe we shouldnt care about this error
                            // return Err(err)
                        }
                    }
                }
                7 => {
                    // piece
                    let (res_buf, buf_begin) = match msg.parse_piece(&progress) {
                        Ok(res) => res,
                        Err(err) => return Err(err),
                    };
                    progress.downloaded += res_buf.len();
                    for (i, u) in res_buf.iter().enumerate() {
                        progress.buf[(buf_begin as usize) + i] = *u;
                    }
                    progress.backlog -= 1;
                }
                6 => {
                    // TODO: implement seeding
                    // for request messages
                    // implement it to seed files
                }
                _ => {}
            },
            Err(err) => {
                if err != "keep alive signal" {
                    return Err(err);
                }
            }
        }
    }
    Ok(progress)
}

fn pieces_workers(torrent: &Torrent) -> Vec<PieceWork> {
    // gets all the pieces from the torrent file: (index, hash, lenght)
    let mut pieces_workers: Vec<PieceWork> = Vec::new();
    for (ind, piece_hash) in torrent.info.pieces.iter().enumerate() {
        let piece_len = calc_piece_len(torrent, ind);
        pieces_workers.push(PieceWork {
            // would only error if torrent has more than 2³²-1=4,294,967,296 pieces // kind of impossible
            index: ind.try_into().unwrap(),
            hash: *piece_hash,
            length: piece_len,
        })
    }
    pieces_workers
}

fn calc_piece_len(torrent: &Torrent, ind: usize) -> usize {
    let begin = ind * torrent.info.piece_length as usize;
    let mut end = begin + torrent.info.piece_length as usize;

    let files = match &torrent.info.files {
        FileInfo::Single(length) => {
            if end > *length as usize {
                end = *length as usize
            }
            end - begin
        }
        FileInfo::Multiple(files) => {
            let length: usize = files.iter().map(|s| s.length as usize).sum();
            if end > length {
                end = length
            }
            end - begin
        }
    };
    files
}
