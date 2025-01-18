mod handshake;
use handshake::{build_handshake, INTERESTED_MESSAGE};

use std::io::prelude::*;
use std::net::TcpStream;

use crate::tracker::PeersResult;

//keepalive  0000
//choke  00010
//unchoke  00011
//intrested  00012
//unintrested  00013
//have  00054<pieceindex 4bytes>
//bitfield <0001+len>5<bitfield>
//request <0013><6><index><begin><length>
//piece <len=0009+X><id=7><index><begin><block>
//cancel <len=0013><id=8><index><begin><length>
//port <len=0003><id=9><listen-port>

struct Download {
    socket: TcpStream,
    buffer: Vec<u8>,
    ip_addresses: Vec<String>,
}

pub fn download(peers: PeersResult) -> std::io::Result<()> {
    let binding = build_handshake(&peers).unwrap();
    let handshake = binding.as_slice();

    for peer in peers.ips {
        if peer != String::from("41.98.200.138:6881") || peer != String::from("41.98.200.138:6881")
        {
            println!("tcp ip::::: {}", peer);

            println!("hello ther");
            let mut stream = TcpStream::connect(peer).unwrap();
            println!("tcp ip::::: {:?}", stream);

            let gg: usize = stream.write(handshake).unwrap();
            println!("tcp ip::::: {}", gg);

            whole_msg(&mut stream);
        }
    }

    Ok(())
}

fn whole_msg(stream: &mut TcpStream) {
    let mut saved_buffer: &[u8] = &[0];
    let mut recu_buffer: &mut [u8] = &mut [0];
    let mut handshake = true;

    // on data is here i think
    stream.read(recu_buffer).unwrap();
    println!("recu message **********:: {:?}", recu_buffer);

    //combines 2 sllices into one
    let binding = std::iter::once(&saved_buffer[..])
        .chain(std::iter::once(&recu_buffer[..]))
        .flatten()
        .copied()
        .collect::<Vec<_>>();
    saved_buffer = binding.as_slice();

    let msg_len = get_msg_len(&handshake, &saved_buffer);
    while saved_buffer.len() >= 4 && saved_buffer.len() >= msg_len {
        // handle the data here
        handle_message(&saved_buffer[0..msg_len], stream);

        saved_buffer = &saved_buffer[msg_len..];
        handshake = false;
    }
}

fn handle_message(param: &[u8], stream: &mut TcpStream) {
    if is_handshake(param) {
        stream.write(&INTERESTED_MESSAGE).unwrap();
    } else {
        let gg = parse(param);
        println!("ggggggggggg {:?}", gg);
    }
}

fn is_handshake(msg: &[u8]) -> bool {
    if msg.len() == (msg[0] + 49) as usize
        && &msg[0..20] == String::from("BitTorrent protocol").as_bytes()
    {
        return true;
    } else {
        return false;
    }
}

fn get_msg_len(handshake: &bool, buffer: &[u8]) -> usize {
    if *handshake {
        (buffer[0] + 49).into()
    } else {
        (u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]) + 4)
            .try_into()
            .unwrap()
    }
}

fn parse(buffer: &[u8]) -> (u32, Option<u8>, Option<&[u8]>) {
    let id = if buffer.len() > 4 {
        Some(buffer[0])
    } else {
        None
    };
    let payload: Option<&[u8]> = if buffer.len() > 5 {
        Some(&buffer[5..])
    } else {
        None
    };

    if id == Some(6) || id == Some(7) || id == Some(8) {
        let rest = &payload.unwrap()[8..];
        let mut payload: (u32, u32, &[u8]) = (
            u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]),
            u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]),
            &[],
        );
        if id == Some(7) {
            payload.2 = rest;
        }
        //payload[id === 7 ? 'block' : 'length'] = rest;
    }

    return (
        u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]),
        id,
        payload,
    );
}
