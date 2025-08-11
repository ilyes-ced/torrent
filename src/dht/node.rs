use rand::Rng;
use std::net::SocketAddr;

// the documentation says that the nodeID is 160bits long so 20 bytes(u8) but when we do 20 bytes and turn to hex codes each bytes is 2 chars
// the docs site uses nodeID as a string so its ASCII but turning our hex to ASCII usually generates unreadable chars
// so we split in the nodeID lenght of bytes half to get the desired 20char long ASCII nodeID when encoding the bianry to hex

#[derive(Debug)]
pub struct NodeId(pub(crate) [u8; 20]);
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
pub enum NodeStatus {
    Bad,
    Questionable,
    Good,
}
#[derive(Debug)]
pub struct Node {
    pub id: NodeId,
    pub addr: SocketAddr,
}
