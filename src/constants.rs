// commented out messages are not needed
#[derive(Debug)]
pub enum MsgId {
    // CHOKE = 0,
    Unchoke = 1,
    Interested = 2,
    // NOTINTRESTED = 3,
    Have = 4,
    // BITFIELD = 5,
    Request = 6,
    Piece = 7,
    // CANCEL = 8,
}
impl MsgId {
    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

// tcp connection timeout
// set to a small value for easier/faster debugging
// maybe set it to 10 in production
pub(crate) const TIMEOUT_DURATION: u64 = 5; // in seconds

pub(crate) const PORT: u16 = 6881;

// max number of pieces requested from one client
pub(crate) const MAX_BACKLOG: u8 = 5;

// 16384 // 65535
//{https://wiki.theory.org/BitTorrentSpecification#request:_.3Clen.3D0013.3E.3Cid.3D6.3E.3Cindex.3E.3Cbegin.3E.3Clength.3E}
pub(crate) const MAX_BLOCK_SIZE: u16 = 16384;

// pub(crate) const MAX_RETRIES: u16 = 8;
// pub(crate) const INITIAL_TIMEOUT: u32 = 100000000; // in nanoseconds // set to 100 ms
