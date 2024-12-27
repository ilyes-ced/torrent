use crate::torrent::Torrent;

pub struct Peers {}

impl Peers {
    pub fn new(torrent_data: Torrent) -> Result<Peers, String> {
        Ok(Peers {})
    }
}
