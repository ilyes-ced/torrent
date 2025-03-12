// implement reader here that allows resuming download
// it should be called from download::download // let pieces = pieces_workers(&torrent);

use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::Path,
};

use sha1::{Digest, Sha1};

use super::mapping::{mapping, Mapping};

use crate::{
    download::download::PieceWork,
    log::{debug, error, info},
    torrentfile::torrent::{
        FileInfo::{Multiple, Single},
        Files, Torrent,
    },
};

// result vector of piece indexes already downloaded
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
                    < file.metadata().expect("unable to read metadata").len()
                {
                    let start = piece.index as u64 * torrent.info.piece_length;
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
    let mut downloaded: Vec<u32> = Vec::new();
    for piece in pieces {
        let mappings = mapping(torrent, piece.index)?;
        debug(format!(
            "for piece: {}, mappings {:?}",
            piece.index, mappings
        ));

        if mappings.len() == 1 {
            // here we read piece as is
            let file_path = files[mappings[0].file_index].clone().paths.join("/");
            let file = get_file(&file_path).map_err(|e| e.to_string())?;
            match file {
                Some(mut file) => {
                    if piece.index as u64 * torrent.info.piece_length
                        < file.metadata().expect("unable to read metadata").len()
                    {
                        let start = piece.index as u64 * torrent.info.piece_length;
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
                None => {}
            }
        } else {
            // here we read parts of the piece from the files
            for mapping in mappings {}
            //
        }
    }

    Ok(downloaded)
}

fn get_file(path: &str) -> Result<Option<File>, String> {
    if Path::new(path).exists() {
        //debug("file exists".to_string());
        Ok(Some(
            File::options()
                .read(true)
                .write(true)
                .open(path)
                .map_err(|e| e.to_string())?,
        ))
    } else {
        //debug("file does not exist.".to_string());
        Ok(None)
    }
}
