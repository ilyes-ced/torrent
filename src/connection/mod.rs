use crate::peers::PeersResult;

mod handshake;

use handshake::Handshake;
use std::io::prelude::*;
use std::net::TcpStream;

pub fn start(peers: PeersResult, info_hash: [u8; 20], peer_id: [u8; 20]) -> Result<String, String> {
    let handshake = Handshake::new(info_hash, peer_id)
        .create_handshake()
        .unwrap();
    println!("handshake: {:?}", handshake);
    let response = connect(peers, handshake).unwrap();
    let rec_handshake = handshake::read_handshake(response).unwrap();

    println!("-----------------------------------------");
    println!(
        "recieved handshake: \n\tprotocol id:{:?} \n\tinfo hash:{:?} \n\tpeer id:{:?}",
        rec_handshake.protocol_id,
        rec_handshake.info_hash,
        String::from_utf8_lossy(&rec_handshake.peer_id).to_string()
    );

    if rec_handshake.info_hash == info_hash {
        // seccuss continue the cmmunication
    } else {
        //failure
    }

    Ok(String::from("handshake here"))
}

fn connect(peers: PeersResult, handshake: [u8; 68]) -> Result<[u8; 68], String> {
    //connect to tcp and send handshake
    let ip = format!("{}:{}", peers.peers[0].ip, peers.peers[0].port);
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
