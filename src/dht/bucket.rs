use std::time::Instant;

use crate::{
    dht::node::{Node, NodeStatus},
    log::debug,
};

pub const MAX_BUCKET_SIZE: usize = 8;
pub const MAX_BUCKETS: usize = 160;

#[derive(Debug)]
pub struct Bucket {
    pub nodes: Vec<Node>,
    pub last_activity: Instant,
}

impl Bucket {
    pub fn new() -> Self {
        Bucket {
            nodes: Vec::new(),
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

        if self.nodes.len() < MAX_BUCKET_SIZE {
            // here bucket was not full
            self.nodes.push(node);
            // check older nodes
            for node in &self.nodes {
                match node.status() {
                    NodeStatus::Bad => todo!(),
                    NodeStatus::Good => todo!(),
                    // should appear here
                    NodeStatus::Questionable => todo!(),
                };
            }
        } else {
            // here split the buckets
            // or return false to split in the routing table instead (better)
        }

        false
    }
}
