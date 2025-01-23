use crate::{constants::MsgId, torrent::Torrent};

use super::Client;
use std::{
    sync::{Arc, Mutex},
    thread,
};
#[derive(Debug)]
struct PieceResult {
    index: usize,
    buf: Vec<u8>,
}
#[derive(Debug)]
struct PieceWork {
    index: usize,
    hash: [u8; 20],
    length: usize,
}
#[derive(Debug)]

struct pieceProgress {
    index: usize,
    buf: Vec<u8>,
    downloaded: usize,
    requested: usize,
    backlog: usize,
}

pub fn start(torrent: Torrent, clients: Vec<Client>) -> Result<(), String> {
    let workers_arc: Arc<Mutex<Vec<PieceWork>>> = Arc::new(Mutex::new(pieces_workers(&torrent)));
    let results_arc: Arc<Mutex<Vec<PieceResult>>> = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for mut client in clients {
        let _ = match client.send_msg_id(MsgId::UNCHOKE, None) {
            Err(err) => return Err(String::from(err)),
            _ => {}
        };
        let _ = match client.send_msg_id(MsgId::INTRESTED, None) {
            Err(err) => return Err(String::from(err)),
            _ => {}
        };

        let workers_clone = Arc::clone(&workers_arc);
        let results_clone = Arc::clone(&results_arc);

        let handle = thread::spawn(move || {
            // get element and process it
            loop {
                // break if all workers are finished
                let mut workers_lock = workers_clone.lock().unwrap();
                if workers_lock.is_empty() {
                    break;
                }
                // get a worker from the workers list
                let piece = workers_lock.remove(0);
                drop(workers_lock);

                // download pieces
                // processing here
                // summon download_piece(if
                //      it is Ok(resultPiece)) push into results
                //      if is Err(pieceWork) push it back into workers at the start
                println!(
                    "Client {:?} is using piece index: {}",
                    client.peer, piece.index
                );
                match download_piece(&client, piece) {
                    Ok(piece) => {
                        let mut results_lock = results_clone.lock().unwrap();
                        results_lock.push(PieceResult {
                            index: piece.index,
                            buf: Vec::new(),
                        });
                        drop(results_lock);
                    }
                    Err(piece) => {
                        let mut workers_lock = workers_clone.lock().unwrap();
                        workers_lock.insert(0, piece);
                        drop(workers_lock);
                    }
                };
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

fn download_piece(client: &Client, piece: PieceWork) -> Result<PieceResult, PieceWork> {
    let mut progress = pieceProgress {
        index: piece.index,
        buf: Vec::new(),
        downloaded: 0,
        requested: 0,
        backlog: 0,
    };

    // check availability on bitfield
    if !client.bitfield.has_piece(piece.index) {
        return Err(piece);
    }
    // downlaod
    let buf = if progress.downloaded < piece.length {
        [0];
    };

    // check integrity

    if piece.index < 2700 {
        Ok(PieceResult {
            index: 0,
            buf: vec![],
        })
    } else {
        Err(piece)
    }

    // client.bitfield.has_piece(piece.index)
}

fn pieces_workers(torrent: &Torrent) -> Vec<PieceWork> {
    // gets all the pieces from the torrent file: (index, hash, lenght)
    let mut pieces_workers: Vec<PieceWork> = Vec::new();
    for (ind, piece_hash) in torrent.info.pieces.iter().enumerate() {
        let piece_len = calc_piece_len(&torrent, ind);
        pieces_workers.push(PieceWork {
            index: ind,
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
