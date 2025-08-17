use std::{collections::VecDeque, time::Instant};

use crate::{
    dht::node::{Node, NodeStatus},
    log::debug,
};

pub const MAX_BUCKET_SIZE: usize = 8;
pub const MAX_BUCKETS: usize = 160;

#[derive(Debug)]
pub struct Bucket {
    pub nodes: VecDeque<Node>,
    pub last_activity: Instant,
}

impl Bucket {
    pub fn new() -> Self {
        Bucket {
            nodes: VecDeque::new(),
            last_activity: Instant::now(),
        }
    }
    pub fn from(nodes: Vec<Node>) -> Self {
        Bucket {
            nodes: VecDeque::from(nodes),
            last_activity: Instant::now(),
        }
    }

    pub fn add_node(&mut self, node: Node) -> bool {
        // test
        // if status bad return
        // if alredy in table return

        /*
            if bucket is not full we add directly to the end of the bucket
            if bucket is full = MAX_BUCKET_SIZE
                we check the status of oldest nodes
                    if any node is questionable (last activity > 15mins) we refresh it
                        if node doesnt repond it is bad and is replaced
                        if no nodes are bad discard the new node
        */

        debug("=========================================".to_string());
        debug(format!("bucket length:  {:?}", self.nodes.len()));
        debug(format!(
            "bucket last_activity:  {:?}",
            self.last_activity.elapsed()
        ));

        if self.nodes.contains(&node) {
            // move it to the start of the list
            self.nodes.retain(|x| x != &node);
            self.nodes.push_front(node);
            return true;
        }

        if self.nodes.len() < MAX_BUCKET_SIZE {
            // bucket not full push directly
            self.nodes.push_front(node);
            return true;
        }

        // here we check if we split the buckets
        // or return false to split in the routing table instead (better)

        // ? order is reversed because we need to check the oldest nodes first which are at the end of the vector
        for node_ind in (0..self.nodes.len()).rev() {
            match self.nodes[node_ind].status() {
                NodeStatus::Bad => {
                    //replace the bad node
                    self.nodes.remove(node_ind);
                    self.nodes.push_front(node);
                    return true;
                }
                // should do nothing if the checked node is good
                NodeStatus::Good => {}
                // ? should never appear here, because the NODE.status() function send a ping query and returns either Good or Bad
                NodeStatus::Questionable => todo!(),
            };
        }

        // if we reach this point we dont need to add the node and it is discarded
        false
    }
}
