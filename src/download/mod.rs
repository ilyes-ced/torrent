use crate::torrent::Torrent;

mod connection;
mod handshake;
mod message;

pub fn start(torrent: Torrent) -> Result<String, String> {
    // make threads that make connections and maintains the tcp streams after a successful handshake
    let connection = connection::start(torrent).unwrap();
    Ok(String::new())
}
