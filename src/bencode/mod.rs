//                               _     _
//               _ __     ___   | |_  (_)   ___    ___
//              | '_ \   / _ \  | __| | |  / __|  / _ \
//              | | | | | (_) | | |_  | | | (__  |  __/
//              |_| |_|  \___/   \__| |_|  \___|  \___|
//
// we go on with the encoder/decoder assuming then bencode always starts with the dict

pub mod decode;
pub mod encode;

use decode::{DecodeError, Decoder, DecoderElement};
use encode::{EncodeError, Encoder};

// set wheter it return string or byte arrays
// return something idk what yet
pub fn from_bencode(input: &[u8]) -> Result<DecoderElement, DecodeError> {
    Decoder::decode(input).start()
}

// return something idk what yet
pub fn to_bencode(input: DecoderElement) -> Result<Vec<u8>, String> {
    // match here
    let result = Encoder::new(input).start().unwrap();
    Ok(result)
}

///
///
///
///
///
///
///
///
// new torrent data structure
pub struct Torrent {
    announce: String,
    announce_list: String, // implement later
    comment: String,
    creation_date: u32,
    created_by: String,
    info: TorrentInfo,
}

pub struct TorrentInfo {
    name: String,
    pieces: String,
    piece_lenght: u64,
    //only if there is 1 file
    lenght: Option<u64>,
    // only used when there is more than 1 file
    files: Option<TorrentFile>,
}
pub struct TorrentFile {
    path: Vec<String>,
    lenght: u64,
}
