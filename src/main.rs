mod bencode;
mod client;
mod constants;
mod dht;
mod download;
mod io;
mod log;
mod magnet;
mod peers;
mod torrentfile;
mod utils;

use clap::Parser;
use dht::Dht;
use log::{error, info};
use magnet::Magnet;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{self, Span, Text};
use ratatui::widgets::{Block, BorderType, Borders, Gauge, Paragraph, Wrap};
use serde_json::Value;
use std::fmt::format;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use std::{fs::File, io::Read};
use torrentfile::torrent::Torrent;

use tokio;

use crate::bencode::decoder::Decoder;
use crate::bencode::encoder::encode;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(flatten)]
    source: Source,
    // ~/Downloads used to work fine idk why it changed
    #[arg(short, long, default_value = "/home/ilyes/Downloads")]
    download_dir: String,
}
#[derive(Parser, Debug)]
#[group(required = true, multiple = false)]
pub struct Source {
    #[arg(short, long)]
    torrent_file: Option<String>,
    #[arg(short, long)]
    magnet_url: Option<String>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // infohash
    // 6fcf7ef136e73f0fb6186b30fe67d741cc260c5c

    let dht = Dht::new().await.unwrap();

    //let args = Args::parse();
    //// download directory checking
    //info(format!("download directory: {}", args.download_dir));
    //if !Path::new(&args.download_dir).exists() {
    //    error(format!("the provided directory does not exist"));
    //    std::process::exit(0);
    //}
    //let peer_id = utils::new_peer_id();
    //// get torrent data torrent_file or magnet_url
    //let res = if args.source.magnet_url == None {
    //    info(format!(
    //        "starting downloade for torrent: {}",
    //        args.source.torrent_file.clone().unwrap()
    //    ));
    //    let path = &args.source.torrent_file.unwrap();
    //    let mut file = File::open(path).map_err(|e| e.to_string()).unwrap();
    //    let mut buf = vec![];
    //    file.read_to_end(&mut buf)
    //        .map_err(|e| e.to_string())
    //        .unwrap();d
    //    buf
    //} else {
    //    let magnet_data = Magnet::new(&args.source.magnet_url.unwrap());
    //    info(format!("magnet data: {:?}", magnet_data));
    //    todo!();
    //    Vec::new()
    //};
    //// reading torrent file
    ////maybe we need a static PeerId
    //// execution
    //let bencode_data = Decoder::new(&res).start().unwrap();
    //let torrent_data = Torrent::new(bencode_data, peer_id).unwrap();
    //let peers = peers::get_peers(&torrent_data, peer_id).unwrap();
    //download::start(torrent_data, peers.peers, args.download_dir).unwrap();

    // start_tui();
    Ok(())
}

use crossterm::event::{self, Event};
use ratatui::Frame;
use Constraint::{Fill, Length, Min, Percentage};

fn start_tui() {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw).expect("failed to draw frame");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
}

fn draw(frame: &mut Frame) {
    let vertical = Layout::vertical([Length(4), Min(0), Length(4)]);
    let [title_area, main_area, status_area] = vertical.areas(frame.area());

    let main_horizontal = Layout::horizontal([Percentage(50), Percentage(50)]);
    let [top, bottom] = main_horizontal.areas(main_area);

    let main_vertical = Layout::vertical([Fill(1), Fill(1)]);
    let [top_left, bottom_left] = main_vertical.areas(top);
    let [top_right, bottom_right] = main_vertical.areas(bottom);

    ///top title bar///////////////////////////////////////////////////////////////
    let text = vec![
        text::Line::from(vec![
            Span::from("torrent: "),
            Span::styled("torrent_ful_name_here", Style::default().fg(Color::Green)),
        ]),
        text::Line::from(vec![
            Span::from("download dir: "),
            Span::styled("~/Downloads", Style::default().fg(Color::Green)),
        ]),
    ];
    let title = Paragraph::new(text)
        .block(Block::bordered().title("Title Bar"))
        .wrap(Wrap { trim: true });

    frame.render_widget(title, title_area);
    ///////////////////////////////////////////////////////////////////////////////
    ///bottom gauge bar////////////////////////////////////////////////////////////
    let chunks = Layout::vertical([Constraint::Length(1), Constraint::Length(1)])
        .margin(1)
        .split(status_area);
    let block = Block::bordered().title("Graphs");
    frame.render_widget(block, status_area);

    let label = format!("{:.2}%", 0.12 * 100.0);
    let gauge = Gauge::default()
        .block(Block::new())
        .gauge_style(
            Style::default()
                .fg(Color::Magenta)
                .bg(Color::Black)
                .add_modifier(Modifier::ITALIC | Modifier::BOLD),
        )
        .use_unicode(true)
        .label(label)
        .ratio(0.12);

    let total_width = chunks[0].width as usize;
    let spacing = total_width.saturating_sub(16 + 27);

    let text = vec![text::Line::from(vec![
        Span::from("progress: "),
        Span::styled("xx.yy%", Style::default().fg(Color::Blue)),
        Span::raw(" ".repeat(spacing)).into(),
        Span::from("Pieces downloaded:"),
        Span::styled("XXXX/YYYY", Style::default().fg(Color::Blue)),
    ])];

    let progress = Paragraph::new(text)
        .block(Block::new())
        .wrap(Wrap { trim: false });

    frame.render_widget(progress, chunks[0]);
    frame.render_widget(gauge, chunks[1]);
    ///////////////////////////////////////////////////////////////////////////////
    let logs: Vec<(&str, &str)> = vec![
        ("INFO", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("INFO", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("INFO", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("ERROR", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("ERROR", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("ERROR", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("ERROR", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
    ];

    let mut text = vec![];

    for log in logs {
        let color = match &log.0 {
            &"INFO" => Color::Blue,
            &"DEBUG" => Color::Green,
            &"WARNING" => Color::Yellow,
            &"ERROR" => Color::Red,
            _ => Color::White,
        };
        text.push(text::Line::from(vec![
            Span::styled(
                format!("dd-mm-yyyy hh-mm-ss-mmm [{}] ", log.0),
                Style::default().fg(color),
            ),
            Span::from(log.1),
        ]));
    }

    let log = Paragraph::new(text)
        .block(Block::bordered().title("Top Right"))
        .wrap(Wrap { trim: false });

    // "dd-mm-yyyy hh-mm-ss-mmm [INFO] lorem ipsum lorem ipsum"

    frame.render_widget(Block::bordered().title("Top Left"), top_left);
    frame.render_widget(log, top_right);
    frame.render_widget(Block::bordered().title("Bottom Left"), bottom_left);
    frame.render_widget(Block::bordered().title("Bottom Right"), bottom_right);
}
