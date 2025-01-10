use std::net::Ipv4Addr;

use crate::torrent::Torrent;
use crate::utils::encode_binnary_to_http_chars;
use crate::{bencode::Decoder, constants};
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

pub fn get_peers(torrent_data: Torrent, peer_id: [u8; 20]) -> Result<PeersResult, String> {
    Peer::get_peers(torrent_data, peer_id)
}

impl Peer {
    pub fn get_peers(torrent_data: Torrent, peer_id: [u8; 20]) -> Result<PeersResult, String> {
        let url = build_http_url(torrent_data, peer_id).unwrap();
        // todo: add error handling for in case disconnected
        let result = send_request(url).unwrap();
        let decoded_response = Decoder::new(result.as_bytes()).start().unwrap();

        let json_response: Value = serde_json::from_str(&decoded_response.result).unwrap();
        let mut peers: Vec<Peer> = Vec::new();
        //extract peers ip adresses from the serde json object and insert them into the list of peers
        if let Some(array) = json_response["peers"].as_array() {
            for item in array {
                if let (Some(ip), Some(port)) = (item["ip"].as_str(), item["port"].as_u64()) {
                    peers.push(Peer {
                        ip: ip.parse().expect("Invalid IP address format"),
                        port: port.try_into().unwrap(),
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

fn build_http_url(torrent_data: Torrent, peer_id: [u8; 20]) -> Result<String, String> {
    let url = torrent_data.announce
        + "?info_hash="
        + &encode_binnary_to_http_chars(torrent_data.info_hash)
        + "&peer_id="
        + &encode_binnary_to_http_chars(peer_id)
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
