use serde_json::Value;
use tokio::{net::UdpSocket, time::timeout};

use crate::{
    bencode::decoder::Decoder,
    dht::message::Response,
    log::{debug, error},
};
use std::{
    net::{Ipv4Addr, SocketAddr},
    time::Duration,
};

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

        self.socket
            .send_to(&msg, node_addr)
            .await
            .map_err(|e| format!("failed to send: {}", e))?;

        debug(format!(
            "message sent: {:?}, to {}",
            String::from_utf8_lossy(&msg).to_string(),
            node_addr
        ));

        //let (len, _node_addr) = self
        //    .socket
        //    .recv_from(&mut buf)
        //    .await
        //    .map_err(|e| format!("failed to recieve: {}", e))?;

        match timeout(Duration::from_secs(2), self.socket.recv_from(&mut buf)).await {
            Ok(Ok((len, _node_addr))) => {
                let res: Response = Response::decode_response(&buf[..len]).await?;
                return Ok(res);
            }
            Ok(Err(e)) => {
                return Err(format!("Socket error: {}", e));
            }
            Err(_) => {
                return Err("Timed out waiting for UDP packet.".to_string());
            }
        }
    }
}
