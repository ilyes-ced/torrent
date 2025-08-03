use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
};

mod node;
mod routing_table;
mod rpc;

use node::Node;
use routing_table::RTable;
use rpc::Rpc;

const BOOTSTRAP_NODES: [&str; 4] = [
    "router.bittorrent.com:6881",
    "router.utorrent.com:6881",
    "dht.transmissionbt.com:6881",
    "router.bittorrent.org:6881",
];
const IP_ADDR: &str = "192.168.1.1:9090";

pub struct Dht {
    routing_table: RTable,
    store: HashMap<String, String>,
    my_node: Node,
    rpc_con: Rpc,
}

impl Dht {
    // mew node
    pub fn new() -> Result<Dht, String> {
        Err(String::new())
    }
}
