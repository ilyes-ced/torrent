use super::Client;
use crate::{
    constants::{MsgId, MAX_BACKLOG, MAX_BLOCK_SIZE},
    torrent::Torrent,
};
use sha1::{Digest, Sha1};
use std::{
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

pub struct pieceProgress<'a> {
    pub index: u32,
    pub buf: Vec<u8>,
    pub client: &'a mut Client,
    pub downloaded: usize,
    pub requested: usize,
    pub backlog: usize,
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
                // summon prepare_download(if
                //      it is Ok(resultPiece)) push into results
                //      if is Err(pieceWork) push it back into workers at the start
                println!(
                    "Client {:?} is using piece index: {}",
                    client.peer, piece.index
                );
                //println!("--- client {:?} is  unchoked", client.peer);
                //thread::sleep(std::time::Duration::from_secs(1));

                //

                //println!("--********////////////////");
                //println!("{:?}", client);
                //println!("--********////////////////");
                //std::process::exit(0);
                match prepare_download(&mut client, piece) {
                    Ok(piece) => {
                        let mut results_lock = results_clone.lock().unwrap();
                        results_lock.push(PieceResult {
                            index: piece.index,
                            buf: Vec::new(),
                        });
                        drop(results_lock);
                    }
                    Err(err) => {
                        println!("/** {}", err.1);
                        let mut workers_lock = workers_clone.lock().unwrap();
                        // return this later
                        // workers_lock.insert(0, err.0);
                        workers_lock.push(err.0);
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

fn prepare_download(
    client: &mut Client,
    piece: PieceWork,
) -> Result<PieceResult, (PieceWork, String)> {
    let progress = pieceProgress {
        index: piece.index,
        buf: Vec::new(),
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
    if !progress.client.bitfield.has_piece(piece.index as usize) {
        return Err((piece, String::from("client does not have this piece")));
    }

    // download
    let piece_result = match download(progress, &piece) {
        Ok(piece) => piece,
        Err(err) => return Err((piece, err)),
    };

    // check integrity
    let mut hasher = Sha1::new();
    hasher.update(&piece_result.buf);
    let hash = hasher.finalize();

    println!("---------------------");
    println!("{:?}", hash);
    println!("{:?}", piece.hash);
    println!("---------------------");

    if !(hash == piece.hash.into()) {
        return Err((piece, String::from("integrity check failed")));
    }

    // hasher.update(info_binary.clone());
    // let result = hasher.finalize();

    println!(
        "--------------------- completed download of piece {} ---------------------",
        piece.index
    );
    Ok(PieceResult {
        index: piece.index,
        buf: piece_result.buf,
    })

    // client.bitfield.has_piece(piece.index)
}

fn download<'a>(
    mut progress: pieceProgress<'a>,
    piece: &'a PieceWork,
) -> Result<pieceProgress<'a>, String> {
    while progress.downloaded < piece.length {
        // * if client is unchoked
        // *       send request for the piece
        // * else
        // *       attempt unchoke request
        if !progress.client.choked {
            //downlaod logic here
            while progress.backlog < MAX_BACKLOG as usize || progress.requested < piece.length {
                let mut block_size = MAX_BLOCK_SIZE as usize;
                //* last block could be smalle than the rest so we change block size
                if piece.length - progress.requested < block_size {
                    block_size = piece.length - progress.requested
                }
                // this one to be fixed
                // sed request Msg
                let mut payload: [u8; 12] = [0; 12];
                // add index
                println!("--------------------{}", progress.requested);
                payload[0..4].copy_from_slice(&piece.index.to_be_bytes());
                payload[4..8].copy_from_slice(&(progress.requested as u32).to_be_bytes());
                payload[8..12].copy_from_slice(&(block_size as u32).to_be_bytes());
                // ! error handling
                let _ = progress
                    .client
                    .send_msg_id(MsgId::HAVE, Some(payload.to_vec()))
                    .map_err(|e| e.to_string())?;
                progress.backlog += 1;
                progress.requested += block_size;
                println!("--------------------{}", progress.requested);
            }
        }

        //let _ = progress.client.send_msg_id(MsgId::UNCHOKE, None);

        let msg = match progress.client.read_msg() {
            Ok(msg) => match msg.id {
                0 => progress.client.choked = true,
                1 => progress.client.choked = false,
                4 => {
                    let piece_index = match msg.have() {
                        Ok(ind) => ind,
                        Err(err) => return Err(err),
                    };
                    println!("***************************************************** bitfield is being set at {}", piece_index);
                    progress.client.bitfield.set_piece(piece_index as usize);
                }
                7 => {
                    let vec = match msg.parse_piece(&progress) {
                        Ok(res) => res,
                        Err(err) => return Err(err),
                    };
                    progress.buf.extend_from_slice(&vec);
                    progress.downloaded += vec.len();
                    progress.backlog -= 1;
                }
                _ => {}
            },
            Err(err) => {
                if err != "keep alive signal" {
                    println!("{}", err)
                }
            }
        };
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
