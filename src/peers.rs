use std::net::{Ipv4Addr, SocketAddr, UdpSocket};

use crate::torrent::Torrent;
use crate::utils::encode_binnary_to_http_chars;
use crate::{bencode::Decoder, constants};
use reqwest::blocking::Client;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Peer {
    pub ip: Ipv4Addr,
    pub port: u16,
}

#[derive(Debug)]
pub struct PeersResult {
    pub peers: Vec<Peer>,
    pub interval: u64,
}

pub fn get_peers(torrent_data: &Torrent, peer_id: [u8; 20]) -> Result<PeersResult, String> {
    //todo: make this request to get peers in udp instead of http
    //let url: String = torrent_data.announce.clone();
    //let url_parts: Vec<&str> = url.split("/").collect();
    //println!("{:?}", url_parts);
    //let addr = url_parts[2];
    //
    //let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    //
    //let mut buf = [0; 16];
    //let protocol_id = u64::to_be_bytes(41727101980);
    //let action = u32::to_be_bytes(0);
    //let transaction_id = u32::to_be_bytes(1563244); //random
    //buf[0..8].copy_from_slice(&protocol_id);
    //buf[8..12].copy_from_slice(&action);
    //buf[12..16].copy_from_slice(&transaction_id);
    //
    //println!("{:?}", buf);
    //println!("{:?}", addr);
    //
    //let remote_addr: SocketAddr = "nyaa.tracker.wf:7777"
    //    .parse()
    //    .expect("could not parse addr");
    //
    //socket.connect(remote_addr).unwrap();
    //socket.send(&buf).unwrap();
    //
    //let mut buffer = [0; 1024];
    //let (amt, src) = socket.recv_from(&mut buffer).unwrap();
    //
    //println!("******************{}", amt);
    //println!("******************{}", src);
    // // // // // // // // // // // //
    // // // // // // // // // // // //
    // // // // // // // // // // // //
    // // // // // // // // // // // //
    // // // // // // // // // // // //
    // // // // // // // // // // // //
    // old http based peers tracking

    let url = build_http_url(torrent_data, peer_id).unwrap();
    println!("{}", url);
    // todo: add error handling for in case disconnected
    let result = send_request(url).unwrap();
    println!("{}", result);
    let decoded_response = Decoder::new(result.as_bytes()).start().unwrap();

    let json_response: Value = serde_json::from_str(&decoded_response.result).unwrap();
    let mut peers: Vec<Peer> = Vec::new();
    //extract peers ip addresses from the serde json object and insert them into the list of peers
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

fn build_http_url(torrent_data: &Torrent, peer_id: [u8; 20]) -> Result<String, String> {
    let url = torrent_data.announce.clone()
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
