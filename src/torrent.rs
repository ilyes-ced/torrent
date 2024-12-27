use serde_json::Value;
use std::{error::Error, fmt, io::Bytes};

pub struct Torrent {
    announce: String,
    announce_list: Option<String>, // implement later
    comment: Option<String>,
    creation_date: Option<u32>,
    created_by: Option<String>,
    info: TorrentInfo,
}

pub struct TorrentInfo {
    name: String,
    pieces: Vec<[u8; 20]>,
    piece_length: u64,
    //only if there is 1 file
    length: Option<u64>,
    // only used when there is more than 1 file
    files: Option<Vec<TorrentFile>>,
}
pub struct TorrentFile {
    paths: Vec<String>,
    length: u64,
}

impl Torrent {
    pub fn new(data: String) -> Torrent {
        //println!("{}", data);

        let object: Value = serde_json::from_str(&data).unwrap();

        let result = extract_torrent(&object).unwrap();

        //println!("{}", object["info"]);
        println!("{}", result);

        result
    }
}

fn extract_torrent(value: &Value) -> Result<Torrent, String> {
    // Extracting fields from the JSON Value
    let announce = value["announce"]
        .as_str()
        .ok_or("Missing or invalid announce")?
        .to_string();

    let announce_list = value
        .get("announce list")
        .and_then(Value::as_str)
        .map(|s| s.to_string());

    let comment = value
        .get("comment")
        .and_then(Value::as_str)
        .map(|s| s.to_string());

    let creation_date = value["creation date"].as_u64().map(|n| n as u32);

    let created_by = value
        .get("created by")
        .and_then(Value::as_str)
        .map(|s| s.to_string());

    // Extracting info
    let info_value = &value["info"];

    let pieces: Vec<u8> = info_value["pieces"]
        .as_str()
        .ok_or("Missing or invalid pieces in info")?
        .split_whitespace()
        .map(|hex| u8::from_str_radix(hex, 16))
        .collect::<Result<_, _>>()
        .unwrap();

    let piece_hashes: Vec<[u8; 20]> = pieces
        .chunks(20)
        .map(|chunk| {
            let mut array = [0; 20];
            array[..chunk.len()].copy_from_slice(chunk);
            array
        })
        .collect();

    let info = TorrentInfo {
        name: info_value["name"]
            .as_str()
            .ok_or("Missing or invalid name in info")?
            .to_string(),
        pieces: piece_hashes,
        piece_length: info_value["piece length"]
            .as_u64()
            .ok_or("Missing or invalid piece_length in info")?,
        length: info_value.get("length").and_then(Value::as_u64),
        files: if let Some(files_value) = info_value.get("files").and_then(Value::as_array) {
            let files = files_value
                .iter()
                .map(|file_value| {
                    Ok(TorrentFile {
                        paths: file_value["path"]
                            .as_array()
                            .map(|paths| {
                                paths
                                    .iter()
                                    .filter_map(Value::as_str)
                                    .map(String::from)
                                    .collect()
                            })
                            .unwrap_or_default(),
                        length: file_value["length"]
                            .as_u64()
                            .ok_or("Missing or invalid length in file")?,
                    })
                })
                .collect::<Result<Vec<TorrentFile>, String>>()?;
            Some(files)
        } else {
            None
        },
    };

    Ok(Torrent {
        announce,
        announce_list,
        comment,
        creation_date,
        created_by,
        info,
    })
}

impl fmt::Display for Torrent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Torrent Information:\n\
                     Announce: {}\n\
                     Comment: {:?}\n\
                     Creation Date: {:?}\n\
                     Created By: {:?}\n\
                     Info:\n{}",
            self.announce, self.comment, self.creation_date, self.created_by, self.info
        ) // You may need to implement Display for TorrentInfo as well
    }
}

impl fmt::Display for TorrentInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let files_info = match &self.files {
            Some(files) => files
                .iter()
                .map(|file| file.to_string()) // Call Display on each TorrentFile
                .collect::<Vec<String>>()
                .join("\n"), // Join with new line
            None => "No files available".to_string(),
        };
        write!(
            f,
            "\tName: {}\n\
                     \tPieces: [{:?} . . . . {:?}] \n\
                     \tPiece length: {}\n\
                     \tlength: {:?}\n\
                     \tFiles: {:?}",
            self.name,
            self.pieces[0],
            self.pieces[self.pieces.len() - 1],
            self.piece_length,
            self.length,
            files_info
        )
    }
}

impl fmt::Display for TorrentFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Paths: {:?}, length: {}", self.paths, self.length)
    }
}
