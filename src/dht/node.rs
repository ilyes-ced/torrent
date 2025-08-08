use rand::Rng;
use std::net::SocketAddr;

pub enum NodeStatus {
    Bad,
    Questionable,
    Good,
}

pub struct NodeId(pub(crate) [u8; 20]);

pub struct Node {
    id: NodeId,
    addr: SocketAddr,
}

impl NodeId {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let mut arr: [u8; 20] = [0; 20];
        rng.fill(&mut arr);
        NodeId(arr)
    }

    pub fn to_hex_string(self: Self) -> String {
        let mut hex_string = String::with_capacity(20 * 2);
        for byte in self.0 {
            use std::fmt::Write;
            write!(&mut hex_string, "{:02x}", byte).unwrap();
        }
        hex_string
    }
}
