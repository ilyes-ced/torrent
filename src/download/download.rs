use crate::{
    client::Client,
    constants::{MsgId, MAX_BACKLOG, MAX_BLOCK_SIZE},
    io::reader::read_file,
    log::{debug, error, info, warning},
    torrentfile::torrent::{FileInfo, Torrent},
    utils::check_integrity,
};
use sha1::{Digest, Sha1};
use std::sync::mpsc::Sender;
use std::{
    sync::{Arc, Mutex},
    thread,
};

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

pub fn start(
    torrent: Torrent,
    clients: Vec<Client>,
    tx: Sender<(Option<PieceResult>, f64)>,
    download_dir: String,
) -> Result<(), String> {
    info("checking pre downloaded pieces, please be patient . . . .".to_string());

    let pieces = pieces_workers(&torrent);
    let all_pieces_len = pieces.len();

    // chack already downloaded pieces
    let already_downloaded = read_file(&pieces, &torrent, download_dir)?;
    // replace pieces by a new array (new_array = old_pieces_array.remove(already_downloaded))
    let pieces: Vec<PieceWork> = pieces
        .into_iter()
        .filter(|piece| !already_downloaded.contains(&piece.index))
        .collect();

    let num_pieces = Arc::new(all_pieces_len);
    let workers = Arc::new(Mutex::new(pieces));
    let results_counter = Arc::new(Mutex::new(already_downloaded.len()));
    let tx = Arc::new(tx);

    let mut handles = vec![];

    for mut client in clients {
        init_client(&mut client)?;

        let workers_clone = Arc::clone(&workers);
        let results_counter_clone = Arc::clone(&results_counter);
        let num_pieces_clone = Arc::clone(&num_pieces);
        let tx_clone = Arc::clone(&tx);

        let handle = thread::spawn(move || {
            client_download(
                &mut client,
                workers_clone,
                results_counter_clone,
                num_pieces_clone,
                tx_clone,
            )
        });
        handles.push(handle);
    }

    for handle in handles {
        if let Err(e) = handle.join() {
            error(format!("Thread encountered an error: {:?}", e));
        }
    }

    // Display results
    let results_lock = results_counter.lock().unwrap();
    debug(format!("Results len(): {}", results_lock));
    let workers_lock = workers.lock().unwrap();
    debug(format!("workers len(): {}", workers_lock.len()));

    Ok(())
}

fn init_client(client: &mut Client) -> Result<(), String> {
    client.send_msg_id(MsgId::Unchoke, None)?;
    client.send_msg_id(MsgId::Interested, None)?;
    Ok(())
}

fn client_download(
    client: &mut Client,
    workers: Arc<Mutex<Vec<PieceWork>>>,
    results_counter: Arc<Mutex<usize>>,
    num_pieces: Arc<usize>,
    tx: Arc<Sender<(Option<PieceResult>, f64)>>,
) {
    info(format!("client {:?} thread starts", client.peer));

    loop {
        let mut workers_lock = workers.lock().expect("Failed to lock workers");
        let results_counter_lock = results_counter.lock().expect("Failed to lock results");

        if *results_counter_lock == *num_pieces {
            info(format!(
                "all pieces are downloaded | client {:?} is finished",
                client.peer
            ));
            let _ = tx.send((None, 100.00));
            break;
        }
        drop(results_counter_lock);

        if !workers_lock.is_empty() {
            debug(format!("number of workers left: {}", workers_lock.len()));

            let piece = workers_lock.remove(0);
            drop(workers_lock);

            if client.bitfield.has_piece(piece.index as usize) {
                info(format!(
                    "client {:?} has piece {}",
                    client.peer, piece.index
                ));

                match prepare_download(client, piece) {
                    Ok(piece) => {
                        // todo: better debugging

                        let mut results_counter_lock =
                            results_counter.lock().expect("Failed to lock results");
                        *results_counter_lock += 1;

                        //info(format!(
                        //    "download progress: {}/{} pieces {:.3}%",
                        //    *results_counter_lock,
                        //    *num_pieces,
                        //    (*results_counter_lock as f64 / *num_pieces as f64) * 100.0,
                        //));

                        let _ = tx.send((
                            Some(PieceResult {
                                index: piece.index,
                                buf: piece.buf,
                            }),
                            // total number / number done
                            (*results_counter_lock as f64 / *num_pieces as f64) * 100.0,
                        ));

                        // increment results_counter
                        drop(results_counter_lock);
                        // when downloading a piece reset error count
                        client.err_co = 0;
                    }
                    Err(err) => {
                        error(err.1.clone());
                        if err.1 == "Resource temporarily unavailable (os error 11)"
                            || err.1 == "failed to fill whole buffer"
                            || err.1 == "Broken pipe (os error 32)"
                        {
                            client.err_co += 1;
                            warning(format!(
                                "increased error counter for client {:?} ",
                                client.peer
                            ));
                            'outer: loop {
                                match client.restart_con() {
                                    Ok(_) => break 'outer,
                                    Err(_) => {}
                                }
                            }
                            if client.err_co > 3 {
                                error(
                                    "************************************************************"
                                        .to_string(),
                                );
                                error(format!(
                                    "client {:?} restarted 3 times and it didnt work client will be dropped",
                                    client.peer
                                ));
                                error(
                                    "************************************************************"
                                        .to_string(),
                                );
                                // break client loop here (ending connection with the client)
                                break;
                            }
                        }

                        let mut workers_lock = workers.lock().expect("Failed to lock workers");
                        // return this later
                        // workers_lock.insert(0, err.0);
                        workers_lock.push(err.0);
                        drop(workers_lock);
                    }
                };
            } else {
                // put piece back in queue
                error("client does not have piece".to_string());
                let mut workers_lock = workers.lock().expect("Failed to lock workers");
                workers_lock.push(piece);
                drop(workers_lock);
                // todo:
                // to sleep this thread so others would take the piece (temporary solution)
                // maybe not temporary but there could be a better solution
                thread::sleep(std::time::Duration::from_millis(1000));
            }
        } else {
            // we dont end loop here because even tho the workers array is empty there could be one that is being processed
            drop(workers_lock);
        }
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
