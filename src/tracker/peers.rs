use std::net::Ipv4Addr;

use crate::torrent::Torrent;
use crate::{bencode::decode::Decoder, constants};
use rand::distributions::{Alphanumeric, DistString};
use reqwest::blocking::Client;
use serde_json::Value;

#[derive(Debug)]
pub struct Peer {
    ip: Ipv4Addr,
    port: u16,
}

#[derive(Debug)]
pub struct PeersResult {
    peers: Vec<Peer>,
    interval: u64,
}

impl Peer {
    pub fn get_peers(torrent_data: Torrent) -> Result<PeersResult, String> {
        let url = build_http_url(torrent_data).unwrap();
        let result = send_request(url).unwrap();
        let decoded_response = Decoder::new(result.as_bytes()).start().unwrap();

        let json_response: Value = serde_json::from_str(&decoded_response.result).unwrap();
        let mut peers: Vec<Peer> = Vec::new();
        if let Some(array) = json_response["peers"].as_array() {
            for item in array {
                if let (Some(ip), Some(port)) = (item["ip"].as_str(), item["port"].as_u64()) {
                    peers.push(Peer {
                        ip: ip.parse().expect("Invalid IP address format"),
                        port: 10,
                    })
                }
            }
        } else {
            panic!("we couldnt find peers in the tracker response");
        }

        let interval = json_response["interval"].as_u64().unwrap_or(900);

        Ok(PeersResult { peers, interval })
    }
}

fn send_request(url: String) -> Result<String, String> {
    let client = Client::new();

    let response = match client.get(url).send() {
        Ok(res) => res,
        Err(err) => return Err(format!("Failed to fetch data: {}", err)),
    };

    if response.status().is_success() {
        let body = match response.text() {
            Ok(res) => res,
            Err(err) => return Err(format!("Failed to fetch data: {}", err)),
        };
        Ok(body)
    } else {
        Err(format!("Failed to fetch data: {}", response.status()))
    }
}

fn build_http_url(torrent_data: Torrent) -> Result<String, String> {
    let url = torrent_data.announce
        + "?info_hash="
        + &encode_bin(torrent_data.info_hash)
        + "&peer_id="
        + &encode_bin(new_peer_id())
        + "&port="
        + &constants::PORT.to_string()
        + "&uploaded="
        + "0" //uploaded
        + "&downloaded="
        + "0" //downloaded
        + "&left="
        + "0"; //left calculate it later

    Ok(url)
}

fn new_peer_id() -> [u8; 20] {
    //"-IT0001-"+12 random chars
    let mut id = [0; 20];
    id[0..8].copy_from_slice("-IT0001-".as_bytes());
    let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 12);
    id[8..20].copy_from_slice(string.as_bytes());
    id
}

fn encode_bin(input: [u8; 20]) -> String {
    let mut return_string = String::new();
    for byte in input {
        return_string.push_str("%");
        return_string.push_str(&format!("{:02x}", byte));
    }
    return_string
}
