use std::net::TcpStream;

use crate::peers::{Peer, PeersResult};

pub struct Handshake {
    pub protocol_id: String,
    pub info_hash: [u8; 20],
    pub peer_id: [u8; 20],
}

impl Handshake {
    pub fn new(info_hash: [u8; 20], peer_id: [u8; 20]) -> Handshake {
        Handshake {
            protocol_id: String::from("BitTorrent protocol"),
            info_hash: info_hash,
            peer_id: peer_id,
        }
    }
    pub fn create_handshake(&self) -> Result<[u8; 68], String> {
        let mut buffer: [u8; 68] = [0; 68];
        buffer[0..1].copy_from_slice(&[19]);
        buffer[1..20].copy_from_slice("BitTorrent protocol".as_bytes());
        buffer[20..28].copy_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]);
        buffer[28..48].copy_from_slice(&self.info_hash);
        buffer[48..68].copy_from_slice(&self.peer_id);
        Ok(buffer)
    }
}

// to be implemented
pub fn read_handshake(handshake: [u8; 68]) -> Result<Handshake, String> {
    Ok(Handshake {
        protocol_id: String::from_utf8_lossy(&handshake[1..20]).to_string(),
        info_hash: handshake[28..48].try_into().unwrap(),
        peer_id: handshake[48..68].try_into().unwrap(),
    })
}
