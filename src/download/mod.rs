use std::io::{self, Read};
use std::sync::{Arc, Mutex};
use std::{net::TcpStream, thread};

use crate::torrent::Torrent;

mod connection;
mod download;
mod handshake;
mod message;

pub fn start(torrent: Torrent) -> Result<String, String> {
    println!("\nstarting download\n");

    let connections: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));
    let torrent_arc: Arc<Torrent> = Arc::new(torrent.clone());
    let mut handles = vec![];

    for i in 0..torrent_arc.peers.len() {
        let connections_clone = Arc::clone(&connections);
        let torrent_clone = Arc::clone(&torrent_arc);
        let index_clone = i;
        println!(
            "starting handshake with peer {index_clone}: {:?}\n",
            torrent_clone.peers[index_clone]
        );
        let handle = thread::spawn(
            move || match connection::start(&torrent_clone, index_clone) {
                Ok(con) => {
                    let mut num: std::sync::MutexGuard<'_, Vec<TcpStream>> =
                        connections_clone.lock().unwrap();
                    num.push(con);
                    // not sure
                    //drop(num)
                }
                Err(err) => println!(
                    "connection with peer was dropped: index:{index_clone}, {:?}\ncause: {}",
                    torrent_clone.peers[index_clone], err
                ),
            },
        );
        handles.push(handle);
    }

    for handle in handles {
        if let Err(e) = handle.join() {
            eprintln!("Thread encountered an error: {:?}", e);
        }
    }

    let cons = connections.lock().unwrap();
    // recieving bitfields
    for i in 0..cons.len() {
        println!("con ip: {:?}", cons[i].peer_addr().unwrap());
        let response = message::from_buf(&cons[i]).unwrap();
        println!("second read :::::::::: {:?}", response)
    }
    drop(cons);

    let f = download::start(torrent).unwrap();

    Ok(String::new())
}
