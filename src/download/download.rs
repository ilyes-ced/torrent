use crate::torrent::Torrent;

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
    //println!("number of pieces: {}", torrent.info.pieces.len());
    let mut pieces_workers: Vec<PieceWork> = Vec::new();
    for (ind, piece_hash) in torrent.info.pieces.iter().enumerate() {
        let piece_len = calc_piece_len(&torrent, ind, piece_hash);
        pieces_workers.push(PieceWork {
            index: ind,
            hash: *piece_hash,
            length: piece_len,
        })
    }

    // for loop and threads here to download each piece

    Ok(())
}

fn calc_piece_len(torrent: &Torrent, ind: usize, piece_hash: &[u8; 20]) -> usize {
    let begin = ind * torrent.info.piece_length as usize;
    let mut end = begin + torrent.info.piece_length as usize;
    if end > torrent.info.length.unwrap() as usize {
        end = torrent.info.length.unwrap() as usize
    }
    let res = end - begin;
    //println!("{} ===> {:?} ===> {}", ind, piece_hash, res);
    res
}
