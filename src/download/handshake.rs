use crate::tracker::PeersResult;





pub const keepalive_message:  [u8; 4] = [0, 0, 0, 0];
pub const choke_message:      [u8; 5] = [0, 0, 0, 1, 0];
pub const unchoke_message:    [u8; 5] = [0, 0, 0, 1, 1];
pub const intrested_message:  [u8; 5] = [0, 0, 0, 1, 2];
pub const unintrested_message:[u8; 5] = [0, 0, 0, 1, 3];
















pub fn build_handshake(peers: &PeersResult) -> Result<[u8; 68], String> {

    let mut buffer: [u8; 68] = [0; 68];
    buffer[0..1].copy_from_slice(&[19]);
    buffer[1..20].copy_from_slice("BitTorrent protocol".as_bytes());
    buffer[20..24].copy_from_slice(&[0, 0, 0, 0]);
    buffer[24..28].copy_from_slice(&[0, 0, 0, 0]);
    
    buffer[28..48].copy_from_slice(&peers.info_hash);
    // might need a  newly generated one
    buffer[48..68].copy_from_slice(&peers.peer_id);



    println!("handshake builf ** {:?}", buffer);

    Ok(buffer)
}




























fn have(index: [u8; 4]) -> [u8; 9] {
    [0, 0, 0, 5, 4, index[0], index[1], index[2], index[3]]
}



fn bitfield(length: u32, index: [u8; 9]) -> [u8; 14] {
    let bytes = transform_u32_to_array_of_u8(length + 1);
    [bytes[0], bytes[1], bytes[2], bytes[3], 5, index[0], index[1], index[2], index[3], index[4], index[5], index[6], index[7], index[8]]
}



fn request(index: [u8; 4], begin: [u8; 4], length: [u8; 4]) -> [u8; 17] {
    let bytes = transform_u32_to_array_of_u8(13);
    [
        bytes[0], bytes[1], bytes[2], bytes[3],
        6, index[0], index[1], index[2],
        index[3], begin[0], begin[1], begin[2],
        begin[3], length[0], length[1], length[2],
        length[3]
    ]
}



fn piece(index: [u8; 4], begin: [u8; 4], block: [u8; 4]) -> Vec<u8> {
    let mut result = Vec::new();
    let bytes = transform_u32_to_array_of_u8((13 + block.len()).try_into().unwrap());
    result.push(bytes[0]);
    result.push(bytes[1]);
    result.push(bytes[2]);
    result.push(bytes[3]);

    result.push(7);

    result.push(index[0]);
    result.push(index[1]);
    result.push(index[2]);
    result.push(index[3]);

    result.push(begin[0]);
    result.push(begin[1]);
    result.push(begin[2]);
    result.push(begin[3]);

    // the rest here
    result.push(begin[3]);


    result
}



fn cancel(index: [u8; 4], begin: [u8; 4], length: [u8; 4]) -> [u8; 17] {
    let bytes = transform_u32_to_array_of_u8(13);
    [
        bytes[0], bytes[1], bytes[2], bytes[3],
        8, index[0], index[1], index[2],
        index[3], begin[0], begin[1], begin[2],
        begin[3], length[0], length[1], length[2],
        length[3]
    ]
}
fn port(port: [u8; 2]) -> [u8; 7] {
    let bytes = transform_u32_to_array_of_u8(3);
    [bytes[0], bytes[1], bytes[2], bytes[3], 9, port[0], port[1]]
}




fn transform_u32_to_array_of_u8(x: u32) -> [u8; 4] {
    let b1: u8 = ((x >> 24) & 0xff) as u8;
    let b2: u8 = ((x >> 16) & 0xff) as u8;
    let b3: u8 = ((x >> 8) & 0xff) as u8;
    let b4: u8 = (x & 0xff) as u8;
    [b1, b2, b3, b4]
}
