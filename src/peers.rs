use crate::constants::{INITIAL_TIMEOUT, MAX_RETRIES, PORT};
use crate::log::{debug, error, info, warning};
use crate::torrent::Torrent;
use crate::utils::{self, encode_binnary_to_http_chars};
use crate::{bencode::Decoder, constants};
use core::str;
use rand::{rng, Rng};
use reqwest::blocking::Client;
use serde_json::Value;
use std::net::{Ipv4Addr, ToSocketAddrs, UdpSocket};
use std::time::Duration;

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
    let url: String = torrent_data.announce.clone();
    let url_parts: Vec<&str> = url.split("/").collect();
    debug(format!("{:?}", url_parts));
    let addr: &str = url_parts[2];

    let mut buf: [u8; 16] = [0; 16];
    let protocol_id = u64::to_be_bytes(41727101980);
    let action = u32::to_be_bytes(0);
    buf[0..8].copy_from_slice(&protocol_id);
    buf[8..12].copy_from_slice(&action);
    buf[12..16].copy_from_slice(&u32::to_be_bytes(utils::new_transaction_id()));

    let my_protocol_id = &buf[0..8];
    let my_action = &buf[8..12];
    let my_transaction_id = &buf[12..];
    debug(format!("protocol id:    {:?}", my_protocol_id));
    debug(format!("action:         {:?}", my_action));
    debug(format!("transaction id: {:?}", my_transaction_id));

    debug(format!("{:?}", buf));
    debug(format!("tracker server addr: {:?}", addr));

    // todo: handle debugs
    let remote_host = addr.to_socket_addrs().unwrap().next().unwrap();

    debug(format!("remote_host: {:?}", remote_host));

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.connect(remote_host).unwrap();
    //socket.send(&buf).unwrap();
    let res = udp_req(&socket, buf.to_vec()).unwrap();
    let mut buffer = [0; 16];
    let (amt, _) = socket.recv_from(&mut buffer).unwrap();
    debug(format!("read {} bytes", amt));
    debug(format!("******************{:?}", buffer));

    let recv_action = &buffer[0..4];
    let recv_transaction_id = &buffer[4..8];
    let recv_connection_id = &buffer[8..];
    debug(format!("action:         {:?}", recv_action));
    debug(format!("transaction id: {:?}", recv_transaction_id));
    debug(format!("connection id:  {:?}", recv_connection_id));

    if recv_transaction_id == my_transaction_id {
        info(format!(
            "transactions ids match: {:?}, {:?}",
            my_transaction_id, recv_transaction_id
        ));
    } else {
        error("transactions ids do not match".to_string());
    }

    let announce_transaction_id = u32::to_be_bytes(utils::new_transaction_id());
    let mut req_buf = [0; 98];
    req_buf[0..8].copy_from_slice(&buffer[8..]); // connection_id
    req_buf[8..12].copy_from_slice(&u32::to_be_bytes(1)); // action
    req_buf[12..16].copy_from_slice(&announce_transaction_id); // transaction_id
    req_buf[16..36].copy_from_slice(&torrent_data.info_hash); // info_hash
    req_buf[36..56].copy_from_slice(&peer_id); // peer_id
    req_buf[56..64].copy_from_slice(&u64::to_be_bytes(0)); // downloaded
    req_buf[64..72].copy_from_slice(&u64::to_be_bytes(
        torrent_data.info.pieces.len().try_into().unwrap(),
    )); // left
    req_buf[72..80].copy_from_slice(&u64::to_be_bytes(0)); // uploaded
    req_buf[80..84].copy_from_slice(&u32::to_be_bytes(0)); // event
    req_buf[84..88].copy_from_slice(&u32::to_be_bytes(0)); // IP
    req_buf[88..92].copy_from_slice(&u32::to_be_bytes(rng().random::<u32>())); // key
    req_buf[92..96].copy_from_slice(&i32::to_be_bytes(-1)); // num_want = -1
    req_buf[96..98].copy_from_slice(&u16::to_be_bytes(PORT)); // port
    debug(format!("{:?}", req_buf));

    let announce_response = udp_req(&socket, req_buf.to_vec()).unwrap();
    debug(format!("res: {:?}", announce_response));

    //let (amt, _) = socket.recv_from(&mut announce_response).unwrap();
    //debug(format!("read {} bytes", amt));
    //debug(format!("announce response: {:?}", announce_response));

    if announce_transaction_id == announce_response[4..8] {
        info(format!(
            "transactions ids match: {:?}, {:?}",
            announce_transaction_id,
            &announce_response[4..8]
        ));
    } else {
        error("transactions ids do not match".to_string());
    }

    let action = &announce_response[0..4];
    if action == [0, 0, 0, 3] {
        error(format!(
            "error: {}",
            str::from_utf8(&announce_response[8..]).unwrap()
        ))
    }

    std::process::exit(0);
    // // // // // // // // // // // //
    // // // // // // // // // // // //
    // // // // // // // // // // // //
    // // // // // // // // // // // //
    // // // // // // // // // // // //
    // // // // // // // // // // // //
    // old http based peers tracking

    let url = build_http_url(torrent_data, peer_id).unwrap();
    // todo: add error handling for in case disconnected
    let result = send_request(url).unwrap();
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

fn udp_req(socket: &UdpSocket, request: Vec<u8>) -> Result<Vec<u8>, String> {
    let mut retry = true;
    let mut re_tries: u16 = 0;
    let mut timeout: Duration = Duration::new(0, INITIAL_TIMEOUT);

    socket
        .set_read_timeout(Some(timeout))
        .expect("set_read_timeout call failed");

    while retry {
        if re_tries == MAX_RETRIES {
            return Err(String::from("too many retries, cant receive data"));
        }

        if let Err(e) = socket.send(&request) {
            error(format!("Failed to send request 1.: {}", e));
        }

        let mut response: [u8; 1024] = [0; 1024];

        match socket.recv_from(&mut response) {
            Ok((amt, _)) => {
                info(format!("received: {:?}", response));
                debug(format!("read {} bytes", amt));
                debug(format!("announce response: {:?}", response));
                retry = false;
                return Ok(response.to_vec());
            }
            Err(err) => {
                if err.kind() == std::io::ErrorKind::WouldBlock {
                    // Handle WouldBlock by waiting
                    warning("Resource temporarily unavailable, waiting...".to_string());
                    std::thread::sleep(timeout);
                } else {
                    error(format!("Error receiving data 2.: {}", err));
                }
                warning(format!("doubled the timeout to {:?}", timeout));
                re_tries += 1;
                timeout = timeout * 2;
                socket
                    .set_read_timeout(Some(timeout))
                    .expect("set_read_timeout call failed");
            }
        }
    }
    return Err(String::from("too many retries, cant receive data"));
}
