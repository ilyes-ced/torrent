use std::fs::File;
use std::io::prelude::*;

mod bencode;
mod download;
mod tracker;
mod utils;

use bencode::decode::{DecodeError, Decoder, DecoderElement, Pair};
use bencode::encode::{EncodeError, Encoder};

fn main() -> std::io::Result<()> {
    let path = "rar.torrent";
    let mut file = File::open(path)?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;

    //let gg = bencode::to_bencode(&buf).unwrap();
    //println!("\t{:#?}", gg);

    //// write the content to file
    //let mut file = File::create("test.txt")?;
    //write!(file, "{:?}", buf);

    //{
    //    if let DecoderElement::Dict(ele) = gg {
    //        println!("annound here: {}", ele[0].name);
    //        if let DecoderElement::String(string) = &ele[0].value {
    //            println!("url here: {}", String::from_utf8_lossy(&string).to_string());
    //        } else {
    //            println!("error: url not found");
    //        }
    //    } else {
    //        println!("error: invalid file structure");
    //    }
    //}

    //let gg = bencode::from_bencode(&buf).unwrap();
    //println!("{:?}", gg);
    //let gg = bencode::to_bencode(gg).unwrap();
    //println!("{:?}", gg);

    let mut peers = tracker::Peers::new(buf, file).unwrap();
    // match here
    let peers = peers.get_peers().unwrap();

    println!("list of peers {:?}", peers);
    println!("\x1b[93m////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////\x1b[0m");
    println!("\x1b[93m////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////\x1b[0m");
    println!("\x1b[93m////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////\x1b[0m");
    println!("\x1b[93m////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////\x1b[0m");
    println!("\x1b[93m////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////\x1b[0m");

    let _ = download::download(peers);

    Ok(())
}
