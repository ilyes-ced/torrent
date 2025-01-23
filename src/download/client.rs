use crate::{constants::TIMEOUT_DURATION, peers::Peer, torrent::Torrent};
use std::{
    io::{self, Read, Write},
    net::{SocketAddr, TcpStream},
    time::Duration,
};

use super::{
    bitfield::Bitfield,
    handshake::{self, Handshake},
    message::{from_buf, Message},
};
#[derive(Debug)]

pub(crate) struct Client {
    con: TcpStream,
    choked: bool,
    bitfield: Bitfield,
    peer: Peer,
    infoHash: [u8; 20],
    peerID: [u8; 20],
}

impl Client {
    pub fn new(torrent: &Torrent, peer_index: usize) -> Result<Self, String> {
        let handshake = Handshake::new(torrent.info_hash, torrent.peer_id).create_handshake();
        let con = match connect(torrent, peer_index) {
            Ok(tcp_stream) => tcp_stream,
            Err(err) => return Err(err),
        };
        let con = match complete_handshake(con, torrent, handshake, peer_index) {
            Ok(tcp_stream) => tcp_stream,
            Err(err) => return Err(err),
        };
        let bitfield = match bitfield(&con) {
            Ok(msg) => msg,
            Err(err) => return Err(err),
        };

        Ok(Client {
            con,
            choked: true,
            bitfield: Bitfield::new(bitfield.payload),
            peer: torrent.peers[peer_index].clone(),
            infoHash: torrent.info_hash,
            peerID: torrent.peer_id,
        })
    }
}

pub fn connect(torrent: &Torrent, peer_index: usize) -> Result<TcpStream, String> {
    //connect to tcp and send handshake
    let stream = match TcpStream::connect_timeout(
        &SocketAddr::new(
            std::net::IpAddr::V4(torrent.peers[peer_index].ip),
            torrent.peers[peer_index].port,
        ),
        Duration::from_secs(TIMEOUT_DURATION),
    ) {
        Ok(con) => con,
        Err(e) => {
            if e.kind() == io::ErrorKind::TimedOut {
                return Err(String::from("connection operation timed out!"));
            } else {
                return Err(String::from(format!("network error occurred: {}", e)));
            }
        }
    };

    // not sure if read and write timeouts are needed
    let _ = match stream.set_read_timeout(Some(Duration::from_secs(TIMEOUT_DURATION))) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string()),
    };
    let _ = match stream.set_write_timeout(Some(Duration::from_secs(TIMEOUT_DURATION))) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string()),
    };

    // return the connection
    Ok(stream)
}

pub fn complete_handshake(
    mut stream: TcpStream,
    torrent: &Torrent,
    handshake: [u8; 68],
    peer_index: usize,
) -> Result<TcpStream, String> {
    // send/write handshake
    match stream.write(&handshake) {
        Ok(_) => {}
        Err(e) => {
            if e.kind() == io::ErrorKind::TimedOut {
                return Err(String::from("Write operation timed out!"));
            } else {
                return Err(String::from(format!("An error occurred: {}", e)));
            }
        }
    };

    // recieve/read response
    // only reads 68 bytes of responnse // could cause problems i dont know for sure
    let mut buffer = [0; 68];
    match stream.read(&mut buffer) {
        Ok(_) => {}
        Err(e) => {
            if e.kind() == io::ErrorKind::TimedOut {
                return Err(String::from("read operation timed out!"));
            } else {
                return Err(String::from(format!("An error occurred: {}", e)));
            }
        }
    };
    let response = buffer;
    let rec_handshake = match handshake::read_handshake(response) {
        Ok(handshake) => handshake,
        Err(_) => return Err(String::from("error recieving the handshake")),
    };

    println!("-----------------------------------------");
    println!(
    "recieved handshake: from peer: {:?} \n\tprotocol id:{:?} \n\tinfo hash:{:?} \n\tpeer id:{:?}",
    format!(
        "{}:{}",
        torrent.peers[peer_index].ip, torrent.peers[peer_index].port
    ),
    rec_handshake.protocol_id,
    rec_handshake.info_hash,
    String::from_utf8_lossy(&rec_handshake.peer_id).to_string()
);
    println!("-----------------------------------------");

    if rec_handshake.info_hash == torrent.info_hash {
        // seccuss continue the cmmunication
        println!("successfull handshake");
        Ok(stream)
    } else {
        //failure
        Err(String::from(
            "info hash recieved does not match your info hash",
        ))
    }
}

pub fn bitfield(con: &TcpStream) -> Result<Message, String> {
    let response = match from_buf(con) {
        Ok(msg) => msg.unwrap(),
        Err(err) => {
            return Err(String::from(format!(
                "error occured when getting bitfields message: {err}",
            )))
        }
    };
    println!("second read: recieving bitfields:  {:?}", response);
    Ok(response)
}
