use std::io::prelude::*;
use std::net::TcpStream;


use crate::tracker::PeersResult;






pub fn build_handshake(peers: Vec<String>) -> std::io::Result<()> {
    Ok(())
}





pub fn download(peers: PeersResult) -> std::io::Result<()> {


    for peer in peers.ips {
        let mut stream = TcpStream::connect(peer)?;

        stream.write(&[1])?;
        stream.read(&mut [0; 128])?;
    }
    
    Ok(())
}