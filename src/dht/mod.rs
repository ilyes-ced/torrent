/*
ping node
if works (if not try other bootstrap nodes):
    1. find_node
        id: my_id
        target: my_id
            the remote nodes calculates my_node_id XOR the node_ids he has and
            returns the closest to my node_id

    2. get_peers (lookup)
        calculate my_node_id XOR the returned nodes
        choose K=8 closest Nodes (maybe keep the others because they can be helpfull to other people when we announce)
        sort them by the XOR distance (info_hash XOR new_node_id)

    3. loop
        get_peers
            from the K closest nodes
            with each iteration we should be getting closer until we reach the closest

    4. we stop when the response has "values" in it which is the peers addresses
    or when we can no longer find closer nodes

    5.announce
*/

use std::{
    collections::HashMap,
    hash::Hash,
    net::{Ipv4Addr, SocketAddr, ToSocketAddrs},
};

mod bucket;
mod message;
mod node;
mod routing_table;
mod socket;

use crate::{
    bencode::decoder::Decoder,
    dht::message::Message,
    log::debug,
    utils::{hex_str_to_binary, new_transaction_id},
};
use node::Node;
use routing_table::RoutingTable;
use socket::Socket;
use tokio::net::UdpSocket;

use crate::dht::node::NodeId;

const BOOTSTRAP_NODES: [&str; 4] = [
    "router.bittorrent.com:6881",
    "router.utorrent.com:6881",
    "dht.transmissionbt.com:6881",
    "router.bittorrent.org:6881",
];
#[derive(Debug)]
pub struct Dht {
    my_node: Node,
    routing_table: RoutingTable,
    store: HashMap<String, String>,
    socket: Socket,
}
impl Dht {
    pub async fn new() -> Result<Dht, String> {
        let node_id = NodeId::new();
        debug(format!("node id:  {:?}", node_id.0));

        let trans_id = new_transaction_id();
        debug(format!("trans_id:  {:?}", trans_id));

        let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 9000));
        let my_node = Node::new(node_id, addr);
        let routing_table = RoutingTable::new();
        let store = HashMap::new();
        let socket = Socket::new(addr).await?;

        debug(format!("trans_id:  {:?}", my_node));
        std::thread::sleep(std::time::Duration::from_millis(1000));
        debug(format!(
            "how long the object lived:  {:?}",
            my_node.last_activity.elapsed()
        ));

        Ok(Dht {
            my_node,
            routing_table,
            store,
            socket,
        })

        // todo: replace AA with a randomized transaction id
        // todo: tokens

        // ping query
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
        // let encoded_msg = Message::new(Message::Query(message::Query::GetPeers(
        //     message::GetPeers {
        //         id: node_id,
        //         info_hash: hex_str_to_binary(
        //             "6f cf 7e f1 36 e7 3f 0f b6 18 6b 30 fe 67 d7 41 cc 26 0c 5c",
        //         )
        //         .unwrap()
        //         .try_into()
        //         .unwrap(),
        //     },
        // )))?;
        //
        // debug(format!(
        //     "message to send (text):  {:?}",
        //     String::from_utf8_lossy(&encoded_msg).to_string()
        // ));
        //
        // let node_addr = BOOTSTRAP_NODES[0]
        //     .to_socket_addrs()
        //     .unwrap()
        //     .next()
        //     .expect("Failed to resolve address");
        //
        // debug(format!("bootstrap ip:  {:?}", node_addr));
        //
        // // socket
        // let socket = Socket::new().await?;
        //
        // let res = socket.send(encoded_msg, node_addr).await?;
    }
    pub async fn search(self) -> Result<Dht, String> {
        Err(String::new())
    }

    // in this one we use just one bootstraping node (whichever works first)
    pub async fn bootstrap(&mut self) -> Result<Dht, String> {
        /*
            send ping to first bootstrap node
            recieve response (add timeout with doubling duration)
            if it doesnt respond within the 3rd try try the second node and so on
        */
        let mut bootstrap_ind = 0;

        'bootstraping: loop {
            let ping_msg = Message::new(Message::Query(message::Query::Ping(message::Ping {
                id: &self.my_node.id,
            })))?;
            let find_node_msg = Message::new(Message::Query(message::Query::FindNode(
                message::FindNode {
                    id: &self.my_node.id,
                    target: &self.my_node.id,
                },
            )))?;

            let bootstrap_node_addr = BOOTSTRAP_NODES[bootstrap_ind]
                .to_socket_addrs()
                .unwrap()
                .next()
                .expect("Failed to resolve address");
            debug(format!("bootstrap ip:  {:?}", bootstrap_node_addr));

            // send ping first
            match self.socket.send(ping_msg, bootstrap_node_addr).await {
                Ok(res) => {
                    debug(format!("bootstrap ping request response:  {:?}", res));
                    res
                }
                Err(err) => {
                    debug(format!(
                        "error sending ping request to bootstraping node {}:  {:?}",
                        bootstrap_ind, bootstrap_node_addr
                    ));
                    // repeater logic
                    bootstrap_ind += 1;
                    if bootstrap_ind > 3 {
                        break;
                    }
                    continue 'bootstraping;
                }
            };

            // find_nodes request
            match self.socket.send(find_node_msg, bootstrap_node_addr).await {
                Ok(response) => {
                    // TODO: here add the recieved nodes to the routing table
                    // ping new nodes to make sure they are good nodes
                    debug(format!(
                        "bootstrap find_nodes request response:  {:?}",
                        response
                    ));

                    if response.values.len() > 0 {
                        //we found the peers we need
                    } else if response.nodes.len() > 0 {
                        // add the nodes to buckets
                        for node in response.nodes {
                            let res =
                                self.routing_table
                                    .add(node, self.my_node.id.0)
                                    .map_err(|e| {
                                        format!("failed to add node to routing table: {}", e)
                                    })?;
                        }
                    } else {
                        // not sure what to put here
                    }

                    debug(format!("DHT object:  {:?}", self));

                    todo!("implement thhe adding the recieved nodes to the routing table");

                    break;
                }
                Err(err) => {
                    debug(format!(
                        "error sending find_node request to bootstraping node {}:  {:?}",
                        bootstrap_ind, bootstrap_node_addr
                    ));
                    // repeater logic
                    bootstrap_ind += 1;
                    if bootstrap_ind > 3 {
                        break;
                    }
                    continue 'bootstraping;
                }
            };
        }

        Err(String::from(
            "failed getting response from bootstraping nodes",
        ))
    }
}
