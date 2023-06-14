use std::{net::{ToSocketAddrs, UdpSocket, SocketAddr}, time::Duration, fs::File};
use sha1::{Sha1, Digest};


use crate::bencode::{decode::{self, DecoderElement}, self};


const MAX_RETRIES: u32 = 8;
const INITIAL_TIMEOUT: u32 = 100000000; // in nanoseconds // set to 100 ms


#[derive(Debug)]
pub struct Peers{
    url: String,
    //file: File,
    socket: UdpSocket,
    socket_addr: SocketAddr,
    retry: bool,
    retry_counter: u32,
    transaction_id: [u8; 4],
    connection_id: [u8; 8],
}


impl Peers {
    pub fn new(torrent_string: Vec<u8>) -> Result<Self, String>{
        let gg = bencode::to_bencode(&torrent_string).unwrap();
        if let DecoderElement::Dict(ele) = gg {
            println!("annound here: {}", ele[0].name);
            if let DecoderElement::String(string) = &ele[0].value {
                println!("url here: {}", String::from_utf8_lossy(&string).to_string());
                let url = String::from_utf8_lossy(&string).to_string();

                // get torrent ip address
                let mut addrs_iter = "tracker.openbittorrent.com:80".to_socket_addrs().unwrap();
                let socket_addr = addrs_iter.next().unwrap();
                println!("{:?}", addrs_iter.next());
                let socket = UdpSocket::bind("0.0.0.0:34254").unwrap();

                let res = Peers {
                    url,
                    //file: todo!(),
                    socket,
                    socket_addr,
                    retry: true,
                    retry_counter: 0,
                    transaction_id: [0; 4],
                    connection_id: [0; 8],
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


    pub fn get_peers(&mut self) {
        let fir = transform_u64_to_array_of_u8(0x41727101980);
        let sec = transform_u32_to_array_of_u8(0x0);
        let thi = transform_u32_to_array_of_u8(0x3645); // random 4 bytes
        let mut buffer = [
            fir[0], fir[1], fir[2], fir[3], fir[4], fir[5], fir[6], fir[7],
            sec[0], sec[1], sec[2], sec[3],
            thi[0], thi[1], thi[2], thi[3],
        ];


        //let mut transaction_id: [u8; 4] = [0, 0, 0, 0];
        //let mut connection_id: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        //let mut retry: bool = true;
        //let mut retry_counter = 0;
        let mut timeout: Duration = Duration::new(0, INITIAL_TIMEOUT);
        self.socket.set_read_timeout(Some(timeout)).expect("set_read_timeout call failed");
        while self.retry {
            if self.retry_counter == MAX_RETRIES {
                println!("too many retries");
                std::process::exit(1);
            }
            self.retry_counter += 1;
            self.socket.send_to(&buffer, self.socket_addr).unwrap();
            match self.socket.recv_from(&mut buffer){
                Ok(result) => {
                    self.retry = false;
                    self.transaction_id.copy_from_slice(&buffer[4..8]);
                    self.connection_id.copy_from_slice(&buffer[8..16]);
                    if self.transaction_id.len() !=4 || self.connection_id.len() != 8 {
                        println!("errored response");
                        std::process::exit(1);
                    }
                    println!("response:");
                    println!("\t{:x?}", &buffer);
                    println!("\t{:?}", result);
                    println!("\t{:x?}", self.transaction_id);
                    println!("\t{:x?}", self.connection_id);
                },
                Err(..) => {
                    println!("doubled the timeout to {:?}", timeout);
                    timeout = timeout*2;
                }
            }
        }

        self.retry_counter = 0;

        //    __ _ _ __  _ __  _ __   ___  _   _ _ __   ___ ___ 
        //   / _` | '_ \| '_ \| '_ \ / _ \| | | | '_ \ / __/ _ \
        //  | (_| | | | | | | | | | | (_) | |_| | | | | (_|  __/
        //   \__,_|_| |_|_| |_|_| |_|\___/ \__,_|_| |_|\___\___|



        let mut hasher = Sha1::new();
        // torrent file here
        hasher.update(b"");
        
        let result = hasher.finalize();
        println!("\thash: {:x?}", result);

        if result.len() != 20 {
            // throw error
        }

        let conn = [self.connection_id[0], self.connection_id[1], self.connection_id[2], self.connection_id[3], self.connection_id[4], self.connection_id[5], self.connection_id[6], self.connection_id[7]];
        let mut buf: [u8; 98] = [
            conn[0], conn[1], conn[2], conn[3], conn[4], conn[5], conn[6], conn[7],  //connection_id
            0, 0, 0, 0,  //action
            165, 0xf6, 0xb5, 30,  //transaction_id
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  //info_hash
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  //peer_id
            0, 0, 0, 0, 0, 0, 0, 0,  //downloaded
            0, 0, 0, 0, 0, 0, 0, 0,  //left
            0, 0, 0, 0, 0, 0, 0, 0,  //uploaded
            0, 0, 0, 0,  //event
            0, 0, 0, 0,  //IP
            0, 0, 0, 0,  //key
            0, 0, 0, 0,  //num_want
            0, 0  //port
        ];
        println!("               \t{:x?}", &buf);


    }
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
