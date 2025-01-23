use std::sync::{Arc, Mutex};
use std::{net::TcpStream, thread};

use client::Client;

use crate::torrent::Torrent;

mod bitfield;
mod client;
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

    // get the clients list
    let lock = Arc::try_unwrap(clients).expect("Lock still has multiple owners");
    let clients = lock.into_inner().expect("Mutex cannot be locked");

    println!("number of clients:  {:?}", clients.len());
    //for client in clients {
    //    println!("ip of client:  {:?}", client.peer);
    //}

    let download = download::start(torrent, clients).unwrap();

    Ok(String::new())
}
