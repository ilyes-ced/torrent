// mapping to which files each piece belongs to

use crate::torrentfile::torrent::FileInfo::{Multiple, Single};
use crate::Torrent;
use std::cmp::{max, min};

#[derive(Debug)]
pub struct Mapping {
    pub file_index: usize,
    pub file_write_offset: u64,
    pub piece_write_len: u64,
}

pub fn mapping(torrent: &Torrent, piece_index: u32) -> Result<Vec<Mapping>, String> {
    let files = match &torrent.info.files {
        Multiple(files) => files,
        Single(_) => return Err(String::from("we cant accept single files here")),
        // should never happen
        // this was for figuring out the size of the last piece because it was needed for the final piece which is smaller than default piece size for reading already downloaded pieces

        // ugly solution to a small problem yes
        //Single(len) => &[Files {
        //    paths: [torrent.info.name.clone()].to_vec(),
        //    length: *len,
        //}]
        //.to_vec(),
    };

    let mut piece_to_file_mapping = Vec::new();
    let p_len = torrent.info.piece_length;
    let p_ind = piece_index as u64;

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
            let file_offset = max(piece_start, file_start);
            let length = min(piece_end, file_end) - file_offset;

            // file index, write offset in file, piece index, part of piece bounds (start, end)
            //piece_to_file_mapping.push((i, file_offset - file_start, p_ind, length));
            piece_to_file_mapping.push(Mapping {
                file_index: i,
                file_write_offset: file_offset - file_start,
                piece_write_len: length,
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
