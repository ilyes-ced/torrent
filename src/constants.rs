pub(crate) const MAX_RETRIES: u32 = 8;
pub(crate) const INITIAL_TIMEOUT: u32 = 100000000; // in nanoseconds // set to 100 ms
pub(crate) const IP_ADDR: &str = "0.0.0.0";
pub(crate) const PORT: u16 = 6881;

// messages IDs
pub(crate) const CHOKE: u8 = 0;
pub(crate) const UNCHOKE: u8 = 1;
pub(crate) const INTRESTED: u8 = 2;
pub(crate) const NOTINTRESTED: u8 = 3;
pub(crate) const HAVE: u8 = 4;
pub(crate) const BITFIELD: u8 = 5;
pub(crate) const REQUEST: u8 = 6;
pub(crate) const PIECE: u8 = 7;
pub(crate) const CANCEL: u8 = 8;

// tcp connection timeout
// set to a small value for easier/faster debugging
// maybe set it to 10 in production
pub(crate) const TIMEOUT_DURATION: u64 = 3; // in secods
