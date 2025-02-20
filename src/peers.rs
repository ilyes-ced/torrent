use crate::bencode::DecoderResults;
use crate::log::{debug, error, info, warning};
use crate::torrent::{self, Torrent};
use crate::utils::{self, encode_binnary_to_http_chars};
use crate::{bencode::Decoder, constants};
use reqwest::blocking::Client;
use serde_json::Value;
use std::fmt::format;
use std::net::Ipv4Addr;

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
    let url = build_http_url(torrent_data, peer_id).unwrap();
    let result: String = send_request(url).unwrap();

    // there is 2 cases:

    error("test000000000".to_string());
    error("test000000000".to_string());
    let peers = match Decoder::new(result.as_bytes()).start() {
        //      1. tracker can send the peers data as utf8 chars so we decode the bencode and use them directly
        Ok(decoded_resp) => {
            error("test000000000".to_string());
            peers_utf(decoded_resp)
        }
        //      2. tracker will send the peer data as binary so which our benocde decoder cant read as string(utf8 chars) so we need to extract them manually from the response
        Err(err) => {
            error("test111111111".to_string());
            peers_binary(result)
        }
    }?;

    Ok(peers)
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

fn peers_utf(decoded_response: DecoderResults) -> Result<PeersResult, String> {
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

fn peers_binary(result: String) -> Result<PeersResult, String> {
    warning(format!("{}", result));
    let peers_raw = result.split_once("peers").unwrap().1;
    let (s, b) = peers_raw.split_once(":").unwrap();
    let size = s.parse::<usize>().unwrap();
    let bytes = b.as_bytes();

    let mut raw_bytes: Vec<u8> = Vec::new();
    for i in 0..size {
        raw_bytes.push(bytes[i])
    }

    if raw_bytes.len() % 3 != 0 {
        error("wrong format for the peers data".to_string());
        return Err(String::from("wrong format for the peers data"));
    }

    let mut peers: Vec<Peer> = Vec::new();
    for i in 0..(raw_bytes.len() / 6) {
        let peer = Peer {
            ip: Ipv4Addr::new(
                raw_bytes[i * 6 + 0],
                raw_bytes[i * 6 + 1],
                raw_bytes[i * 6 + 2],
                raw_bytes[i * 6 + 3],
            ),
            port: u16::from_be_bytes([raw_bytes[i * 6 + 4], raw_bytes[i * 6 + 5]]),
        };
        peers.push(peer);
    }

    // for interval

    let pt1 = result.split_once("interval").unwrap().1;
    let pt2 = pt1.split_once("e").unwrap().0;
    let pt3 = pt2.split_once("i").unwrap().1;
    debug(format!("+++++++++++++++++++++ {}", pt3));
    let size = pt3.parse::<u64>().unwrap();
    debug(format!("+++++++++++++++++++++ {}", pt3));

    Ok(PeersResult {
        peers,
        interval: size,
    })
}

// unused udp connection with trakcer
// should work but does not
///////////////////////////////////////////////////
///////////////////////////////////////////////////
///////////////////////////////////////////////////
///////////////////////////////////////////////////
///////////////////////////////////////////////////
///////////////////////////////////////////////////

//let url: String = torrent_data.announce.clone();
//let url_parts: Vec<&str> = url.split("/").collect();
//debug(format!("{:?}", url_parts));
//let addr: &str = url_parts[2];
//let remote_host = addr.to_socket_addrs().unwrap().next().unwrap();
//debug(format!("remote_host: {:?}", remote_host));
//
//let mut buf: [u8; 16] = [0; 16];
//let protocol_id = u64::to_be_bytes(41727101980);
//let action = u32::to_be_bytes(0);
//buf[0..8].copy_from_slice(&protocol_id);
//buf[8..12].copy_from_slice(&action);
//buf[12..16].copy_from_slice(&u32::to_be_bytes(utils::new_transaction_id()));
//
//let my_protocol_id = &buf[0..8];
//let my_action = &buf[8..12];
//let my_transaction_id = &buf[12..];
//debug(format!("protocol id:    {:?}", my_protocol_id));
//debug(format!("action:         {:?}", my_action));
//debug(format!("transaction id: {:?}", my_transaction_id));
//debug(format!("{:?}", buf));
//debug(format!("tracker server addr: {:?}", addr));
//
//let socket = UdpSocket::bind(format!("0.0.0.0:{}", PORT)).unwrap();
//socket.connect(remote_host).unwrap();
//
//let res = udp_req(&socket, buf.to_vec()).unwrap();
//debug(format!("result 1: {:?}", res));
//
////let mut buffer = [0; 16];
////let s = socket.recv(&mut buffer).unwrap();
////debug(format!("read {} bytes", s));
////debug(format!("******************{:?}", buffer));
//
//let recv_action = &res[0..4];
//let recv_transaction_id = &res[4..8];
//let recv_connection_id = &res[8..16];
//debug(format!("action:         {:?}", recv_action));
//debug(format!("transaction id: {:?}", recv_transaction_id));
//debug(format!("connection id:  {:?}", recv_connection_id));
//
//if recv_transaction_id == my_transaction_id {
//    info(format!(
//        "transactions ids match: {:?}, {:?}",
//        my_transaction_id, recv_transaction_id
//    ));
//} else {
//    error("transactions ids do not match".to_string());
//}
//
//let announce_transaction_id = u32::to_be_bytes(utils::new_transaction_id());
//debug(format!(
//    "new transaction id : {:?}",
//    announce_transaction_id
//));
//let size: u64 = match &torrent_data.info.files {
//    torrent::FileInfo::Multiple(s) => 0,
//    torrent::FileInfo::Single(s) => *s,
//};
//let mut req_buf = [0; 98];
//req_buf[0..8].copy_from_slice(recv_connection_id); // connection_id
//req_buf[8..12].copy_from_slice(&u32::to_be_bytes(1)); // action
//req_buf[12..16].copy_from_slice(&announce_transaction_id); // transaction_id
//req_buf[16..36].copy_from_slice(&torrent_data.info_hash); // info_hash
//req_buf[36..56].copy_from_slice(&peer_id); // peer_id
//req_buf[56..64].copy_from_slice(&u64::to_be_bytes(0)); // downloaded
//req_buf[64..72].copy_from_slice(&u64::to_be_bytes(size)); // left
//req_buf[72..80].copy_from_slice(&u64::to_be_bytes(0)); // uploaded
//req_buf[80..84].copy_from_slice(&u32::to_be_bytes(0)); // event
//req_buf[84..88].copy_from_slice(&u32::to_be_bytes(0)); // IP
//req_buf[88..92].copy_from_slice(&u32::to_be_bytes(rng().random::<u32>())); // key
//req_buf[92..96].copy_from_slice(&i32::to_be_bytes(-1)); // num_want = -1
//req_buf[96..98].copy_from_slice(&u16::to_be_bytes(PORT)); // port
//debug(format!("req_buf: {:?}", req_buf));
//
//let announce_response = udp_req(&socket, req_buf.to_vec()).unwrap();
//debug(format!("result 2: {:?}", announce_response));
//
////let (amt, _) = socket.recv(&mut announce_response).unwrap();
////debug(format!("read {} bytes", amt));
////debug(format!("announce response: {:?}", announce_response));
//
//if announce_transaction_id == announce_response[4..8] {
//    info(format!(
//        "transactions ids match: {:?}, {:?}",
//        announce_transaction_id,
//        &announce_response[4..8]
//    ));
//} else {
//    error(format!(
//        "transactions ids do not match: {:?}, {:?}",
//        announce_transaction_id,
//        &announce_response[4..8]
//    ));
//}
//
//let action = &announce_response[0..4];
//if action == [0, 0, 0, 3] {
//    error(format!(
//        "error: {}",
//        str::from_utf8(&announce_response[8..]).unwrap()
//    ))
//}
//
//fn udp_req(socket: &UdpSocket, request: Vec<u8>) -> Result<Vec<u8>, String> {
//    let mut retry = true;
//    let mut re_tries: u16 = 0;
//    let mut timeout: Duration = Duration::new(0, INITIAL_TIMEOUT);
//
//    socket
//        .set_read_timeout(Some(timeout))
//        .expect("set_read_timeout call failed");
//
//    while retry {
//        if re_tries == MAX_RETRIES {
//            return Err(String::from("too many retries, cant receive data"));
//        }
//
//        if let Err(e) = socket.send(request.as_slice()) {
//            error(format!("Failed to send request 1.: {}", e));
//        }
//
//        let mut response: [u8; 1024] = [0; 1024];
//
//        match socket.recv(&mut response) {
//            Ok(s) => {
//                retry = false;
//                return Ok(response.to_vec());
//            }
//            Err(err) => {
//                if err.kind() == std::io::ErrorKind::WouldBlock {
//                    // Handle WouldBlock by waiting
//                    warning(format!(
//                        "Resource temporarily unavailable, waiting... | error: {:?}",
//                        err
//                    ));
//                    std::thread::sleep(Duration::from_secs(1)); // Wait a bit
//                    continue;
//                } else {
//                    error(format!("Error receiving data 2.: {}", err));
//                }
//                warning(format!("doubled the timeout to {:?}", timeout));
//                re_tries += 1;
//                timeout = timeout * 2;
//                socket
//                    .set_read_timeout(Some(timeout))
//                    .expect("set_read_timeout call failed");
//            }
//        }
//    }
//    return Err(String::from("too many retries, cant receive data"));
//}
//
