// implement reader here that allows resuming download
// it should be called from download::download // let pieces = pieces_workers(&torrent);

use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::Path,
};

use sha1::{Digest, Sha1};

use crate::{
    download::download::PieceWork,
    log::{debug, error, info, warning},
    torrentfile::torrent::{
        FileInfo::{Multiple, Single},
        Files, Torrent,
    },
};

// result vector of pieces already downloaded
// needs (pieces mapping, pieces hashes, files names)
// get piece from file and check its integrity with the pieces hashes
pub fn read_file(
    path: &String,
    pieces: &Vec<PieceWork>,
    torrent: &Torrent,
) -> Result<Vec<u32>, String> {
    match &torrent.info.files {
        Single(_) => check_piece_single_file(path, &pieces, torrent),
        Multiple(files) => check_piece_multi_file(files, path, &pieces, torrent),
    }
}

pub fn check_piece_single_file(
    path: &String,
    pieces: &Vec<PieceWork>,
    torrent: &Torrent,
) -> Result<Vec<u32>, String> {
    let mut downloaded: Vec<u32> = Vec::new();
    let file = get_file(&path).map_err(|e| e.to_string())?;
    match file {
        Some(mut file) => {
            for piece in pieces {
                if piece.index as u64 * torrent.info.piece_length
                    >= file.metadata().expect("unable to read metadata").len()
                {
                    // piece doesnt exist
                } else {
                    let start = piece.index as u64 * torrent.info.piece_length;
                    debug(format!(
                        "------------ {}, {:?}",
                        start,
                        file.metadata().expect("unable to read metadata").len()
                    ));
                    file.seek(SeekFrom::Start(start))
                        .map_err(|e| e.to_string())?;
                    let mut buf = vec![0; torrent.info.piece_length.try_into().unwrap()];
                    file.read_exact(&mut buf).map_err(|e| e.to_string())?;
                    // check buf integrity

                    let mut hasher = Sha1::new();
                    hasher.update(buf);
                    let hash = hasher.finalize();
                    if hash == piece.hash.into() {
                        debug(format!("------------ {:?}", piece.index));
                        downloaded.push(piece.index);
                    } else {
                        debug(format!("piece is invalid"));
                    }
                };
            }
        }
        None => {
            // file doesnt exist so no pieces are downloaded
            // empty array = no pieces are downloaded
            return Ok(Vec::new());
        }
    };
    Ok(downloaded)
}

pub fn check_piece_multi_file(
    files: &Vec<Files>,
    path: &String,
    pieces: &Vec<PieceWork>,
    torrent: &Torrent,
) -> Result<Vec<u32>, String> {
    // find files that have some ofthe piece
    // read the parts and concat them
    // read piece from file and check integrity with piece hash
    // piece index
    Err(String::from("not implemented yet"))
}

fn get_file(path: &str) -> Result<Option<File>, String> {
    error(format!("path: {:?}", path));

    if Path::new(path).exists() {
        info("file exists".to_string());
        Ok(Some(
            File::options()
                .read(true)
                .write(true)
                .open(path)
                .map_err(|e| e.to_string())?,
        ))
    } else {
        info("file does not exist.".to_string());
        Ok(None)
    }
}
