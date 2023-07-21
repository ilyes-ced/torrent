use rand::Rng;
use sha1::{Digest, Sha1};
use std::{
    fs::File,
    net::{SocketAddr, ToSocketAddrs, UdpSocket},
    time::Duration,
};

use crate::{
    bencode::{self, decode::DecoderElement},
    utils::{
        concat, new_peer_id, transform_u16_to_array_of_u8, transform_u32_to_array_of_u8,
        transform_u64_to_array_of_u8,
    },
};

const MAX_RETRIES: u32 = 8;
const INITIAL_TIMEOUT: u32 = 100000000; // in nanoseconds // set to 100 ms
const IP_ADDR: &str = "0.0.0.0";
const PORT: u16 = 6881;
// tempo
const TRACKER_URL: &str = "tracker.openbittorrent.com:80";

#[derive(Debug)]
pub struct Peers {
    url: String,
    buffer: Vec<u8>,
    socket: UdpSocket,
    socket_addr: SocketAddr,
    retry: bool,
    retry_counter: u32,
    transaction_id: [u8; 4],
    connection_id: [u8; 8],
    peer_id: [u8; 20],
    info_hash: [u8; 20],
    size: [u8; 8],
}


#[derive(Debug)]
pub struct PeersResult {
    pub ips: Vec<String>,
    pub peer_id: [u8; 20],
    pub info_hash: [u8; 20],
}

impl Peers {
    pub fn new(torrent_string: Vec<u8>) -> Result<Self, String> {
        let decoded_torrent_file = bencode::from_bencode(&torrent_string).unwrap();
        if let DecoderElement::Dict(ele) = decoded_torrent_file {
            if let DecoderElement::String(string) = &ele[0].value {
                let url = String::from_utf8_lossy(&string).to_string();
                // the original url doesn't work because of the resource path: </announce> and the <udp://>
                let mut addrs_iter = TRACKER_URL.to_socket_addrs().unwrap();
                let socket_addr = addrs_iter.next().unwrap();
                let socket = UdpSocket::bind(format!("{}:{}", IP_ADDR, PORT)).unwrap();
                let res = Peers {
                    url,
                    buffer: torrent_string,
                    socket,
                    socket_addr,
                    retry: true,
                    retry_counter: 0,
                    transaction_id: [0; 4],
                    connection_id: [0; 8],
                    peer_id: new_peer_id(),
                    info_hash: [0; 20],
                    size: [0; 8],
                };
                Ok(res)
            } else {
                println!("error: url not found");
                Err(String::from("some error"))
            }
        } else {
            println!("error: invalid file structure");
            Err(String::from("some error"))
        }
    }

    pub fn get_peers(&mut self) -> Result<PeersResult, String> {
        let mut rng = rand::thread_rng();
        self.transaction_id = rng.gen::<[u8; 4]>();
        // crb = connection_request_buffer
        let mut crb = [0; 16];
        crb[0..8]
            .copy_from_slice(&transform_u64_to_array_of_u8(0x41727101980)); // connection_id
        crb[8..12].copy_from_slice(&transform_u32_to_array_of_u8(0x0)); // action: connect 0
        crb[12..16].copy_from_slice(&self.transaction_id); // transaction_id
        let mut timeout: Duration = Duration::new(0, INITIAL_TIMEOUT);
        self.socket
            .set_read_timeout(Some(timeout))
            .expect("set_read_timeout call failed");
        while self.retry {
            if self.retry_counter == MAX_RETRIES {
                println!("too many retries");
                std::process::exit(1);
            }
            self.retry_counter += 1;
            self.socket
                .send_to(&crb, self.socket_addr)
                .unwrap();
            match self.socket.recv_from(&mut crb) {
                Ok(..) => {
                    self.retry = false;
                    self.transaction_id
                        .copy_from_slice(&crb[4..8]);
                    self.connection_id
                        .copy_from_slice(&crb[8..16]);
                }
                Err(..) => {
                    println!("doubled the timeout to {:?}", timeout);
                    timeout = timeout * 2;
                    self.socket
                        .set_read_timeout(Some(timeout))
                        .expect("set_read_timeout call failed");
                }
            }
        }

        // reinit timeout
        self.retry = true;
        self.retry_counter = 0;
        timeout = Duration::new(0, INITIAL_TIMEOUT);

        //    __ _ _ __  _ __  _ __   ___  _   _ _ __   ___ ___
        //   / _` | '_ \| '_ \| '_ \ / _ \| | | | '_ \ / __/ _ \
        //  | (_| | | | | | | | | | | (_) | |_| | | | | (_|  __/
        //   \__,_|_| |_|_| |_|_| |_|\___/ \__,_|_| |_|\___\___|

        let decoded_file = bencode::from_bencode(&self.buffer).unwrap();
        self.info_hash(decoded_file.clone())?;
        self.size(decoded_file)?;

        // arb = announce_request_buffer
        let mut arb = [0; 98];
        arb[0..8].copy_from_slice(&self.connection_id); // connection_id
        arb[8..12].copy_from_slice(&[0, 0, 0, 1]); // action
        arb[12..16].copy_from_slice(&self.transaction_id); // transaction_id
        arb[16..36].copy_from_slice(&self.info_hash); // info_hash
        arb[36..56].copy_from_slice(&self.peer_id); // peer_id
        arb[56..64].copy_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]); // downloaded
        arb[64..72].copy_from_slice(&self.size); // left
        arb[72..80].copy_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]); // uploaded
        arb[80..84].copy_from_slice(&[0, 0, 0, 0]); // event
        arb[84..88].copy_from_slice(&[0, 0, 0, 0]); // IP
        arb[88..92].copy_from_slice(&rng.gen::<[u8; 4]>()); // key
        arb[92..96].copy_from_slice(&[255, 255, 255, 255]); // num_want = -1
        arb[96..98].copy_from_slice(&transform_u16_to_array_of_u8(PORT)); // port
        let clone = arb.clone();

        while self.retry {
            if self.retry_counter == MAX_RETRIES {
                println!("too many retries");
                std::process::exit(1);
            }
            self.retry_counter += 1;
            self.socket
                .send_to(&arb, self.socket_addr)
                .unwrap();
            match self.socket.recv_from(&mut arb) {
                Ok(_) => {
                    self.retry = false;
                    let extra_bytes = {
                        let mut extra_bytes = 0;
                        for i in 0..98 {
                            if &arb[i..] == &clone[i..] {
                                extra_bytes = i;
                                break;
                            }
                        }
                        extra_bytes - 16
                    };
                    if extra_bytes == 0 {
                        return Err(String::from("no extra bytes"));
                    }

                    let mut peers_ids: Vec<String> = Vec::new();
                    let num_ips = (extra_bytes - 4) / 6;

                    for i in 0..num_ips {
                        peers_ids.push(format!(
                            "{}.{}.{}.{}:{}",
                            arb[20 + i * 6 + 0],
                            arb[20 + i * 6 + 1],
                            arb[20 + i * 6 + 2],
                            arb[20 + i * 6 + 3],
                            u16::from_be_bytes([
                                arb[20 + i * 6 + 4],
                                arb[20 + i * 6 + 5]
                            ])
                        ))
                    }

                    println!("\tips {:?}", peers_ids);
                    return Ok(PeersResult {
                        ips: peers_ids,
                        peer_id: self.peer_id,
                        info_hash: self.info_hash,
                    });
                }
                Err(..) => {
                    println!("doubled the timeout to {:?}", timeout);
                    timeout = timeout * 2;
                    self.socket
                        .set_read_timeout(Some(timeout))
                        .expect("set_read_timeout call failed");
                }
            }
        }

        Err(String::from("hello there"))
    }

    fn size(&mut self, element: DecoderElement) -> Result<(), String> {
        let mut total: u64 = 0;
        // idk
        if let DecoderElement::Dict(pairs) = element {
            for pair in pairs {
                if pair.name == String::from("info") {
                    if let DecoderElement::Dict(info_dicts) = pair.value {
                        for info_dict in info_dicts {
                            if info_dict.name == String::from("files") {
                                if let DecoderElement::List(files_list) = info_dict.value {
                                    for file in files_list {
                                        if let DecoderElement::Dict(file_details) = file {
                                            for details in file_details {
                                                if details.name == String::from("length") {
                                                    if let DecoderElement::Number(value) =
                                                        details.value
                                                    {
                                                        total += concat(&value) as u64
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if total == 0 {
            return Err(String::from("lengths not found"));
        }
        self.size = transform_u64_to_array_of_u8(total);
        Ok(())
    }

    fn info_hash(&mut self, decoded_file: DecoderElement) -> Result<(), String> {
        let mut info_bytes: Vec<u8> = Vec::new();
        // decoded again here might need a work around
        let _: Result<Vec<u8>, String> = {
            if let DecoderElement::Dict(pairs) = decoded_file {
                for pair in pairs {
                    if pair.name == String::from("info") {
                        info_bytes = match bencode::to_bencode(pair.value) {
                            Ok(result) => Ok(result),
                            Err(_) => Err(String::from("idk")),
                        }?
                    }
                }
                Err(String::from(
                    "info field not fount (might require nested searching feature)",
                ))
            } else {
                Err(String::from("malformed file"))
            }
        };
        let mut hasher = Sha1::new();
        hasher.update(info_bytes);
        let info_hash: &[u8] = &hasher.finalize();
        if info_hash.len() != 20 {
            Err(String::from("bad hash i think"))
        } else {
            self.info_hash[0..20].copy_from_slice(info_hash);
            Ok(())
        }
    }
}
