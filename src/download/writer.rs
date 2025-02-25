use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Read;
use std::io::SeekFrom;
use std::os::unix::fs::FileExt;
use std::path::Path;

use crate::log::warning;
use crate::log::{debug, error, info};
use crate::torrent::FileInfo::{Multiple, Single};
use crate::torrent::Torrent;

use super::download::PieceResult;

//todo:  needs cleaning up, too many calculations they need to be organized in variables

pub(crate) fn write_file(torrent: &Torrent, piece: PieceResult) -> Result<(), String> {
    let _ = match &torrent.info.files {
        Single(len) => write_single_file(torrent, piece),
        Multiple(files) => write_multi_file(torrent, piece),
    };
    Err(String::from("test"))
}

/*
    ? check if file already has the bytes inited
    ? if for example the file is empty and we want to write piece 2 at index 262144
    ? it would cause an error
    ? must init all pieces with zeros before writing piece 2
*/
pub(crate) fn write_single_file(torrent: &Torrent, piece: PieceResult) -> Result<(), String> {
    let ind = piece.index as u64;
    let file = get_file(&torrent.info.name)?;

    let file_len = file.metadata().unwrap().len();
    let piece_len = torrent.info.piece_length;

    if file_len < (ind * piece_len) {
        let num_blocks_to_fill = ind - (file_len / piece_len);

        warning(format!(
            "Adding {} blocks of size {}",
            num_blocks_to_fill, piece_len
        ));

        let zeros: Vec<u8> = vec![0; (num_blocks_to_fill * piece_len) as usize];

        file.write_at(&zeros, file_len).map_err(|e| e.to_string())?;
    }

    file.write_at(&piece.buf, ind * piece_len)
        .map_err(|err| err.to_string())?;

    Ok(())
}

fn write_multi_file(torrent: &Torrent, piece: PieceResult) -> Result<(), String> {
    // we have files in torrent and piece index we can calculate to which file or multiple files each pioece belongs

    let files = match &torrent.info.files {
        Multiple(files) => files,
        Single(_) => return Err(String::from("we cant accept single files here")), // should never happen
    };

    let p_len = torrent.info.piece_length;
    let p_ind = piece.index as u64;

    let mappings = mapping(torrent, &piece)?;

    debug(format!("mappings {:?}", mappings));

    for mapping in mappings {
        let ind = piece.index as u64;
        let file_path = files[mapping.file_index].clone().paths.join("/");
        let file = get_file(&file_path)?;

        let file_len = file.metadata().unwrap().len();
        let piece_len = torrent.info.piece_length;

        if file_len < (ind * piece_len) {
            let num_blocks_to_fill = ind - (file_len / piece_len);

            warning(format!(
                "Adding {} blocks of size {}",
                num_blocks_to_fill, piece_len
            ));

            let zeros: Vec<u8> = vec![0; changeme as usize];

            file.write_at(&zeros, file_len).map_err(|e| e.to_string())?;
        }

        file.write_at(&piece.buf, change_me)
            .map_err(|err| err.to_string())?;
    }

    std::process::exit(0);

    Ok(())
}

fn get_file(path: &str) -> Result<File, String> {
    if Path::new(path).exists() {
        info("file exists".to_string())
    } else {
        info("file does not exists. creating . . .".to_string());
        let test = File::create(path).unwrap();
    }

    let file = File::options()
        .read(true)
        .write(true)
        .open(path)
        .map_err(|e| e.to_string())?;

    Ok(file)
}

#[derive(Debug)]
struct Mapping {
    file_index: usize,
    file_write_offset: u64,
    piece_write_length: u64,
}

fn mapping(torrent: &Torrent, piece: &PieceResult) -> Result<Vec<Mapping>, String> {
    let files = match &torrent.info.files {
        Multiple(files) => files,
        Single(_) => return Err(String::from("we cant accept single files here")), // should never happen
    };

    let mut piece_to_file_mapping = Vec::new();
    let p_len = torrent.info.piece_length;
    let p_ind = piece.index as u64;

    let mut files_len: Vec<u64> = Vec::new();
    for file in files {
        files_len.push(file.length);
    }

    let piece_start = p_len * p_ind;
    let piece_end = p_len * p_ind + p_len;

    let mut cumulative_file_length: u64 = 0;
    for (i, file) in files.iter().enumerate() {
        //file bounds

        let file_start = cumulative_file_length;
        let file_end = cumulative_file_length + file.length;

        if piece_start < file_end && piece_end > file_start {
            let file_offset = std::cmp::max(piece_start, file_start);
            let length = std::cmp::min(piece_end, file_end) - file_offset;

            // file index, write offset in file, piece index, part of piece length
            //piece_to_file_mapping.push((i, file_offset - file_start, p_ind, length));
            piece_to_file_mapping.push(Mapping {
                file_index: i,
                file_write_offset: file_offset - file_start,
                piece_write_length: length,
            })
        }

        cumulative_file_length += file.length;
    }

    //info(format!("{:?}", piece_to_file_mapping.len()));
    //info(format!(
    //    "{}",
    //    piece_to_file_mapping
    //        .clone()
    //        .into_iter()
    //        .map(|(a, b, c, d)| format!("({}, {}, {}, {})", a, b, c, d))
    //        .collect::<Vec<_>>()
    //        .join("\n")
    //));
    Ok(piece_to_file_mapping)
}

//////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////

// todo: remove all prints and useless stuff
// todo: add ability to download pieces to text files and test with them because they are too large to put in github
#[cfg(test)]
mod tests {
    use std::fs;

    use crate::torrent;

    use super::*;
    #[test]
    fn it_works() {
        let torrent = Torrent {
            info: torrent::TorrentInfo {
                name: "POLEN23E.zip".to_string(),
                piece_length: 4190208,
                pieces: Default::default(),
                files: torrent::FileInfo::Single(34561190),
            },
            announce: Default::default(),
            announce_list: Default::default(),
            comment: Default::default(),
            creation_date: Default::default(),
            created_by: Default::default(),
            info_hash: Default::default(),
            peer_id: Default::default(),
        };

        let mut files = [
            (File::open("piece_8.txt").unwrap(), 8),
            (File::open("piece_4.txt").unwrap(), 4),
            (File::open("piece_7.txt").unwrap(), 7),
            (File::open("piece_3.txt").unwrap(), 3),
            (File::open("piece_6.txt").unwrap(), 6),
            (File::open("piece_0.txt").unwrap(), 0),
            (File::open("piece_1.txt").unwrap(), 1),
            (File::open("piece_2.txt").unwrap(), 2),
            (File::open("piece_5.txt").unwrap(), 5),
        ];

        for ind in 0..files.len() {
            let file_ind = files[ind].1;
            let mut file = &files[ind].0;

            let metadata = file.metadata().expect("unable to read metadata");

            info(format!(
                "in iteration: {ind}, for piece: {:?}, with length: {}, index: {}",
                fs::read_link(std::path::PathBuf::from(format!(
                    "/proc/self/fd/{}",
                    std::os::fd::AsRawFd::as_raw_fd(file)
                )))
                .unwrap(),
                metadata.len(),
                file_ind
            ));
            let mut buffer = vec![0; metadata.len() as usize];

            file.read(&mut buffer).expect("buffer overflow");
            //let res = write_single_file(&torrent, piece);
            //std::thread::sleep(Duration::from_secs(10));
        }
    }
}
