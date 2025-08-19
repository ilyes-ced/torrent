use rand::Rng;
use std::{
    net::{Ipv4Addr, SocketAddr},
    time::{Duration, Instant},
};

use crate::{
    dht::{
        message::{self, Message},
        socket::Socket,
    },
    log::debug,
};

// the documentation says that the nodeID is 160bits long so 20 bytes(u8) but when we do 20 bytes and turn to hex codes each bytes is 2 chars
// the docs site uses nodeID as a string so its ASCII but turning our hex to ASCII usually generates unreadable chars
// so we split in the nodeID lenght of bytes half to get the desired 20char long ASCII nodeID when encoding the bianry to hex

#[derive(Debug, PartialEq, Clone)]
pub struct NodeId(pub [u8; 20]);
impl NodeId {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let mut arr: [u8; 20] = [0; 20];
        rng.fill(&mut arr);
        NodeId(arr)
    }
    // pub fn to_hex_string(&self) -> String {
    //     let mut hex_string = String::with_capacity(20 * 2);
    //     for byte in self.0 {
    //         use std::fmt::Write;
    //         write!(&mut hex_string, "{:02x}", byte).unwrap();
    //     }
    //     hex_string
    // }
}
////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub enum NodeStatus {
    Bad,
    Questionable,
    Good,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    pub id: NodeId,
    pub addr: SocketAddr,
    pub last_activity: Instant,
    // u32 should be enough
    pub refresh_requests: u32,
}

impl Node {
    pub fn new(id: NodeId, addr: SocketAddr) -> Self {
        Node {
            id,
            addr,
            last_activity: Instant::now(),
            refresh_requests: 0,
        }
    }

    // initlizer empty node
    pub fn init() -> Self {
        Node {
            id: NodeId([0; 20]),
            addr: SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0000)),
            last_activity: Instant::now(),
            refresh_requests: 0,
        }
    }

    pub fn refresh() -> Self {
        // send ping reauest and update the last 2 attributes
        todo!()
    }

    pub fn status(&self) -> NodeStatus {
        // minutes
        match self.last_activity.elapsed() > Duration::from_secs(60 * 15) {
            // questinable
            true => {
                // todo: ping the node, if it responds
                // update the last_activity
                // return nodestatus::good
                // if it doesnt repond return nodeststus::bad
                todo!("ping node to check if its good or bad");
                return NodeStatus::Good;
            }
            false => return NodeStatus::Good,
        }
    }

    pub async fn new_status(&self, socket: &mut Socket, my_node_id: &NodeId) -> NodeStatus {
        // send reauest
        let ping_msg = Message::new(Message::Query(message::Query::Ping(message::Ping {
            id: &my_node_id,
        })))
        .unwrap();
        debug(format!("sending a ping reauest to a new node:  {:?}", self));

        // send ping first
        match socket.send(ping_msg, self.addr).await {
            Ok(res) => {
                debug(format!("new node ping request response:  {:?}", res));
                NodeStatus::Good
            }
            Err(err) => {
                debug(format!(
                    "failed pinging a new node to check status: {}",
                    err
                ));
                NodeStatus::Bad
            }
        }
    }
}
