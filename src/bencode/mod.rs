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
    Decoder::new(input).start()
}

// return something idk what yet
pub fn to_bencode(input: DecoderElement) -> Result<Vec<u8>, String> {
    // match here
    let result = Encoder::new(input).start().unwrap();
    Ok(result)
}
