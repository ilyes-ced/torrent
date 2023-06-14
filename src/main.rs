use std::fs::File;
use std::io::prelude::*;

mod bencode;
mod tracker;

use bencode::decode::{DecodeError, Decoder, DecoderElement, Pair};
use bencode::encode::{EncodeError, Encoder, EncoderElement};

fn main() -> std::io::Result<()> {
    let path = "1669901338-Satisfactory.v0.6.1.5.Early.Access(1).torrent";
    let mut file = File::open(path)?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;

    let gg = bencode::to_bencode(&buf).unwrap();

    // write the content to file
    //let mut file = File::create("test.txt")?;
    //write!(file, "{:?}", gg);

    {
        if let DecoderElement::Dict(ele) = gg {
            println!("annound here: {}", ele[0].name);
            if let DecoderElement::String(string) = &ele[0].value {
                println!("url here: {}", String::from_utf8_lossy(&string).to_string());
            } else {
                println!("error: url not found");
            }
        } else {
            println!("error: invalid file structure");
        }
    }
    let _ = tracker::get_peers();

    Ok(())
}

fn connect() {}

fn get_peers() {}
