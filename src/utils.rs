use rand::distr::{Alphanumeric, SampleString};
use sha1::{Digest, Sha1};

pub fn new_peer_id() -> [u8; 20] {
    //"-IT0001-"+12 random chars
    let mut id = [0; 20];
    id[0..8].copy_from_slice("-IT0001-".as_bytes());
    let string = Alphanumeric.sample_string(&mut rand::rng(), 12);
    id[8..20].copy_from_slice(string.as_bytes());
    id
}

pub fn encode_binnary_to_http_chars(input: [u8; 20]) -> String {
    let mut return_string = String::new();
    for byte in input {
        return_string.push('%');
        return_string.push_str(&format!("{:02x}", byte));
    }
    return_string
}

pub fn concat(vec: &Vec<u8>) -> usize {
    let mut acc: usize = 0;
    for elem in vec {
        acc *= 10;
        match elem {
            b'0' => acc += 0,
            b'1' => acc += 1,
            b'2' => acc += 2,
            b'3' => acc += 3,
            b'4' => acc += 4,
            b'5' => acc += 5,
            b'6' => acc += 6,
            b'7' => acc += 7,
            b'8' => acc += 8,
            b'9' => acc += 9,
            _ => {
                // impossible i think
            }
        }
    }
    acc
}

pub fn check_integrity(buf: &Vec<u8>, expected_hash: [u8; 20]) -> Result<bool, String> {
    let mut hasher = Sha1::new();
    hasher.update(buf);
    let hash = hasher.finalize();
    if hash == expected_hash.into() {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha1::{Digest, Sha1};

    #[test]
    fn test_new_peer_id() {
        let peer_id = new_peer_id();
        // Assert that the peer ID has a length of 20
        assert_eq!(peer_id.len(), 20);
        // Assert that the first 8 bytes match the expected "-IT0001-"
        assert_eq!(&peer_id[0..8], b"-IT0001-");
        // Assert that the remaining bytes are alphanumeric (random characters)
        for &byte in &peer_id[8..] {
            assert!(
                byte.is_ascii_alphanumeric(),
                "Invalid byte in peer ID: {}",
                byte
            );
        }
    }

    #[test]
    fn test_encode_binary_to_http_chars() {
        let input = [
            0x12, 0x34, 0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x12,
            0x34, 0x56, 0x78, 0x90, 0x12, 0x34,
        ];
        let result = encode_binnary_to_http_chars(input);
        for byte in input.iter() {
            assert!(result.contains(&format!("%{:02x}", byte)));
        }
    }

    #[test]
    fn test_concat() {
        let vec = vec![b'1', b'2', b'3', b'4', b'5'];
        let result = concat(&vec);
        assert_eq!(result, 12345);
    }

    #[test]
    fn test_check_integrity_success() {
        let buf = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ];
        let expected_hash = {
            let mut hasher = Sha1::new();
            hasher.update(&buf);
            hasher.finalize().into()
        };

        let result = check_integrity(&buf, expected_hash);
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn test_check_integrity_failure() {
        let buf = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ];
        let wrong_hash = [0u8; 20];

        let result = check_integrity(&buf, wrong_hash);
        assert_eq!(result, Ok(false));
    }
}
