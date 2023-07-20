use rand::distributions::{Alphanumeric, DistString};

pub fn transform_u16_to_array_of_u8(x: u16) -> [u8; 2] {
    let b1: u8 = ((x >> 8) & 0xff) as u8;
    let b2: u8 = (x & 0xff) as u8;
    [b1, b2]
}
pub fn transform_u32_to_array_of_u8(x: u32) -> [u8; 4] {
    let b1: u8 = ((x >> 24) & 0xff) as u8;
    let b2: u8 = ((x >> 16) & 0xff) as u8;
    let b3: u8 = ((x >> 8) & 0xff) as u8;
    let b4: u8 = (x & 0xff) as u8;
    [b1, b2, b3, b4]
}
pub fn transform_u64_to_array_of_u8(x: u64) -> [u8; 8] {
    let b1: u8 = ((x >> 56) & 0xff) as u8;
    let b2: u8 = ((x >> 48) & 0xff) as u8;
    let b3: u8 = ((x >> 40) & 0xff) as u8;
    let b4: u8 = ((x >> 32) & 0xff) as u8;
    let b5: u8 = ((x >> 24) & 0xff) as u8;
    let b6: u8 = ((x >> 16) & 0xff) as u8;
    let b7: u8 = ((x >> 8) & 0xff) as u8;
    let b8: u8 = (x & 0xff) as u8;
    [b1, b2, b3, b4, b5, b6, b7, b8]
}

pub fn new_peer_id() -> [u8; 20] {
    //"-IT0001-"+12 random chars
    let mut res = [0; 20];
    res[0..8].copy_from_slice("-IT0001-".as_bytes());
    let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 12);
    res[8..20].copy_from_slice(string.as_bytes());
    res
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

pub fn digits(num: usize) -> impl Iterator<Item = u32> {
    num.to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<_>>()
        .into_iter()
}

pub fn len_to_bytes(len: usize) -> Vec<u8> {
    let mut vec = Vec::new();
    let digits = digits(len);
    for digit in digits {
        match digit {
            0 => vec.push(48),
            1 => vec.push(49),
            2 => vec.push(50),
            3 => vec.push(51),
            4 => vec.push(52),
            5 => vec.push(53),
            6 => vec.push(54),
            7 => vec.push(55),
            8 => vec.push(56),
            9 => vec.push(57),
            _ => {}
        }
    }
    vec
}
