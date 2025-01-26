use super::Client;
use crate::{
    constants::{MsgId, MAX_BACKLOG, MAX_BLOCK_SIZE},
    peers::Peer,
    torrent::Torrent,
};
use sha1::{Digest, Sha1};
use std::{
    os::unix::process,
    sync::{Arc, Mutex},
    thread,
};

#[derive(Debug)]
struct PieceResult {
    index: u32,
    buf: Vec<u8>,
}
#[derive(Debug)]
struct PieceWork {
    index: u32,
    hash: [u8; 20],
    length: usize,
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

pub fn start(torrent: Torrent, mut clients: Vec<Client>) -> Result<(), String> {
    let pieces = pieces_workers(&torrent);
    let num_pieces = pieces.len();
    let workers_arc: Arc<Mutex<Vec<PieceWork>>> = Arc::new(Mutex::new(pieces));
    let results_arc: Arc<Mutex<Vec<PieceResult>>> = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for mut client in &clients {
        println!("{:?} => {:?}", client.peer, client.bitfield)
    }

    thread::sleep(std::time::Duration::from_millis(2000));

    //let mut client = clients.remove(0);
    //println!("{:?} => {:?}", client.peer, client.bitfield);
    //thread::sleep(std::time::Duration::from_millis(1000));

    for mut client in clients {
        client.send_msg_id(MsgId::UNCHOKE, None)?;
        client.send_msg_id(MsgId::INTRESTED, None)?;

        let workers_clone = Arc::clone(&workers_arc);
        let results_clone = Arc::clone(&results_arc);

        let handle = thread::spawn(move || {
            println!("----- client {:?} thread starts", client.peer);
            // take a piece and process it
            loop {
                let mut workers_lock = workers_clone.lock().expect("Failed to lock workers");
                let results_lock = results_clone.lock().expect("Failed to lock results");
                if results_lock.len() == num_pieces {
                    println!("6666666666666 all pieces are downloaded");
                    break;
                }
                drop(results_lock);
                if !workers_lock.is_empty() {
                    let piece = workers_lock.remove(0);
                    drop(workers_lock);

                    //println!(
                    //    "Client {:?} is using piece index: {}",
                    //    client.peer, piece.index
                    //);

                    let results_lock = results_clone.lock().expect("Failed to lock results");
                    //println!(
                    //    "--********//////////////// --> resutls : {}",
                    //    results_lock.len()
                    //);
                    drop(results_lock);
                    if client.bitfield.has_piece(piece.index as usize) {
                        println!("??????? client has piece",);
                        match prepare_download(&mut client, piece) {
                            Ok(piece) => {
                                // here the result needs to be written to file so it doesnt consume alot of ram
                                let mut results_lock =
                                    results_clone.lock().expect("Failed to lock results");
                                results_lock.push(PieceResult {
                                    index: piece.index,
                                    buf: Vec::new(),
                                });
                                drop(results_lock);
                            }
                            Err(err) => {
                                println!("||||||||||||||||||||||||||||||:: {}", err.1);
                                let mut workers_lock =
                                    workers_clone.lock().expect("Failed to lock workers");
                                // return this later
                                // workers_lock.insert(0, err.0);
                                workers_lock.push(err.0);
                                drop(workers_lock);
                            }
                        };
                    } else {
                        // put piece back in queue
                        println!("...... client does not have piece",);
                        let mut workers_lock =
                            workers_clone.lock().expect("Failed to lock workers");
                        workers_lock.push(piece);
                        drop(workers_lock);
                        thread::sleep(std::time::Duration::from_millis(1000));
                    }
                } else {
                    drop(workers_lock);
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        if let Err(e) = handle.join() {
            eprintln!("Thread encountered an error: {:?}", e);
        }
    }

    // Display results
    let results_lock = results_arc.lock().unwrap();
    println!("Results len(): {}", results_lock.len());
    let workers_lock = workers_arc.lock().unwrap();
    println!("workers len(): {}", workers_lock.len());

    Ok(())
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

    // check availability on bitfield

    //println!("{:?}", progress);
    //println!("******************");
    //println!(
    //    "{} \n {}",
    //    progress.client.bitfield,
    //    progress.client.bitfield.has_piece(piece.index as usize)
    //);
    //println!("******************");
    // bad/needs change because when a client doesnt have a piece we keep asking him for it
    //if !progress.client.bitfield.has_piece(piece.index as usize) {
    //    return Err((piece, String::from("client does not have this piece")));
    //}

    //// download
    let piece_result = match download(progress, &piece) {
        Ok(piece) => piece,
        Err(err) => {
            return Err((piece, err));
        }
    };

    // check integrity
    //let mut hasher = Sha1::new();
    //hasher.update(&piece_result.buf);
    //let hash = hasher.finalize();
    //
    //println!("---------------------");
    //println!("{:?}", hash);
    //println!("{:?}", piece.hash);
    //println!("---------------------");
    //
    //if !(hash == piece.hash.into()) {
    //    return Err((piece, String::from("integrity check failed")));
    //}
    //
    //println!(
    //    "--------------------- completed download of piece {} ---------------------",
    //    piece.index
    //);

    println!(
        "222222222222222222222222222222222222222222222222222222 {:?} ",
        PieceResult {
            index: piece.index,
            buf: [].to_vec(),
        }
    );

    //std::process::exit(0);

    Ok(PieceResult {
        index: piece.index,
        buf: [].to_vec(),
    })

    // client.bitfield.has_piece(piece.index)
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
                let _ = progress
                    .client
                    .send_msg_id(MsgId::REQUEST, Some(payload.to_vec()))
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
                    //match msg.have() {
                    //    Ok(index) => {
                    //        println!("***************************************************** bitfield is being set at {}", index);
                    //        progress.client.bitfield.set_piece(index as usize);
                    //    }
                    //    Err(_) => {
                    //        // maybe we shouldnt care about this error
                    //        // return Err(err)
                    //    }
                    //}
                }
                7 => {
                    // piece
                    let (res_buf, buf_begin) = match msg.parse_piece(&progress) {
                        Ok(res) => res,
                        Err(err) => return Err(err),
                    };

                    progress.downloaded += res_buf.len();
                    progress.buf.splice((buf_begin as usize).., res_buf);
                    progress.backlog -= 1;
                }
                _ => {}
            },
            Err(err) => {
                if err != "keep alive signal" {
                    println!("{}", err)
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
        let piece_len = calc_piece_len(&torrent, ind);
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
    if end > torrent.info.length.unwrap() as usize {
        end = torrent.info.length.unwrap() as usize
    }
    let res = end - begin;
    //println!("{} ===> {:?} ===> {}", ind, piece_hash, res);
    res
}
