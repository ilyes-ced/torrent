use crate::{bencode::DecoderResults, log::info};
use serde_json::Value;
use std::fmt;

#[derive(Clone)]
pub enum FileInfo {
    // single file: length
    Single(u64),
    // multiple files: array[Dir:[path1,path2....]+length]
    Multiple(Vec<Files>),
}

#[derive(Clone)]
pub struct Torrent {
    pub announce: String,
    pub announce_list: Option<String>, // implement later
    pub comment: Option<String>,
    pub creation_date: Option<u32>,
    pub created_by: Option<String>,
    pub info: TorrentInfo,
    pub info_hash: [u8; 20],
    pub peer_id: [u8; 20],
}

#[derive(Clone)]
pub struct TorrentInfo {
    pub name: String,
    pub pieces: Vec<[u8; 20]>,
    pub piece_length: u64,
    //only if there is 1 file
    // pub length: Option<u64>,
    //only used when there is more than 1 file
    // pub files: Option<Vec<TorrentFile>>,
    pub files: FileInfo,
}

#[derive(Clone)]
pub struct Files {
    pub paths: Vec<String>,
    pub length: u64,
}

impl Torrent {
    pub fn new(data: DecoderResults, peer_id: [u8; 20]) -> Result<Torrent, String> {
        let json_object: Value = serde_json::from_str(&data.result).unwrap();
        let result = extract_torrent_data(&json_object, data.info_hash, peer_id).unwrap();

        info(format!("{}", result));

        Ok(result)
    }
}

fn extract_torrent_data(
    json_object: &Value,
    info_hash: [u8; 20],
    peer_id: [u8; 20],
) -> Result<Torrent, String> {
    // Extracting fields from the JSON Value
    let announce = json_object["announce"]
        .as_str()
        .ok_or("Missing or invalid announce")?
        .to_string();

    let announce_list = json_object
        .get("announce list")
        .and_then(Value::as_str)
        .map(|s| s.to_string());

    let comment = json_object
        .get("comment")
        .and_then(Value::as_str)
        .map(|s| s.to_string());

    let creation_date = json_object["creation date"].as_u64().map(|n| n as u32);

    let created_by = json_object
        .get("created by")
        .and_then(Value::as_str)
        .map(|s| s.to_string());

    // Extracting info
    let info_value = &json_object["info"];

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
        files: if let Some(elements) = info_value.get("files").and_then(Value::as_array) {
            let test: Vec<Files> = elements
                .iter()
                .map(|file_value| {
                    Ok(Files {
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
                .collect::<Result<Vec<Files>, String>>()
                .unwrap();

            FileInfo::Multiple(test)
        } else {
            // todo: handle error
            FileInfo::Single(info_value.get("length").and_then(Value::as_u64).unwrap())
        },
    };

    Ok(Torrent {
        announce,
        announce_list,
        comment,
        creation_date,
        created_by,
        info,
        info_hash,
        peer_id,
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
                     info hash: {:?}--{:?}\n\
                     Info:\n{}",
            self.announce,
            self.comment,
            self.creation_date,
            self.created_by,
            self.info_hash
                .iter()
                .map(|b| format!("{:02x}", b)) // Format each byte as two-digit hex
                .collect::<Vec<String>>() // Collect into a vector of strings
                .join(" "),
            self.info_hash
                .iter()
                .map(|b| format!("{}", b)) // Format each byte as two-digit hex
                .collect::<Vec<String>>() // Collect into a vector of strings
                .join(" "),
            self.info,
        ) // You may need to implement Display for TorrentInfo as well
    }
}

impl fmt::Display for TorrentInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let files_info = match &self.files {
            FileInfo::Single(length) => format!("Single file: length: {}", length.to_string()),
            FileInfo::Multiple(files) => {
                let mut names = String::new();
                for file in files {
                    names.push_str(&format!(
                        "length:{}, file: \n\t\t",
                        file.length.to_string().as_str()
                    ));
                    for path in &file.paths {
                        names.push_str(path);
                        names.push_str("/");
                    }
                    names.pop();
                    names.push('\n');
                    names.push('\t');
                    names.push('\t');
                }
                names
            }
        };
        write!(
            f,
            "\tName: {}\n\
                     \tPieces: [{:?} . . . . {:?}] \n\
                     \tPiece length: {}\n\
                     \tFiles: \n\t\t{}",
            self.name,
            self.pieces[0],
            self.pieces[self.pieces.len() - 1],
            self.piece_length,
            files_info
        )
    }
}

impl fmt::Display for Files {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Paths: {:?}, length: {}", self.paths, self.length)
    }
}
