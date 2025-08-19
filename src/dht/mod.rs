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
    log::{debug, error, info},
    utils::{hex_str_to_binary, new_transaction_id, xor_distance},
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

        let leading_zeros =
            crate::utils::count_leading_zeros(node_id.0.try_into().unwrap()) as usize;
        debug(format!("my node id leading zeros:  {:?}", leading_zeros));

        let trans_id = new_transaction_id();
        debug(format!("trans_id:  {:?}", trans_id));

        let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 9000));
        let routing_table = RoutingTable::new(node_id.clone());
        let my_node = Node::new(node_id, addr);
        let store = HashMap::new();
        let socket = Socket::new(addr).await?;

        Ok(Dht {
            my_node,
            routing_table,
            store,
            socket,
        })
    }

    // in this one we use just one bootstraping node (whichever works first)
    pub async fn bootstrap(&mut self) -> Result<(), String> {
        /*
            send ping to first bootstrap node
            recieve response (add timeout with doubling duration)
            if it doesnt respond within the 3rd try try the second node and so on
        */

        let bootstrap_node_addr = BOOTSTRAP_NODES[0]
            .to_socket_addrs()
            .unwrap()
            .next()
            .expect("Failed to resolve address");
        debug(format!("bootstrap ips:  {:?}", bootstrap_node_addr));

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

            //* send ping first
            match self.socket.send(ping_msg, bootstrap_node_addr).await {
                Ok(res) => {
                    debug(format!("bootstrap ping request response:  {:?}", res));
                    res
                }
                Err(err) => {
                    debug(format!(
                        "error sending ping request to bootstraping node {}:  {:?} >>> {}",
                        bootstrap_ind, bootstrap_node_addr, err
                    ));
                    // repeater logic
                    bootstrap_ind += 1;
                    if bootstrap_ind > 3 {
                        break;
                    }
                    continue 'bootstraping;
                }
            };

            //* find_nodes request
            match self.socket.send(find_node_msg, bootstrap_node_addr).await {
                Ok(response) => {
                    if response.values.len() > 0 {
                        //? we found the peers we need
                        // TODO: send them to peers reciever thread (planned)
                    } else if response.nodes.len() > 0 {
                        //? ping recieved nodes to make sure they are good than keep the 8 closest to us
                        let mut good_nodes: Vec<Node> = Vec::new();

                        for node in response.nodes {
                            error(format!("checking node status >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>"));
                            match node.new_status(&mut self.socket, &self.my_node.id).await {
                                node::NodeStatus::Good => good_nodes.push(node),
                                // dont care discard
                                _ => {}
                            }
                        }

                        // sort by XOR distance
                        good_nodes.sort_by_key(|node| xor_distance(self.my_node.id.0, node.id.0));

                        for node in good_nodes.into_iter().take(8).collect::<Vec<Node>>() {
                            self.routing_table.add(node).map_err(|e| {
                                format!("failed to add node to routing table: {}", e)
                            })?;
                        }
                    } else {
                        // not sure what to put here
                    }

                    break;
                }
                Err(_) => {
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

        Ok(())
    }

    // search recursivelly untill we find our target
    pub async fn lookup(&mut self) -> Result<(), String> {
        loop {
            error(format!(
                "starting lookup ................................................................. bucket index: {}", self.routing_table.buckets.len() - 1
            ));
            std::thread::sleep(std::time::Duration::from_secs(2));

            let ping_msg = Message::new(Message::Query(message::Query::Ping(message::Ping {
                id: &self.my_node.id,
            })))?;
            let find_node_msg = Message::new(Message::Query(message::Query::FindNode(
                message::FindNode {
                    id: &self.my_node.id,
                    target: &self.my_node.id,
                },
            )))?;

            // i dont like the clone
            let nodes = self.routing_table.buckets[self.routing_table.buckets.len() - 1]
                .nodes
                .clone();

            for node in nodes {
                // i dont like the clone
                debug(format!("=================="));
                debug(format!("=================="));
                debug(format!("=================="));
                debug(format!("sending reauest to node: {:?}", node));

                //? no need to send ping reauest because its already sent when the node was added
                // send ping first
                // let res = match self.socket.send(ping_msg.clone(), node.addr).await {
                //     Ok(res) => {
                //         debug(format!("node ping request response:  {:?}", res));
                //         Ok(())
                //     }
                //     Err(_) => {
                //         debug(format!("error sending ping request to a node "));
                //         Err(())
                //     }
                // };

                match self.socket.send(find_node_msg.clone(), node.addr).await {
                    Ok(response) => {
                        if response.values.len() > 0 {
                            //? we found the peers we need
                            // TODO: send them to peers reciever thread (planned)
                            debug(format!("finally found peers: {:?}", response));
                            panic!("finally found peers")
                        } else if response.nodes.len() > 0 {
                            //? ping recieved nodes to make sure they are good than keep the 8 closest to us
                            let mut good_nodes: Vec<Node> = Vec::new();

                            for node in response.nodes {
                                match node.new_status(&mut self.socket, &self.my_node.id).await {
                                    node::NodeStatus::Good => good_nodes.push(node),
                                    // dont care discard
                                    _ => {}
                                }
                            }

                            // sort by XOR distance
                            good_nodes
                                .sort_by_key(|node| xor_distance(self.my_node.id.0, node.id.0));

                            for node in good_nodes.into_iter().take(8).collect::<Vec<Node>>() {
                                self.routing_table.add(node).map_err(|e| {
                                    format!("failed to add node to routing table: {}", e)
                                })?;
                            }
                        } else {
                            // not sure what to put here
                        }

                        break;
                    }
                    Err(e) => {
                        debug(format!("error sending find_node request to a node: {}", e));
                    }
                };
            }
        }
        Ok(())
    }
}
