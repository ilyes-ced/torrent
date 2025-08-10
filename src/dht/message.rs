use crate::dht::node::NodeId;

pub enum Message {
    // q
    //    ping
    //    find_node
    //    get_peer
    //    announce_peer
    Query(Query),
    // r
    Response(Response),
    // e
    Error(Error),
}
////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub enum Query {
    Ping(Ping),
    FindNode(FindNode),
    GetPeers(GetPeers),
    AnnouncePeer(AnnouncePeer),
}
pub struct Ping {
    pub id: NodeId,
}
pub struct FindNode {
    pub id: NodeId,
    pub target: NodeId,
}
pub struct GetPeers {
    pub id: NodeId,
    pub info_hash: [u8; 20],
}
pub struct AnnouncePeer {
    pub id: NodeId,
    pub info_hash: [u8; 20],
    pub port: Option<u16>,
    pub token: Vec<u8>,
}
////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Response {
    pub id: NodeId,

    // TODO: fix this
    // pub nodes_v4: Vec<ip>,
    // pub nodes_v6: Vec<ip>,

    // Only present in responses to GetPeers
    pub values: Vec<SocketAddr>,
    // Only present in responses to GetPeers
    pub token: Option<Vec<u8>>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Error {
    pub code: u8,
    pub message: String,
}
