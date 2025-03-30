#[derive(Debug)]
pub struct Bitfield {
    bytes: Vec<u8>,
}

impl Bitfield {
    pub fn new(bytes: Vec<u8>) -> Self {
        Bitfield { bytes }
    }

    pub fn has_piece(&self, index: usize) -> bool {
        let byte_index = index / 8;
        let offset = index % 8;

        if byte_index >= self.bytes.len() {
            return false;
        }

        (self.bytes[byte_index] >> (7 - offset)) & 1 != 0
    }

    pub fn set_piece(&mut self, index: usize) {
        let byte_index = index / 8;
        let offset = index % 8;

        if byte_index >= self.bytes.len() {
            return;
        }

        self.bytes[byte_index] |= 1 << (7 - offset);
    }
}

impl std::fmt::Display for Bitfield {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in &self.bytes {
            write!(f, "{:08b} ", byte)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_piece() {
        let bitfield = Bitfield::new(vec![0b10000000, 0b10000001]);

        assert_eq!(bitfield.has_piece(0), true);
        assert_eq!(bitfield.has_piece(7), false);
        assert_eq!(bitfield.has_piece(8), true);
        assert_eq!(bitfield.has_piece(15), true);
        assert_eq!(bitfield.has_piece(16), false);

        assert_eq!(bitfield.has_piece(17), false);
    }

    #[test]
    fn set_piece() {
        let mut bitfield = Bitfield::new(vec![0b00000000, 0b00000000]);

        bitfield.set_piece(0);
        assert_eq!(bitfield.has_piece(0), true);

        bitfield.set_piece(7);
        assert_eq!(bitfield.has_piece(7), true);

        bitfield.set_piece(8);
        assert_eq!(bitfield.has_piece(8), true);

        bitfield.set_piece(0);
        assert_eq!(bitfield.has_piece(0), true);

        assert_eq!(bitfield.has_piece(1), false);
    }

    #[test]
    fn out_of_bounds_set() {
        let mut bitfield = Bitfield::new(vec![0b00000000]);

        bitfield.set_piece(8);

        assert_eq!(bitfield.has_piece(8), false);
    }
}
