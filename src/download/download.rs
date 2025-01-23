use std::{io, io::Write, net::TcpStream};

use crate::{constants::MsgId, torrent::Torrent};

use super::message::{to_buf, Message};

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

pub fn start(torrent: Torrent) -> Result<(), String> {
    // for loop and threads here to download each piece
    let workers = pieces_workers(&torrent);

    // pieces_workers and pieces_results should be accessible from many threads

    for peer in torrent.peers {}

    Ok(())
}

fn pieces_workers(torrent: &Torrent) -> Vec<PieceWork> {
    //println!("number of pieces: {}", torrent.info.pieces.len());
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

// sends Messages of CHOKE/INTRESTED/..........
fn send_msg_id(mut con: TcpStream, signal: MsgId, payload: Vec<u8>) -> Result<(), String> {
    // signal is one of the constants
    let msg = Some(Message {
        id: signal.to_u8(),
        payload: payload,
    });
    match con.write(&to_buf(msg)) {
        Ok(_) => {}
        Err(e) => {
            if e.kind() == io::ErrorKind::TimedOut {
                return Err(String::from("Write operation timed out!"));
            } else {
                return Err(String::from(format!("An error occurred: {}", e)));
            }
        }
    };
    Ok(())
}
