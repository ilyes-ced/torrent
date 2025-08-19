use crate::utils::new_transaction_id;
use crate::{bencode::decoder::Decoder, dht::node::Node, log::error, utils::hex_str_to_binary};
use crate::{
    bencode::encoder::{encode, JsonObj},
    dht::node::NodeId,
};
use serde_json::Value;
use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddr},
};

pub enum Message<'a> {
    // q
    //    ping
    //    find_node
    //    get_peer
    //    announce_peer
    Query(Query<'a>),
    // r
    Response(Response),
    // e
    Error(Error),
}
////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub enum Query<'a> {
    Ping(Ping<'a>),
    FindNode(FindNode<'a>),
    GetPeers(GetPeers<'a>),
    AnnouncePeer(AnnouncePeer<'a>),
}
#[derive(Debug)]
pub struct Ping<'a> {
    pub id: &'a NodeId,
}
pub struct FindNode<'a> {
    pub id: &'a NodeId,
    pub target: &'a NodeId,
}
pub struct GetPeers<'a> {
    pub id: &'a NodeId,
    pub info_hash: [u8; 20],
}
pub struct AnnouncePeer<'a> {
    pub id: &'a NodeId,
    pub info_hash: [u8; 20],
    pub port: Option<u16>,
    pub token: Vec<u8>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Error {
    pub code: u8,
    pub message: String,
}
////////////////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a> Message<'a> {
    pub fn new(msg_type: Message<'a>) -> Result<Vec<u8>, String> {
        match &msg_type {
            Message::Query(query) => match query {
                Query::Ping(ping) => {
                    return Ok(ping_msg(ping)
                        .map_err(|e| format!("failed to build ping message: {}", e))?);
                }

                Query::FindNode(find_node) => {
                    return Ok(find_node_msg(find_node)
                        .map_err(|e| format!("failed to build ping message: {}", e))?);
                }

                Query::GetPeers(get_peers) => {
                    return Ok(find_peers_msg(get_peers)
                        .map_err(|e| format!("failed to build get_peers message: {}", e))?);
                }

                Query::AnnouncePeer(announce_peer) => todo!(),
            },
            Message::Response(response) => todo!(),
            Message::Error(error) => todo!(),
        }
        Err(String::new())
    }
}

fn ping_msg(ping: &Ping) -> Result<Vec<u8>, String> {
    let node_id = ping.id.0;
    let msg = JsonObj::Dict(HashMap::from([
        (String::from("t"), JsonObj::String(new_transaction_id())),
        (String::from("y"), JsonObj::String("q".to_owned())),
        (String::from("q"), JsonObj::String("ping".to_owned())),
        (
            String::from("a"),
            JsonObj::Dict(HashMap::from([(
                String::from("id"),
                JsonObj::Binary(node_id.to_vec()),
            )])),
        ),
    ]));
    let res = encode(msg).unwrap();
    return Ok(res);
}

fn find_peers_msg(get_peers: &GetPeers) -> Result<Vec<u8>, String> {
    let node_id = get_peers.id.0;
    let info_hash = get_peers.info_hash;
    let msg = JsonObj::Dict(HashMap::from([
        (String::from("t"), JsonObj::String(new_transaction_id())),
        (String::from("y"), JsonObj::String("q".to_owned())),
        (String::from("q"), JsonObj::String("get_peers".to_owned())),
        (
            String::from("a"),
            JsonObj::Dict(HashMap::from([
                (String::from("id"), JsonObj::Binary(node_id.to_vec())),
                (
                    String::from("info_hash"),
                    JsonObj::Binary(info_hash.to_vec()),
                ),
            ])),
        ),
    ]));
    let res = encode(msg).unwrap();
    return Ok(res);
}

fn find_node_msg(get_peers: &FindNode) -> Result<Vec<u8>, String> {
    let node_id = get_peers.id.0;
    let target = get_peers.target.0;
    let msg = JsonObj::Dict(HashMap::from([
        (String::from("t"), JsonObj::String(new_transaction_id())),
        (String::from("y"), JsonObj::String("q".to_owned())),
        (String::from("q"), JsonObj::String("find_node".to_owned())),
        (
            String::from("a"),
            JsonObj::Dict(HashMap::from([
                (String::from("id"), JsonObj::Binary(node_id.to_vec())),
                (String::from("target"), JsonObj::Binary(target.to_vec())),
            ])),
        ),
    ]));
    let res = encode(msg).unwrap();
    return Ok(res);
}
////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct Response {
    pub id: NodeId,
    // could need ipV6 for this one
    // nodeID:ip:port
    pub nodes: Vec<Node>,
    // Only present in responses to GetPeers (IP addresses)
    pub values: Vec<SocketAddr>,
    // Only present in responses to GetPeers
    pub token: Option<Vec<u8>>,
}

impl Response {
    pub async fn decode_response(response_buf: &[u8]) -> Result<Response, String> {
        error(format!("res: {:?}", response_buf));
        let decoded_res = Decoder::new(response_buf).start()?.result;
        error(format!("decoded res: {:?}", decoded_res));

        let json_response: Value = serde_json::from_str(&decoded_res)
            .map_err(|e| format!("failed to decode to json with serde: {}", e))?;

        error(format!("#########################################3"));
        error(format!("response json: {:?}", json_response));
        error(format!("#########################################3"));

        // get nodeID
        let node_id = match json_response["r"]["id"].as_str() {
            Some(node_id) => node_id,
            None => return Err(String::from("reponse didnt have node id")),
        };

        // get nodes addresses in get_peers Query
        let nodes_addrs: Vec<Node> = match json_response["r"]["nodes"].as_str() {
            Some(node_id) => {
                let bytes = hex_str_to_binary(node_id).map_err(|e| {
                    format!("failed to convert ip address binary string to u8: {}", e)
                })?;

                if bytes.len() % 26 != 0 {
                    error("wrong format for the nodes address data".to_string());
                    return Err("wrong format for the nodes address data".to_string());
                }

                let mut nodes: Vec<Node> = Vec::new();
                for i in 0..(bytes.len() / 26) {
                    let id = NodeId(bytes[i * 26..i * 26 + 20].try_into().unwrap());
                    let addr = SocketAddr::new(
                        std::net::IpAddr::V4(Ipv4Addr::new(
                            bytes[i * 26 + 20],
                            bytes[i * 26 + 21],
                            bytes[i * 26 + 22],
                            bytes[i * 26 + 23],
                        )),
                        u16::from_be_bytes([bytes[i * 26 + 24], bytes[i * 26 + 25]]),
                    );
                    nodes.push(Node::new(id, addr));
                }
                nodes
            }
            None => Vec::new(),
        };

        // get values (Peer addresses) in get_peers Query
        // ! untested
        let peers_addrs: Vec<SocketAddr> = match json_response["r"]["values"].as_str() {
            Some(node_id) => {
                let bytes = hex_str_to_binary(node_id).map_err(|e| {
                    format!("failed to convert ip address binary string to u8: {}", e)
                })?;

                if bytes.len() % 6 != 0 {
                    error("wrong format for the nodes address data".to_string());
                    return Err("wrong format for the nodes address data".to_string());
                }

                let mut peers: Vec<SocketAddr> = Vec::new();
                for i in 0..(bytes.len() / 6) {
                    let peer = SocketAddr::new(
                        std::net::IpAddr::V4(Ipv4Addr::new(
                            bytes[i * 0],
                            bytes[i * 1],
                            bytes[i * 2],
                            bytes[i * 3],
                        )),
                        u16::from_be_bytes([bytes[i * 6 + 4], bytes[i * 6 + 5]]),
                    );
                    peers.push(peer);
                }
                peers
            }
            None => Vec::new(),
        };

        // * this ip retreival is useless because for example in ping query it returns our own ip address
        // let ip = match json_response["ip"].as_str() {
        //     Some(ip) => {
        //         let ip_bytes = hex_str_to_binary(ip).map_err(|e| {
        //             format!("failed to convert ip address binary string to u8: {}", e)
        //         })?;
        //         error(format!("ip address bytes  : {:#?}", ip));
        //         let addr = SocketAddr::new(
        //             std::net::IpAddr::V4(Ipv4Addr::new(
        //                 ip_bytes[0],
        //                 ip_bytes[1],
        //                 ip_bytes[2],
        //                 ip_bytes[3],
        //             )),
        //             u16::from_be_bytes([ip_bytes[4], ip_bytes[5]]),
        //         );
        //         warning(format!("ip address: {:#?}", addr));
        //     }
        //     // shouldnt be error
        //     None => return Err(String::from("reponse didnt have node ip")),
        // };

        let binary_node_id = hex_str_to_binary(node_id)
            .map_err(|e| format!("failed to convert hex nodeID to binary: {}", e))?;

        let buf: [u8; 20] = binary_node_id[0..20]
            .try_into()
            .map_err(|e| format!("failed to convert vec to [u8; 20]: {}", e))?;

        Ok(Response {
            id: NodeId(buf),
            nodes: nodes_addrs,
            values: peers_addrs,
            token: Default::default(),
        })
    }
}
