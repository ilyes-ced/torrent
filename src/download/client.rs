use crate::{
    constants::{MsgId, TIMEOUT_DURATION},
    log::{debug, error, info},
    peers::Peer,
    torrent::Torrent,
};
use std::{
    io::{self, Read, Write},
    net::{SocketAddr, TcpStream},
    time::Duration,
};

use super::{
    bitfield::Bitfield,
    handshake::{self, Handshake},
    message::{from_buf, to_buf, Message},
};

#[derive(Debug)]
pub(crate) struct Client {
    pub con: TcpStream,
    pub choked: bool,
    pub bitfield: Bitfield,
    pub peer: Peer,
    pub handshake: [u8; 68],
    pub info_hash: [u8; 20],
}

impl Client {
    pub fn new(torrent: &Torrent, peer: &Peer) -> Result<Self, String> {
        let handshake = Handshake::new(torrent.info_hash, torrent.peer_id).create_handshake();

        let con = connect(peer, torrent.info_hash, handshake)?;

        let bitfield = match bitfield(&con) {
            Ok(msg) => msg,
            Err(err) => return Err(err),
        };

        Ok(Client {
            con,
            choked: true,
            bitfield: Bitfield::new(bitfield.payload),
            peer: peer.clone(),
            handshake,
            info_hash: torrent.info_hash,
        })
    }

    pub fn restart_con(&mut self) -> Result<(), String> {
        debug(format!("restarting connection with peer: {:?}", self.peer));

        if let Err(e) = self.con.shutdown(std::net::Shutdown::Both) {
            match e.kind() {
                io::ErrorKind::NotConnected => {
                    // Already disconnected.  That's fine.
                    error(format!("peer {:?} is already disconnected", self.peer));
                }
                _ => {
                    error(format!("Error shutting down connection: {}", e));
                }
            }
        }
        //  self.con.shutdown(std::net::Shutdown::Both).unwrap();

        let con = connect(&self.peer, self.info_hash, self.handshake).unwrap();

        match bitfield(&con) {
            Ok(msg) => self.bitfield = Bitfield::new(msg.payload),
            Err(err) => return Err(err),
        };
        debug(format!("restarted connection with peer: {:?}", self.peer));
        Ok(())
    }

    // sends Messages of CHOKE/INTRESTED/REQUEST/.../...
    pub fn send_msg_id(&mut self, signal: MsgId, payload: Option<Vec<u8>>) -> Result<(), String> {
        // signal is one of the constants in MsgId
        let msg = Some(Message {
            id: signal.to_u8(),
            payload: payload.unwrap_or_else(Vec::new),
        });
        match self.con.write_all(&to_buf(msg)) {
            Ok(_) => {}
            Err(e) => {
                if e.kind() == io::ErrorKind::TimedOut {
                    return Err(String::from("Write operation timed out!"));
                } else {
                    return Err(e.to_string());
                }
            }
        };
        Ok(())
    }

    pub fn read_msg(&mut self) -> Result<Message, String> {
        match from_buf(&self.con) {
            Ok(msg) => Ok(msg),
            Err(err) => Err(err),
        }
    }
}

pub fn connect(peer: &Peer, info_hash: [u8; 20], handshake: [u8; 68]) -> Result<TcpStream, String> {
    //connect to tcp and send handshake
    let stream = match TcpStream::connect_timeout(
        &SocketAddr::new(std::net::IpAddr::V4(peer.ip), peer.port),
        Duration::from_secs(TIMEOUT_DURATION),
    ) {
        Ok(con) => con,
        Err(e) => {
            if e.kind() == io::ErrorKind::TimedOut {
                return Err(String::from("connection operation timed out!"));
            } else {
                return Err(format!("network error occurred: {}", e));
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

    match complete_handshake(stream, info_hash, peer, handshake) {
        Ok(tcp_stream) => Ok(tcp_stream),
        Err(err) => return Err(err),
    }
}

pub fn complete_handshake(
    mut stream: TcpStream,
    info_hash: [u8; 20],
    peer: &Peer,
    handshake: [u8; 68],
) -> Result<TcpStream, String> {
    // send/write handshake
    match stream.write_all(&handshake) {
        Ok(_) => {}
        Err(e) => {
            if e.kind() == io::ErrorKind::TimedOut {
                return Err(String::from("Write operation timed out!"));
            } else {
                return Err(e.to_string());
            }
        }
    };

    // recieve/read response
    // only reads 68 bytes of responnse // could cause problems i dont know for sure
    let mut buffer = [0; 68];
    match stream.read_exact(&mut buffer) {
        Ok(_) => {}
        Err(e) => {
            if e.kind() == io::ErrorKind::TimedOut {
                return Err(String::from("read operation timed out!"));
            } else {
                return Err(e.to_string());
            }
        }
    };
    let response = buffer;
    let rec_handshake = match handshake::read_handshake(response) {
        Ok(handshake) => handshake,
        Err(_) => return Err(String::from("error receiving the handshake")),
    };

    info(format!(
        "{} {} \n\tprotocol id:{} \n\tinfo hash:{:?}  \n\tpeer id: {}\n",
        "received handshake from peer:",
        format!("{}:{}", peer.ip, peer.port),
        rec_handshake.protocol_id,
        rec_handshake.info_hash,
        String::from_utf8_lossy(&rec_handshake.peer_id).to_string()
    ));

    if rec_handshake.info_hash == info_hash {
        // seccuss continue the cmmunication
        info("successful handshake".to_string());
        Ok(stream)
    } else {
        //failure
        Err(String::from(
            "info hash received does not match your info hash",
        ))
    }
}

pub fn bitfield(con: &TcpStream) -> Result<Message, String> {
    let response = match from_buf(con) {
        Ok(msg) => msg,
        Err(err) => {
            return Err(format!(
                "error occurred when getting bitfields message: {err}",
            ))
        }
    };
    Ok(response)
}
