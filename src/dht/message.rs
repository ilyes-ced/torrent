use crate::{
    bencode::decoder::Decoder,
    log::{debug, warning},
    utils::hex_str_to_binary,
};
use crate::{
    bencode::encoder::{encode, JsonObj},
    dht::node::NodeId,
};
use serde_json::Value;
use std::{collections::HashMap, net::SocketAddr};

pub enum Message {
    // q
    //    ping
    //    find_node
    //    get_peer
    //    announce_peer
    Query(Query),
    // r
    Response(Response),
    // e
    Error(Error),
}
////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub enum Query {
    Ping(Ping),
    FindNode(FindNode),
    GetPeers(GetPeers),
    AnnouncePeer(AnnouncePeer),
}
#[derive(Debug)]
pub struct Ping {
    pub id: NodeId,
}
pub struct FindNode {
    pub id: NodeId,
    pub target: NodeId,
}
pub struct GetPeers {
    pub id: NodeId,
    pub info_hash: [u8; 20],
}
pub struct AnnouncePeer {
    pub id: NodeId,
    pub info_hash: [u8; 20],
    pub port: Option<u16>,
    pub token: Vec<u8>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct Response {
    pub id: NodeId,

    // TODO: fix this
    // pub nodes_v4: Vec<ip>,
    // pub nodes_v6: Vec<ip>,

    // Only present in responses to GetPeers
    pub addresses: Vec<SocketAddr>,
    // Only present in responses to GetPeers
    pub token: Option<Vec<u8>>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Error {
    pub code: u8,
    pub message: String,
}
////////////////////////////////////////////////////////////////////////////////////////////////////////////

impl Message {
    pub fn new(msg_type: Message) -> Result<Vec<u8>, String> {
        match &msg_type {
            Message::Query(query) => match query {
                Query::Ping(ping) => {
                    let node_id = match &msg_type {
                        Message::Query(Query::Ping(ping)) => {
                            println!("Node ID: {:02x?}", ping.id);
                            ping.id.0
                        }
                        _ => {
                            return Err(String::from("Not a ping message"));
                        }
                    };

                    debug(format!("query ping:  {:?}", ping));
                    debug(format!("node_id:  {:?}", node_id));

                    let msg = JsonObj::Dict(HashMap::from([
                        (String::from("t"), JsonObj::String("aa".to_owned())),
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
                    println!("{:#?}", msg);
                    return Ok(encode(msg).unwrap());
                }
                Query::FindNode(find_node) => todo!(),
                Query::GetPeers(get_peers) => todo!(),
                Query::AnnouncePeer(announce_peer) => todo!(),
            },
            Message::Response(response) => todo!(),
            Message::Error(error) => todo!(),
        }
        Err(String::new())
    }
}

impl Response {
    pub async fn decode_response(response_buf: &[u8]) -> Result<Response, String> {
        warning("-------------".to_string());
        // warning(format!("Recieved binary response: {:?}", response_buf));
        // warning(format!(
        //     "Recieved string response: {:?}",
        //     String::from_utf8_lossy(&response_buf).to_string()
        // ));

        let decoded_res = Decoder::new(response_buf).start()?.result;
        // warning(format!("Recieved decoded response: {:?}", decoded_res));

        let json_response: Value = serde_json::from_str(&decoded_res)
            .map_err(|e| format!("failed to decode to json with serde: {}", e))?;
        warning(format!("Recieved json response: {:#?}", json_response));
        warning(format!("Recieved json id: {:#?}", json_response["r"]["id"]));

        let sr = &json_response["r"]["id"].as_str().unwrap();
        warning(format!("Recieved json id: {:#?}", sr));

        let binary = hex_str_to_binary(sr);
        warning(format!(
            "Recieved json id from hex to binary: {:#?}",
            binary
        ));

        if let Some(node_id) = json_response["r"]["id"].as_str() {
            println!("Extracted node ID: {}", node_id);
        } else {
            println!("Node ID not found or not a string");
        }
        warning(format!("got node id from some : {:#?}", node_id));

        let node_id = json_response["r"]["id"].as_str() {
            Some(node_id) => node_id,
            None => return Err(String::new())
        }
        warning("-------------".to_string());

        Err(String::from("test test"))
    }
}
