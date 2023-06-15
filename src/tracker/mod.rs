use std::{net::{ToSocketAddrs, UdpSocket, SocketAddr}, time::Duration, fs::File, io::Read};
use rand::{Rng, distributions::{Alphanumeric, DistString}};
use sha1::{Sha1, Digest};


use crate::bencode::{decode::{self, DecoderElement}, self};


const MAX_RETRIES: u32 = 8;
const INITIAL_TIMEOUT: u32 = 100000000; // in nanoseconds // set to 100 ms


#[derive(Debug)]
pub struct Peers{
    url: String,
    buffer: Vec<u8>,
    socket: UdpSocket,
    socket_addr: SocketAddr,
    retry: bool,
    retry_counter: u32,
    transaction_id: [u8; 4],
    connection_id: [u8; 8],
    peer_id: [u8; 20],
}


impl Peers {
    pub fn new(torrent_string: Vec<u8>, file: File) -> Result<Self, String>{
        let gg = bencode::from_bencode(&torrent_string).unwrap();
        if let DecoderElement::Dict(ele) = gg {
            println!("annound here: {}", ele[0].name);
            if let DecoderElement::String(string) = &ele[0].value {
                println!("url here: {}", String::from_utf8_lossy(&string).to_string());
                let url = String::from_utf8_lossy(&string).to_string();
                // the original url doesnt work because of the resouce path: /announce and the udp://
                let mut addrs_iter = "tracker.openbittorrent.com:80".to_socket_addrs().unwrap();
                let socket_addr = addrs_iter.next().unwrap();
                println!("{:?}", addrs_iter.next());
                let socket = UdpSocket::bind("0.0.0.0:34254").unwrap();
                let res = Peers {
                    url,
                    buffer: torrent_string,
                    socket,
                    socket_addr,
                    retry: true,
                    retry_counter: 0,
                    transaction_id: [0; 4],
                    connection_id: [0; 8],
                    peer_id: new_peer_id()
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
        let mut buffer = [0; 16];
        buffer[0..8].copy_from_slice(&transform_u64_to_array_of_u8(0x41727101980)); // connection_id
        buffer[8..12].copy_from_slice(&transform_u32_to_array_of_u8(0x0)); // action: connect 0
        buffer[12..16].copy_from_slice(&self.transaction_id); // transaction_id



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


        let info_hash = {
            let mut hasher = Sha1::new();
            hasher.update(&self.buffer);
            let decoded = bencode::from_bencode(&self.buffer).unwrap();

            //             __   _            _           _                     _                     _         _            __                             _       _     _                 __   _   _        
            //            / _| (_) __  __   | |   __ _  | |_    ___   _ __    | |__     __ _   ___  | |__     (_)  _ __    / _|   ___      _ __     ___   | |_    | |_  | |__     ___     / _| (_) | |   ___ 
            //           | |_  | | \ \/ /   | |  / _` | | __|  / _ \ | '__|   | '_ \   / _` | / __| | '_ \    | | | '_ \  | |_   / _ \    | '_ \   / _ \  | __|   | __| | '_ \   / _ \   | |_  | | | |  / _ \
            //           |  _| | |  >  <    | | | (_| | | |_  |  __/ | |      | | | | | (_| | \__ \ | | | |   | | | | | | |  _| | (_) |   | | | | | (_) | | |_    | |_  | | | | |  __/   |  _| | | | | |  __/
            //           |_|   |_| /_/\_\   |_|  \__,_|  \__|  \___| |_|      |_| |_|  \__,_| |___/ |_| |_|   |_| |_| |_| |_|    \___/    |_| |_|  \___/   \__|    \__| |_| |_|  \___|   |_|   |_| |_|  \___|
            //                           
            if let DecoderElement::Dict(ele) = decoded {
                println!("info start here======================> here: {}", ele[4].name);
                if let DecoderElement::Dict(DecoderElement) = &ele[4].value {
                    println!("info start here======================> here: {}", DecoderElement[0].name);
                    println!(".............................: {:?}", DecoderElement[0].value);
                }else{
                    println!("error it isnt of list type");
                }
            }                                                                                                                                                              
            let info_hash = hasher.finalize();
            if info_hash.len() != 20 {
                Err(String::from("bad hash i think"))
            }else{
                Ok(info_hash)
            }
        }?;

        self.transaction_id = rng.gen::<[u8; 4]>();
        let mut buf = [0; 98];
        buf[0..8].copy_from_slice(&self.connection_id); // connection_id
        buf[8..12].copy_from_slice(&[0, 0, 0, 1]); // action
        buf[12..16].copy_from_slice(&self.transaction_id); // transaction_id
        buf[16..36].copy_from_slice(&info_hash); // info_hash
        buf[36..56].copy_from_slice(&self.peer_id); // peer_id
        buf[56..64].copy_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]); // downloaded
        buf[64..72].copy_from_slice(&rng.gen::<[u8; 8]>()); // left
        buf[72..80].copy_from_slice(&rng.gen::<[u8; 8]>()); // uploaded
        buf[80..84].copy_from_slice(&rng.gen::<[u8; 4]>()); // event
        buf[84..88].copy_from_slice(&rng.gen::<[u8; 4]>()); // IP
        buf[88..92].copy_from_slice(&rng.gen::<[u8; 4]>()); // key
        buf[92..96].copy_from_slice(&rng.gen::<[u8; 4]>()); // num_want
        buf[96..98].copy_from_slice(&rng.gen::<[u8; 2]>()); // port
        /*
            let arr = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
            let sub_arr1 = &arr[0][0..2];
            let sub_arr2 = &arr[1][1..3];
            let new_arr = [0; 4];
            new_arr[0..2].copy_from_slice(sub_arr1);
            new_arr[2..4].copy_from_slice(sub_arr2);
            println!("{:?}", new_arr);
        */

        Ok(String::new())
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

fn new_peer_id() -> [u8; 20] {
    //"-IT0001-"+12 random chars
    let mut res = [0; 20];
    res[0..8].copy_from_slice("-IT0001-".as_bytes());
    let mut string = Alphanumeric.sample_string(&mut rand::thread_rng(), 12);
    res[8..20].copy_from_slice(string.as_bytes());
    //println!("{:?}", res);
    res
}

