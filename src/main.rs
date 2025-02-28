mod client;
mod constants;
mod download;
mod io;
mod log;
mod peers;
mod torrentfile;
mod utils;

use std::{fs::File, io::Read};
use torrentfile::bencode::Decoder;
use torrentfile::torrent::Torrent;

use crossterm::event::{self, Event};
use ratatui::{
    layout::{Constraint, Layout},
    widgets::Block,
    Frame,
};
fn main() -> std::io::Result<()> {
    //let mut terminal = ratatui::init();
    //loop {
    //    terminal.draw(draw).expect("failed to draw frame");
    //    if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
    //        break;
    //    }
    //}
    //ratatui::restore();

    println!("testtest");
    start_torrent().unwrap();
    Ok(())
}

fn draw(frame: &mut Frame) {
    use Constraint::{Fill, Length, Min};

    let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
    let [title_area, main_area, status_area] = vertical.areas(frame.area());
    let horizontal = Layout::horizontal([Fill(1); 2]);
    let [left_area, right_area] = horizontal.areas(main_area);

    frame.render_widget(Block::bordered().title("Title Bar"), title_area);
    frame.render_widget(Block::bordered().title("Status Bar"), status_area);
    frame.render_widget(Block::bordered().title("Left"), left_area);
    frame.render_widget(Block::bordered().title("Right"), right_area);
}

fn start_torrent() -> Result<(), String> {
    //maybe we need a static PeerId
    let peer_id = utils::new_peer_id();
    //let path = "debian.torrent";
    let path = "tests/torrents/many_files.torrent";
    let mut file = File::open(path).map_err(|e| e.to_string())?;
    let mut buf = vec![];
    file.read_to_end(&mut buf).map_err(|e| e.to_string())?;

    let bencode_data = Decoder::new(&buf).start().unwrap();
    let torrent_data = Torrent::new(bencode_data, peer_id).unwrap();
    let peers = peers::get_peers(&torrent_data, peer_id).unwrap();
    download::start(torrent_data, peers.peers).unwrap();

    Ok(())
}
