use std::sync::{mpsc::channel, Arc, Mutex};
use std::thread;

use crate::client::Client;
use crate::io::writer;
use crate::log::{error, info};
use crate::peers::Peer;
use crate::torrentfile::torrent::Torrent;

pub(crate) mod download;
use download::PieceResult;
use writer::write_file;

pub fn start(torrent: Torrent, peers: Vec<Peer>, download_dir: String) -> Result<(), String> {
    info("starting download\n".to_string());

    let clients = get_clients(&torrent, &peers)?;

    start_download(torrent, clients, download_dir)
}

fn get_clients(torrent: &Torrent, peers: &[Peer]) -> Result<Vec<Client>, String> {
    let clients_arc: Arc<Mutex<Vec<Client>>> = Arc::new(Mutex::new(Vec::new()));
    let torrent_arc: Arc<Torrent> = Arc::new(torrent.clone());
    let peers_arc: Arc<Vec<Peer>> = Arc::new(peers.to_vec());
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
    if clients.is_empty() {
        error("no clients were found for this torrent".to_string());
        return Err(String::from("no clients were found for this torrent"));
    }

    Ok(clients)
}

fn start_download(
    torrent: Torrent,
    clients: Vec<Client>,
    download_dir: String,
) -> Result<(), String> {
    /*
        ! very bad with torrent_arc stuff here, needs to be changed
    */
    let cloned_download_dir = download_dir.clone();
    let torrent_arc = Arc::new(Mutex::new(torrent));
    let torrent_arc_clone = Arc::clone(&torrent_arc);
    // receive Some(piece), if it receives "None" it means downloade is over to avoid the error in the end
    let (tx, rx) = channel::<(Option<PieceResult>, f64)>();
    // mscp channel for finished pieces
    let handle = thread::spawn(move || loop {
        // here we write data to file

        let (finished_piece, prog) = match rx.recv() {
            Ok((res, prog)) => match res {
                Some(res) => (res, prog),
                None => {
                    info(format!("downloade progress: {}%", prog));
                    info("downloade finished".to_string());
                    std::process::exit(0);
                }
            },
            Err(err) => {
                // not sure in which case we would end up here
                error(format!("error receiving in the receiver thread: {}", err));
                // could be done more gracefully
                std::process::exit(0);
            }
        };

        //  {
        //      Ok(res) => match res {
        //          Some(piece) => piece,
        //          None => {
        //              // here we close this thread
        //              // and other downloade finished messages
        //              break;
        //          }
        //      },
        //      Err(err) => {
        //          error(format!("error receiving in the receiver thread: {}", err));
        //          std::process::exit(0);
        //      }
        //  }
        let torrent_guard = torrent_arc_clone.lock().unwrap();
        write_file(&torrent_guard, finished_piece.clone(), download_dir.clone()).unwrap();

        info("-------------------------------------------".to_string());
        info(format!(
            "piece {} successfully downloaded",
            finished_piece.index
        ));
        info(format!("download progress {:.3}%", prog));
        info("-------------------------------------------".to_string());
        // todo: here display downloade percentage
    });

    let torrent_guard = torrent_arc.lock().unwrap().to_owned();
    let _download = download::start(torrent_guard, clients, tx, cloned_download_dir);

    handle.join().unwrap();
    Ok(())
}
