use std::net::{ToSocketAddrs, UdpSocket};

pub fn get_peers() {
    let host = "127.0.0.1";
    let port = 34254;

    let mut addrs_iter = "tracker.openbittorrent.com:80".to_socket_addrs().unwrap();
    let socket_addr = addrs_iter.next().unwrap();
    println!("{:?}", addrs_iter.next());

    let socket = UdpSocket::bind("0.0.0.0:34254").unwrap();
    println!("binded");

    let fir = transform_u64_to_array_of_u8(0x41727101980);
    let sec = transform_u32_to_array_of_u8(0x0);
    let thi = transform_u32_to_array_of_u8(0x3645); // random 4 bytes
    let mut buffer = [
        fir[0], fir[1], fir[2], fir[3], fir[4], fir[5], fir[6], fir[7], sec[0], sec[1], sec[2],
        sec[3], thi[0], thi[1], thi[2], thi[3],
    ];

    //let mut buffer = [
    //    /*magic constant*/0x4, 0x1, 0x7, 0x2, 0x7, 0x1, 0x019, 0x80,// 0x9, 0x8, 0x0,
    //    /*action*/ 0x0, 0x0, 0x0, 0x0,
    //    /*transaction_id*/ 0xf5, 0x10, 0xb6, 0x14
    //];

    socket.send_to(&buffer, socket_addr).unwrap();
    println!("sent {:?}", &buffer);

    let mut buf: [u8; 16] = [0; 16];
    let (amt, src) = socket.recv_from(&mut buffer).unwrap();

    println!("{:?}", &buffer);

    println!("{}", src);
    println!("{}", amt);
}

fn transform_u32_to_array_of_u8(x: u32) -> [u8; 4] {
    let b1: u8 = ((x >> 24) & 0xff) as u8;
    let b2: u8 = ((x >> 16) & 0xff) as u8;
    let b3: u8 = ((x >> 8) & 0xff) as u8;
    let b4: u8 = (x & 0xff) as u8;
    [b1, b2, b3, b4]
}
fn transform_u64_to_array_of_u8(x: u64) -> [u8; 8] {
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
