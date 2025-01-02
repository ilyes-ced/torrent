use std::net::TcpStream;

use crate::peers::PeersResult;

struct Handshake {
    socket: TcpStream,
    buffer: Vec<u8>,
    peers_ip_addresses: Vec<String>,
}

impl Handshake {
    pub fn new(peers_ip_addresses: PeersResult) -> Result<Handshake, String> {
        Ok(Handshake {})
    }
}
