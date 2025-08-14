use crate::dht::node::Node;

pub const MAX_BUCKET_SIZE: usize = 8;

pub struct Bucket {
    nodes: Vec<Node>,
}

pub struct RoutingTable {
    buckets: Vec<Bucket>,
}

impl RoutingTable {
    pub fn new() -> Self {
        RoutingTable {
            buckets: Vec::new(),
        }
    }
}
