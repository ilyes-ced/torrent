use rand::{
    distributions::{Alphanumeric, DistString},
    Rng,
};
use sha1::{Digest, Sha1};
use std::{
    fs::File,
    net::{SocketAddr, ToSocketAddrs, UdpSocket},
    time::Duration,
};

use crate::bencode::{self, decode::DecoderElement};

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




// deconstruct file into 
//      url
//      info
//      info hash
//      size
//      
//      
//      
//      
//      














impl Peers {
    pub fn new(torrent_string: Vec<u8>, file: File) -> Result<Self, String> {
        let gg = bencode::from_bencode(&torrent_string).unwrap();
        if let DecoderElement::Dict(ele) = gg {
            println!("annound here: {}", ele[0].name);
            if let DecoderElement::String(string) = &ele[0].value {
                println!("url here: {}", String::from_utf8_lossy(&string).to_string());
                let url = String::from_utf8_lossy(&string).to_string();
                // the original url doesnt work because of the resouce path: /announce and the udp://
                let mut addrs_iter = TRACKER_URL.to_socket_addrs().unwrap();
                let socket_addr = addrs_iter.next().unwrap();
                println!("{:?}", addrs_iter.next());
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

    pub fn get_peers(&mut self) -> Result<String, String> {
        let mut rng = rand::thread_rng();
        self.transaction_id = rng.gen::<[u8; 4]>();
        let mut connection_reques_buffer = [0; 16];
        connection_reques_buffer[0..8].copy_from_slice(&transform_u64_to_array_of_u8(0x41727101980)); // connection_id
        connection_reques_buffer[8..12].copy_from_slice(&transform_u32_to_array_of_u8(0x0)); // action: connect 0
        connection_reques_buffer[12..16].copy_from_slice(&self.transaction_id); // transaction_id

        //let mut transaction_id: [u8; 4] = [0, 0, 0, 0];
        //let mut connection_id: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        //let mut retry: bool = true;
        //let mut retry_counter = 0;
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
            self.socket.send_to(&connection_reques_buffer, self.socket_addr).unwrap();
            match self.socket.recv_from(&mut connection_reques_buffer) {
                Ok(result) => {
                    self.retry = false;
                    self.transaction_id.copy_from_slice(&connection_reques_buffer[4..8]);
                    self.connection_id.copy_from_slice(&connection_reques_buffer[8..16]);
                    println!("response:");
                    println!("\t{:x?}", &connection_reques_buffer);
                    println!("\t{:?}", result);
                    println!("\tset transaction_id: {:x?}", self.transaction_id);
                    println!("\tset connection_id: {:x?}", self.connection_id);
                }
                Err(..) => {
                    println!("doubled the timeout to {:?}", timeout);
                    timeout = timeout * 2;
                }
            }
        }

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

        //self.transaction_id = rng.gen::<[u8; 4]>();
        let mut announce_request_buffer = [0; 98];
        announce_request_buffer[0..8].copy_from_slice(&self.connection_id); // connection_id
        announce_request_buffer[8..12].copy_from_slice(&[0, 0, 0, 1]); // action
        announce_request_buffer[12..16].copy_from_slice(&self.transaction_id); // transaction_id
        announce_request_buffer[16..36].copy_from_slice(&self.info_hash); // info_hash
        announce_request_buffer[36..56].copy_from_slice(&self.peer_id); // peer_id
        announce_request_buffer[56..64].copy_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]); // downloaded
        announce_request_buffer[64..72].copy_from_slice(&self.size); // left
        announce_request_buffer[72..80].copy_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]); // uploaded
        announce_request_buffer[80..84].copy_from_slice(&[0, 0, 0, 0]); // event
        announce_request_buffer[84..88].copy_from_slice(&[0, 0, 0, 0]); // IP
        announce_request_buffer[88..92].copy_from_slice(&rng.gen::<[u8; 4]>()); // key
        announce_request_buffer[92..96].copy_from_slice(&[255, 255, 255, 255]); // num_want -1
        announce_request_buffer[96..98].copy_from_slice(&transform_u16_to_array_of_u8(PORT)); // port
        let clone = announce_request_buffer.clone();
        println!("\tbuffer before sending:::::::::::::::::\n \t {:?}", &announce_request_buffer);

        while self.retry {
            if self.retry_counter == MAX_RETRIES {
                println!("too many retries");
                std::process::exit(1);
            }
            self.retry_counter += 1;
            self.socket.send_to(&announce_request_buffer, self.socket_addr).unwrap();
            match self.socket.recv_from(&mut announce_request_buffer) {
                Ok(_) => {
                    self.retry = false;
                    println!("announce request response:");
                    println!("\ttransiction id is : {:?}", &self.transaction_id);
                    println!("\tbuffer after recieving:::::::::::::::::\n \t{:?}", &announce_request_buffer);
                    println!("\taction {:?}", (&announce_request_buffer[0..4].to_vec()));
                    println!("\ttransaction_id {:?}", (&announce_request_buffer[4..8].to_vec()));
                    println!("\tinterval {:?}", (&announce_request_buffer[8..12].to_vec()));
                    
                    println!("\tinterval {:?}", u32::from_be_bytes(
                        [
                            announce_request_buffer[8],
                            announce_request_buffer[9],
                            announce_request_buffer[10],
                            announce_request_buffer[11],
                        ]
                    ));
                    
                    println!("\tinterval {:?}", (transform_u32_to_array_of_u8(
                        u32::from_be_bytes(
                            [
                                announce_request_buffer[8],
                                announce_request_buffer[9],
                                announce_request_buffer[10],
                                announce_request_buffer[11],
                            ]
                        )
                    )));


                    println!("\tleechers {:?}", (&announce_request_buffer[12..16].to_vec()));
                    let extra_bytes = {
                        let mut extra_bytes = 0;
                        for i in 0..98 {
                            if &announce_request_buffer[i..] == &clone[i..] {
                                println!("************************************************************************************************************************************************ {}", i);
                                println!("\tthe rest {:?}", &announce_request_buffer[16..i].to_vec());
                                extra_bytes = i;
                                break
                            }
                        }
                        extra_bytes - 16
                    };
                    if extra_bytes == 0 {
                        println!("error no extra bytes here requyest failure");
                        return Err(String::from("no extra bytes"));
                    }
                    let mut seeders: Vec<[u8; 4]> = Vec::new();
                    let num_seeder = (extra_bytes - 6)/4;
                    for i in 0..num_seeder {
                        seeders.push([
                            announce_request_buffer[16 + i*4],
                            announce_request_buffer[16 + i*4 + 1],
                            announce_request_buffer[16 + i*4 + 2],
                            announce_request_buffer[16 + i*4 + 3],
                        ]);
                    }
                    println!("\tseeders {:?}", seeders);
                    let ip: [u8; 4] = [
                        announce_request_buffer[16 + num_seeder*4],
                        announce_request_buffer[16 + num_seeder*4 + 1],
                        announce_request_buffer[16 + num_seeder*4 + 2],
                        announce_request_buffer[16 + num_seeder*4 + 3],
                    ];
                    let port: [u8; 2] = [
                        announce_request_buffer[16 + num_seeder*4 + 4],
                        announce_request_buffer[16 + num_seeder*4 + 5],
                    ];
                    println!("\tip {:?}", ip);
                    println!("\tport {:?}", port);
                    println!("\tport {:?}", u16::from_be_bytes(port));
                    
                    println!("{:x?}", self.info_hash);
                }
                Err(..) => {
                    println!("doubled the timeout to {:?}", timeout);
                    timeout = timeout * 2;
                }
            }
        }

        Ok(String::new())
    }

    fn size(&mut self, element: DecoderElement) -> Result<(), String> {
        let mut total: u64 = 0;

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
                            Ok(result) => {
                                Ok(result)
                            },
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

fn transform_u16_to_array_of_u8(x: u16) -> [u8; 2] {
    let b1: u8 = ((x >> 8) & 0xff) as u8;
    let b2: u8 = (x & 0xff) as u8;
    [b1, b2]
}
fn transform_u32_to_array_of_u8(x: u32) -> [u8; 4] {
    let b1: u8 = ((x >> 24) & 0xff) as u8;
    let b2: u8 = ((x >> 16) & 0xff) as u8;
    let b3: u8 = ((x >> 8) & 0xff) as u8;
    let b4: u8 = (x & 0xff) as u8;
    [b1, b2, b3, b4]
}
fn transform_u64_to_array_of_u8(x: u64) -> [u8; 8] {
    let b1: u8 = ((x >> 56) & 0xff) as u8;
    let b2: u8 = ((x >> 48) & 0xff) as u8;
    let b3: u8 = ((x >> 40) & 0xff) as u8;
    let b4: u8 = ((x >> 32) & 0xff) as u8;
    let b5: u8 = ((x >> 24) & 0xff) as u8;
    let b6: u8 = ((x >> 16) & 0xff) as u8;
    let b7: u8 = ((x >> 8) & 0xff) as u8;
    let b8: u8 = (x & 0xff) as u8;
    [b1, b2, b3, b4, b5, b6, b7, b8]
}

fn new_peer_id() -> [u8; 20] {
    //"-IT0001-"+12 random chars
    let mut res = [0; 20];
    res[0..8].copy_from_slice("-IT0001-".as_bytes());
    let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 12);
    res[8..20].copy_from_slice(string.as_bytes());
    res
}

fn concat(vec: &Vec<u8>) -> usize {
    let mut acc: usize = 0;
    for elem in vec {
        acc *= 10;
        match elem {
            b'0' => acc += 0,
            b'1' => acc += 1,
            b'2' => acc += 2,
            b'3' => acc += 3,
            b'4' => acc += 4,
            b'5' => acc += 5,
            b'6' => acc += 6,
            b'7' => acc += 7,
            b'8' => acc += 8,
            b'9' => acc += 9,
            _ => {
                // impossible i think
            }
        }
    }
    acc
}


