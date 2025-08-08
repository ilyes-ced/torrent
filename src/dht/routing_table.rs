use crate::dht::node::Node;

pub const MAX_BUCKET_SIZE: usize = 8;

pub struct Bucket {
    nodes: [Node; MAX_BUCKET_SIZE],
}

pub struct RTable {
    buckets: Vec<Bucket>,
}
