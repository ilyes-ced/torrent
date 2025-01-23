#[derive(Debug)]
pub struct Bitfield {
    bytes: Vec<u8>,
}

impl Bitfield {
    pub fn new(bytes: Vec<u8>) -> Self {
        Bitfield { bytes: bytes }
    }

    pub fn has_piece(&self, index: usize) -> bool {
        false
    }

    pub fn set_piece(&mut self, index: usize) {}
}
