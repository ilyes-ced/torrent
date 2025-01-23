use std::sync::{Arc, Mutex};
use std::{net::TcpStream, thread};

use client::Client;

use crate::torrent::Torrent;

mod bitfield;
mod client;
mod connection;
mod download;
mod handshake;
mod message;

pub fn start(torrent: Torrent) -> Result<String, String> {
    println!("starting download\n");

    let clients: Arc<Mutex<Vec<Client>>> = Arc::new(Mutex::new(Vec::new()));
    let torrent_arc: Arc<Torrent> = Arc::new(torrent.clone());
    let mut handles = vec![];

    for i in 0..torrent_arc.peers.len() {
        let clients_clone = Arc::clone(&clients);
        let torrent_clone = Arc::clone(&torrent_arc);
        let index_clone = i;
        println!(
            "starting handshake with peer {index_clone}: {:?}",
            torrent_clone.peers[index_clone]
        );
        let handle = thread::spawn(move || match Client::new(&torrent_clone, index_clone) {
            Ok(client) => {
                let mut guard = clients_clone.lock().unwrap();
                guard.push(client);
                // not sure
                //drop(guard)
            }
            Err(err) => println!(
                "connection with peer was dropped: index:{index_clone}, {:?} | cause: {}",
                torrent_clone.peers[index_clone], err
            ),
        });
        handles.push(handle);
    }

    for handle in handles {
        if let Err(e) = handle.join() {
            eprintln!("Thread encountered an error: {:?}", e);
        }
    }

    let guard = clients.lock().unwrap();
    println!("second read: recieving bitfields:  {:?}", guard);

    // recieving bitfields
    //for i in 0..cons.len() {
    //    println!("con ip: {:?}", cons[i].peer_addr().unwrap());
    //    let response = match message::from_buf(&cons[i]) {
    //        Ok(msg) => msg.unwrap(),
    //        Err(err) => {
    //            return Err(String::from(format!(
    //                "error occured when getting bitfields message: {err}",
    //            )))
    //        }
    //    };
    //    println!("second read: recieving bitfields:  {:?}", response)
    //}

    let download = download::start(torrent).unwrap();

    Ok(String::new())
}
