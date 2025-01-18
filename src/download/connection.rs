use crate::peers::Peer;

use crate::download::handshake;
use crate::torrent::Torrent;

use handshake::Handshake;
use std::io::prelude::*;
use std::net::TcpStream;

pub fn start(torrent: Torrent) -> Result<String, String> {
    let handshake = Handshake::new(torrent.info_hash, torrent.peer_id)
        .create_handshake()
        .unwrap();
    println!("handshake: {:?}", handshake);
    let response = connect(torrent.peers, handshake).unwrap();
    let rec_handshake = handshake::read_handshake(response).unwrap();

    println!("-----------------------------------------");
    println!(
        "recieved handshake: \n\tprotocol id:{:?} \n\tinfo hash:{:?} \n\tpeer id:{:?}",
        rec_handshake.protocol_id,
        rec_handshake.info_hash,
        String::from_utf8_lossy(&rec_handshake.peer_id).to_string()
    );

    if rec_handshake.info_hash == torrent.info_hash {
        // seccuss continue the cmmunication
    } else {
        //failure
    }

    Ok(String::from("handshake here"))
}

fn connect(peers: Vec<Peer>, handshake: [u8; 68]) -> Result<[u8; 68], String> {
    //connect to tcp and send handshake
    let ip = format!("{}:{}", peers[0].ip, peers[0].port);
    let mut stream = TcpStream::connect(ip).unwrap();
    stream.write(&handshake).unwrap();

    //read response
    //only reads 68 bytes of responnse // could cause problems i dont know for sure
    let mut buffer = [0; 68];
    stream.read(&mut buffer).unwrap();
    let response = buffer;

    println!("Received: {:?}", response);
    Ok(response)
}
