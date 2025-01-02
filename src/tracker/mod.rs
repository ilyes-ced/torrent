mod peers;

use peers::{Peer, PeersResult};

use crate::torrent::Torrent;

pub fn get_peers(torrent_data: Torrent) -> Result<PeersResult, String> {
    Peer::get_peers(torrent_data)
}
