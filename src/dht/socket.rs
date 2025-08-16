use serde_json::Value;
use tokio::net::UdpSocket;

use crate::{bencode::decoder::Decoder, dht::message::Response, log::debug};
use std::net::{Ipv4Addr, SocketAddr};

#[derive(Debug)]
pub struct Socket {
    pub socket: UdpSocket,
}

impl Socket {
    pub async fn new(addr: SocketAddr) -> Result<Self, String> {
        let socket = UdpSocket::bind(addr)
            .await
            .map_err(|e| format!("bind error: {}", e))?;
        Ok(Socket { socket })
    }

    pub async fn send(&self, msg: Vec<u8>, node_addr: SocketAddr) -> Result<Response, String> {
        let mut buf = [0; 1024];
        loop {
            self.socket
                .send_to(&msg, node_addr)
                .await
                .map_err(|e| format!("failed to send: {}", e))?;
            debug(format!(
                "message sent: {:?}",
                String::from_utf8_lossy(&msg).to_string()
            ));

            // todo: add timeout and repeat

            let (len, _node_addr) = self
                .socket
                .recv_from(&mut buf)
                .await
                .map_err(|e| format!("failed to recieve: {}", e))?;

            let res: Response = Response::decode_response(&buf[..len]).await?;

            return Ok(res);
        }
    }
}
