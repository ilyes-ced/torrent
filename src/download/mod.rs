use std::sync::{mpsc::channel, Arc, Mutex};
use std::thread;

use client::Client;
use download::PieceResult;

use crate::peers::Peer;
use crate::torrent::Torrent;

mod bitfield;
mod client;
mod download;
mod handshake;
mod message;

pub fn start(torrent: Torrent, peers: Vec<Peer>) -> Result<String, String> {
    println!("starting download\n");

    let clients_arc: Arc<Mutex<Vec<Client>>> = Arc::new(Mutex::new(Vec::new()));
    let torrent_arc: Arc<Torrent> = Arc::new(torrent.clone());
    let peers_arc: Arc<Vec<Peer>> = Arc::new(peers.clone());
    let mut handles = vec![];

    for i in 0..peers.len() {
        let clients_clone = Arc::clone(&clients_arc);
        let torrent_clone = Arc::clone(&torrent_arc);
        let peers_clone = Arc::clone(&peers_arc);
        let index_clone = i;
        println!(
            "starting handshake with peer {index_clone}: {:?}",
            peers_clone[index_clone]
        );
        // creates the clients
        let handle =
            thread::spawn(
                move || match Client::new(&torrent_clone, &peers_clone[index_clone]) {
                    Ok(client) => {
                        let mut lock = clients_clone.lock().unwrap();
                        lock.push(client);
                        // not sure
                        drop(lock)
                    }
                    Err(err) => println!(
                        "connection with peer was dropped: index:{index_clone}, {:?} | cause: {}",
                        peers_clone[index_clone], err
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

    // get the clients list
    let clients_lock = Arc::try_unwrap(clients_arc).expect("Lock still has multiple owners");
    let clients = clients_lock
        .into_inner()
        .expect("clients mutex cannot be locked");

    println!("number of clients:  {:?}", clients.len());
    //for client in clients {
    //    println!("ip of client:  {:?}", client.peer);
    //}

    let (tx, rx) = channel::<PieceResult>();
    // mscp channel for finished pieces
    let handle = thread::spawn(move || loop {
        // here we write data to file
        let finished_piece = rx.recv().unwrap();
        println!(
            "!!!!!!--------------------- received completed download of piece {} ---------------------!!!!!!",
            finished_piece.index
        );
    });

    let _download = download::start(torrent, clients, tx);

    handle.join().unwrap();
    Ok(String::new())
}
