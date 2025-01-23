use crate::{constants::MsgId, torrent::Torrent};

use super::Client;
use std::sync::{Arc, Mutex};

struct PieceResult {
    index: usize,
    buf: Vec<u8>,
}

struct PieceWork {
    index: usize,
    hash: [u8; 20],
    length: usize,
}

// here we make threads to download each piece
// number of threads ios the number of pieces

pub fn start(torrent: Torrent, clients: Vec<Client>) -> Result<(), String> {
    let workers: Arc<Mutex<Vec<PieceWork>>> = Arc::new(Mutex::new(pieces_workers(&torrent)));
    let results: Arc<Mutex<Vec<PieceResult>>> = Arc::new(Mutex::new(Vec::new()));

    let test_works = pieces_workers(&torrent);
    for mut client in clients {
        println!("test");
        let _ = match client.send_msg_id(MsgId::UNCHOKE, None) {
            Ok(_) => {}
            Err(err) => return Err(String::from(err)),
        };
        let _ = match client.send_msg_id(MsgId::INTRESTED, None) {
            Ok(_) => {}
            Err(err) => return Err(String::from(err)),
        };

        for piece in test_works {
            println!("test::::::: {}", client.bitfield.has_piece(piece.index));
        }
        println!("end program");

        break;
    }

    Ok(())
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
