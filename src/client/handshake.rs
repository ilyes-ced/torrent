pub struct Handshake {
    pub protocol_id: String,
    pub info_hash: [u8; 20],
    pub peer_id: [u8; 20],
}

impl Handshake {
    pub fn new(info_hash: [u8; 20], peer_id: [u8; 20]) -> Handshake {
        Handshake {
            protocol_id: String::from("BitTorrent protocol"),
            info_hash,
            peer_id,
        }
    }
    pub fn create_handshake(&self) -> [u8; 68] {
        let mut buffer: [u8; 68] = [0; 68];
        buffer[0..1].copy_from_slice(&[19]);
        buffer[1..20].copy_from_slice("BitTorrent protocol".as_bytes());
        buffer[20..28].copy_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]);
        buffer[28..48].copy_from_slice(&self.info_hash);
        buffer[48..68].copy_from_slice(&self.peer_id);
        buffer
    }
}

pub fn read_handshake(handshake: [u8; 68]) -> Result<Handshake, String> {
    Ok(Handshake {
        protocol_id: String::from_utf8_lossy(&handshake[1..20]).to_string(),
        info_hash: handshake[28..48].try_into().unwrap(),
        peer_id: handshake[48..68].try_into().unwrap(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handshake_creation() {
        let info_hash: [u8; 20] = [1; 20];
        let peer_id: [u8; 20] = [2; 20];
        let handshake = Handshake::new(info_hash, peer_id);

        assert_eq!(handshake.protocol_id, "BitTorrent protocol");
        assert_eq!(handshake.info_hash, info_hash);
        assert_eq!(handshake.peer_id, peer_id);
    }

    #[test]
    fn create_handshake_buffer() {
        let info_hash: [u8; 20] = [1; 20];
        let peer_id: [u8; 20] = [2; 20];
        let handshake = Handshake::new(info_hash, peer_id);
        let buffer = handshake.create_handshake();

        assert_eq!(buffer[0], 19);
        assert_eq!(&buffer[1..20], "BitTorrent protocol".as_bytes());
        assert_eq!(&buffer[28..48], &info_hash);
        assert_eq!(&buffer[48..68], &peer_id);
    }

    #[test]
    fn read_handshake_test() {
        let info_hash: [u8; 20] = [3; 20];
        let peer_id: [u8; 20] = [4; 20];
        let handshake = Handshake::new(info_hash, peer_id);
        let handshake_buffer = handshake.create_handshake();

        let read_handshake_result = read_handshake(handshake_buffer);

        assert!(read_handshake_result.is_ok());

        let read_handshake = read_handshake_result.unwrap();

        assert_eq!(read_handshake.protocol_id, "BitTorrent protocol");
        assert_eq!(read_handshake.info_hash, info_hash);
        assert_eq!(read_handshake.peer_id, peer_id);
    }
}
