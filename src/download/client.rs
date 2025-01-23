use crate::{
    constants::{MsgId, TIMEOUT_DURATION},
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
    pub info_hash: [u8; 20],
    pub peer_id: [u8; 20],
}

impl Client {
    pub fn new(torrent: &Torrent, peer_index: usize) -> Result<Self, String> {
        let handshake = Handshake::new(torrent.info_hash, torrent.peer_id).create_handshake();
        let con = match connect(torrent, peer_index) {
            Ok(tcp_stream) => {
                match complete_handshake(tcp_stream, torrent, handshake, peer_index) {
                    Ok(tcp_stream) => tcp_stream,
                    Err(err) => return Err(err),
                }
            }
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
            info_hash: torrent.info_hash,
            peer_id: torrent.peer_id,
        })
    }

    // sends Messages of CHOKE/INTRESTED/.../.../...
    pub fn send_msg_id(&mut self, signal: MsgId, payload: Option<Vec<u8>>) -> Result<(), String> {
        // signal is one of the constants in MsgId
        let msg = Some(Message {
            id: signal.to_u8(),
            payload: payload.unwrap_or_else(Vec::new),
        });
        match self.con.write(&to_buf(msg)) {
            Ok(_) => {}
            Err(e) => {
                if e.kind() == io::ErrorKind::TimedOut {
                    return Err(String::from("Write operation timed out!"));
                } else {
                    return Err(String::from(format!("An error occurred: {}", e)));
                }
            }
        };
        Ok(())
    }

    pub fn read_msg(&mut self) -> Result<Message, String> {
        let response = match from_buf(&self.con) {
            Ok(msg) => msg.unwrap(),
            Err(err) => {
                return Err(String::from(format!(
                    "error occured when reading message: {err}",
                )))
            }
        };
        println!("second read: recieving message:  {:?}", response);
        Ok(response)
    }

    pub fn choking(&mut self, choke: bool) {
        self.choked = choke;
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
