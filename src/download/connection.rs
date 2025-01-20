use crate::constants::TIMEOUT_DURATION;
use crate::download::handshake;
use crate::torrent::Torrent;

use handshake::Handshake;
use std::io::{self, prelude::*};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

pub fn start(torrent: &Torrent, peer_index: usize) -> Result<TcpStream, String> {
    let handshake = Handshake::new(torrent.info_hash, torrent.peer_id).create_handshake();
    //println!("handshake: {:?}", handshake);
    match connect(torrent, handshake, peer_index) {
        Ok(tcp_stream) => Ok(tcp_stream),
        Err(err) => Err(err),
    }
}

fn connect(torrent: &Torrent, handshake: [u8; 68], peer_index: usize) -> Result<TcpStream, String> {
    //connect to tcp and send handshake
    let ip = format!(
        "{}:{}",
        torrent.peers[peer_index].ip, torrent.peers[peer_index].port
    );
    let mut stream = match TcpStream::connect_timeout(
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

    //read response
    //only reads 68 bytes of responnse // could cause problems i dont know for sure
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

    println!("\n-----------------------------------------");
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
