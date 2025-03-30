// implement reader here that allows resuming download
// it should be called from download::download // let pieces = pieces_workers(&torrent);

use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::{Path, PathBuf},
};

use super::mapping::mapping;

use crate::{
    download::download::PieceWork,
    torrentfile::torrent::{
        FileInfo::{Multiple, Single},
        Files, Torrent,
    },
    utils::check_integrity,
};

// result vector of piece indexes already downloaded
// needs (pieces mapping, pieces hashes, files names)
// get piece from file and check its integrity with the pieces hashes
pub fn read_file(
    pieces: &Vec<PieceWork>,
    torrent: &Torrent,
    download_dir: String,
) -> Result<Vec<u32>, String> {
    match &torrent.info.files {
        Single(_) => check_piece_single_file(&pieces, torrent, download_dir),
        Multiple(files) => check_piece_multi_file(files, &pieces, torrent, download_dir),
    }
}

pub fn check_piece_single_file(
    pieces: &Vec<PieceWork>,
    torrent: &Torrent,
    download_dir: String,
) -> Result<Vec<u32>, String> {
    let mut downloaded: Vec<u32> = Vec::new();
    let path = PathBuf::from(download_dir).join(&torrent.info.name);
    let file = get_file(path)?;

    match file {
        Some(file) => {
            for piece in pieces {
                let start = piece.index as u64 * torrent.info.piece_length;
                if let Ok(buf) = read_piece(start, torrent.info.piece_length, &file) {
                    if check_integrity(&buf, piece.hash)? {
                        downloaded.push(piece.index)
                    };
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
    pieces: &Vec<PieceWork>,
    torrent: &Torrent,
    download_dir: String,
) -> Result<Vec<u32>, String> {
    // find files that have some ofthe piece
    // read the parts and concat them
    // read piece from file and check integrity with piece hash
    // piece index
    let mut downloaded: Vec<u32> = Vec::new();
    for piece in pieces {
        let mappings = mapping(torrent, piece.index)?;

        if mappings.len() == 1 {
            let file_path = PathBuf::from(download_dir.clone())
                .join(&files[mappings[0].file_index].clone().paths.join("/"));
            let file = get_file(file_path)?;

            match file {
                Some(file) => {
                    let start = mappings[0].file_write_offset;
                    if let Ok(buf) = read_piece(start, mappings[0].piece_write_len, &file) {
                        if check_integrity(&buf, piece.hash)? {
                            downloaded.push(piece.index)
                        };
                    } else {
                    };
                }
                None => {}
            }
        }
        // for pieces that belong to multiple files
        else {
            let mut piece_buf = Vec::new();

            for mapping in mappings {
                let file_path = PathBuf::from(download_dir.clone())
                    .join(files[mapping.file_index].clone().paths.join("/"));

                let file = get_file(file_path)?;
                match file {
                    Some(file) => {
                        let start = mapping.file_write_offset;
                        if let Ok(mut buf) =
                            read_piece(start, mapping.piece_write_len.try_into().unwrap(), &file)
                        {
                            piece_buf.append(&mut buf)
                        };
                    }
                    None => {}
                }
            }

            if piece_buf.len() == torrent.info.piece_length as usize {
                // check buf integrity
                if check_integrity(&piece_buf, piece.hash)? {
                    downloaded.push(piece.index);
                }
            }
        }
    }

    Ok(downloaded)
}

pub fn get_file(path: PathBuf) -> Result<Option<File>, String> {
    if Path::new(&path).exists() {
        Ok(Some(
            File::options()
                .read(true)
                .open(path)
                .map_err(|e| e.to_string())?,
        ))
    } else {
        //debug("file does not exist.".to_string());
        Ok(None)
    }
}

fn read_piece(start: u64, piece_length: u64, mut file: &File) -> Result<Vec<u8>, String> {
    let file_len = file.metadata().expect("unable to read metadata").len();

    if start < file_len {
        file.seek(SeekFrom::Start(start))
            .map_err(|e| e.to_string())?;
        let mut buf = vec![0; piece_length.try_into().unwrap()];
        file.read_exact(&mut buf).map_err(|e| e.to_string())?;

        return Ok(buf);
    }
    Err(String::from("piece does not exist"))
}
