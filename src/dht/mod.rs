use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddr, ToSocketAddrs},
};

mod message;
mod node;
mod routing_table;
mod socket;
mod utils;

use crate::bencode::{
    decoder::{self, Decoder},
    encoder::{encode, Input},
};
use node::Node;
use routing_table::RTable;
use serde_json::json;
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
        // let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 9000));
        let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();

        let node_id = NodeId::new();
        println!("node id:  {:?}", node_id.0);

        // let node_id = node_id.to_hex_string();
        // println!("node id:  {:?}", node_id);

        let mut buf = [0; 1024];

        let bencode_msg = json!({"t":"aa", "y":"q", "q":"ping", "a":{"id":node_id.0}});
        // let bencode_msg =
        //     json!({"id" : node_id, "info_hash" : "6fcf7ef136e73f0fb6186b30fe67d741cc260c5c"});

        let msg = encode(Input::Json(bencode_msg)).unwrap();

        let node_addr = "router.bittorrent.com:6881"
            .to_socket_addrs()
            .unwrap()
            .next()
            .expect("Failed to resolve address");
        println!("bootstrap ip:  {:?}", node_addr);

        loop {
            socket.send_to(&msg, node_addr).await.unwrap();
            println!(
                "{:?} message sent",
                String::from_utf8_lossy(&msg).to_string()
            );

            let (len, node_addr) = socket.recv_from(&mut buf).await.unwrap();
            println!("{:?} bytes received from {:?}", len, node_addr);
            println!("-------------");
            println!("{:?}", buf);
            let ff = &buf[..len];

            println!("{:?}", ff);
            println!("{:?}", String::from_utf8_lossy(&ff).to_string());
            let decoded_res = Decoder::new(ff).start().unwrap().result;
            println!("{:?}", decoded_res);
            println!("-------------");
        }

        Err(String::new())
    }
    // pub async fn bootstrap() -> Result<Dht, String> {
    //     Err(String::new())
    // }
}
