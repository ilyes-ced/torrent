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
