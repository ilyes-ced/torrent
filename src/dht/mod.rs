use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddr, ToSocketAddrs},
};

mod message;
mod node;
mod routing_table;
mod socket;

use crate::{bencode::decoder::Decoder, dht::message::Message, log::debug};
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
const IP_ADDR: &str = "192.168.1.1:9090";

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

        let encoded_msg = Message::new(Message::Query(message::Query::Ping(message::Ping {
            id: node_id,
        })))?;

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
