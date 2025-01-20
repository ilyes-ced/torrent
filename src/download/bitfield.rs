#[derive(Debug)]
pub struct Bitfield {
    bits: Vec<u8>, // Use a Vec<u8> to store the bits
}

impl Bitfield {
    // Creates a new Bitfield with a specified size (in bits)
    pub fn new(size: usize) -> Self {
        let byte_size = (size + 7) / 8; // Calculate the number of bytes needed
        Bitfield {
            bits: vec![0; byte_size], // Initialize with zeros
        }
    }

    // Checks if the bit at the specified index is set
    pub fn has_piece(&self, index: usize) -> bool {
        let byte_index = index / 8;
        let offset = index % 8;

        if byte_index >= self.bits.len() {
            return false; // Index out of bounds
        }

        (self.bits[byte_index] >> (7 - offset)) & 1 != 0
    }

    // Sets the bit at the specified index
    pub fn set_piece(&mut self, index: usize) {
        let byte_index = index / 8;
        let offset = index % 8;

        if byte_index >= self.bits.len() {
            return; // Index out of bounds, handle as needed (e.g., panic or ignore)
        }

        self.bits[byte_index] |= 1 << (7 - offset);
    }
}

fn main() {
    let mut bf = Bitfield::new(16); // Create a Bitfield with space for 16 bits

    bf.set_piece(3); // Set the bit at index 3
    bf.set_piece(10); // Set the bit at index 10

    println!("Bitfield: {:?}", bf);
    println!("Has piece at index 3: {}", bf.has_piece(3)); // Should print true
    println!("Has piece at index 4: {}", bf.has_piece(4)); // Should print false
    println!("Has piece at index 10: {}", bf.has_piece(10)); // Should print true
}
