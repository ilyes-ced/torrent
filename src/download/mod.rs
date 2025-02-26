use std::sync::{mpsc::channel, Arc, Mutex};
use std::thread;

use crate::client::Client;
use crate::log::{error, info};
use crate::peers::Peer;
use crate::torrent::Torrent;
use crate::writer;

pub(crate) mod download;
use download::PieceResult;
use writer::write_file;

pub fn start(torrent: Torrent, peers: Vec<Peer>) -> Result<String, String> {
    info("starting download\n".to_string());

    let clients_arc: Arc<Mutex<Vec<Client>>> = Arc::new(Mutex::new(Vec::new()));
    let torrent_arc: Arc<Torrent> = Arc::new(torrent.clone());
    let peers_arc: Arc<Vec<Peer>> = Arc::new(peers.clone());
    let mut handles = vec![];

    for i in 0..peers.len() {
        let clients_clone = Arc::clone(&clients_arc);
        let torrent_clone = Arc::clone(&torrent_arc);
        let peers_clone = Arc::clone(&peers_arc);
        let index_clone = i;
        info(format!(
            "starting handshake with peer {index_clone}: {:?}",
            peers_clone[index_clone],
        ));
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
                    Err(err) => error(format!(
                        "connection with peer was dropped: index:{index_clone}, {:?} | cause: {}",
                        peers_clone[index_clone], err
                    )),
                },
            );
        handles.push(handle);
    }

    for handle in handles {
        if let Err(e) = handle.join() {
            error(format!("Thread encountered an error: {:?}", e));
        }
    }

    // get the clients list
    let clients_lock = Arc::try_unwrap(clients_arc).expect("Lock still has multiple owners");
    let clients = clients_lock
        .into_inner()
        .expect("clients mutex cannot be locked");

    info(format!("number of clients:  {:?}", clients.len()));
    if clients.len() == 0 {
        error("no clients were found for this torrent".to_string());
        return Err(String::from("no clients were found for this torrent"));
    }

    /*
        ! very bad with torrent_arc stuff here, needs to be changed
    */

    let torrent_arc = Arc::new(Mutex::new(torrent));
    let torrent_arc_clone = Arc::clone(&torrent_arc);
    let (tx, rx) = channel::<PieceResult>();
    // mscp channel for finished pieces
    let handle = thread::spawn(move || loop {
        // here we write data to file
        let finished_piece = match rx.recv() {
            Ok(res) => res,
            Err(err) => {
                error(format!(
                    "error receiving in the receiver thread: {}",
                    err.to_string()
                ));
                std::process::exit(0);
            }
        };
        let torrent_guard = torrent_arc_clone.lock().unwrap();

        let _ = write_file(&torrent_guard, finished_piece.clone()).unwrap();
        info(format!(
            "!!!!!!--------------------- received completed download of piece {} ---------------------!!!!!!",
            finished_piece.index,
        ));
    });

    let torrent_guard = torrent_arc.lock().unwrap().to_owned(); // Acquire the lock
    let _download = download::start(torrent_guard, clients, tx);

    handle.join().unwrap();
    Ok(String::new())
}
