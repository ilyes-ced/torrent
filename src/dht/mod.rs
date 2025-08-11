use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddr, ToSocketAddrs},
};

mod message;
mod node;
mod routing_table;
mod socket;

use crate::{
    bencode::decoder::Decoder, dht::message::Message, log::debug, utils::hex_str_to_binary,
};
use node::Node;
use routing_table::RTable;
use socket::Socket;
use tokio::net::UdpSocket;

use crate::dht::node::NodeId;

const BOOTSTRAP_NODES: [&str; 4] = [
    "router.bittorrent.com:6881",
    "router.utorrent.com:6881",
    "dht.transmissionbt.com:6881",
    "router.bittorrent.org:6881",
];

pub struct Dht {
    my_node: Node,
    routing_table: RTable,
    store: HashMap<String, String>,
    socket: Socket,
}
impl Dht {
    pub async fn new() -> Result<Dht, String> {
        let node_id = NodeId::new();
        debug(format!("node id:  {:?}", node_id.0));

        // todo: replace AA with a randomized transaction id
        // todo: tokens

        // ping auery
        // let encoded_msg = Message::new(Message::Query(message::Query::Ping(message::Ping {
        //     id: node_id,
        // })))?;

        // find node
        // let encoded_msg = Message::new(Message::Query(message::Query::FindNode(
        //     message::FindNode {
        //         id: node_id,
        //         target: NodeId([
        //             108, 158, 91, 66, 230, 193, 202, 47, 42, 111, 0, 228, 251, 47, 112, 118, 43,
        //             224, 195, 47,
        //         ]),
        //     },
        // )))?;

        // get peers
        let encoded_msg = Message::new(Message::Query(message::Query::GetPeers(
            message::GetPeers {
                id: node_id,
                info_hash: hex_str_to_binary(
                    "6f cf 7e f1 36 e7 3f 0f b6 18 6b 30 fe 67 d7 41 cc 26 0c 5c",
                )
                .unwrap()
                .try_into()
                .unwrap(),
            },
        )))?;

        debug(format!(
            "message to send (text):  {:?}",
            String::from_utf8_lossy(&encoded_msg).to_string()
        ));

        let node_addr = BOOTSTRAP_NODES[0]
            .to_socket_addrs()
            .unwrap()
            .next()
            .expect("Failed to resolve address");

        debug(format!("bootstrap ip:  {:?}", node_addr));

        // socket
        let socket = Socket::new().await?;

        let res = socket.send(encoded_msg, node_addr).await?;

        Err(String::new())
    }
    // pub async fn bootstrap() -> Result<Dht, String> {
    //     Err(String::new())
    // }
}
