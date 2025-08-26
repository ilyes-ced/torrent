use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block, List, ListItem},
    Frame,
};
use Constraint::Fill;

use crate::{
    app::{ActiveBlock, App},
    tracker::Peer,
    utils::readable_size,
};

pub fn draw_peers_tab(frame: &mut Frame, content_area: Rect, app: &mut App) {
    let list_items: Vec<ListItem> = app
        .peers
        .keys()
        .map(|peer| ListItem::new(format!("{:?}", peer)))
        .collect();

    let peers_block = if app.active_block == ActiveBlock::Peers {
        Block::bordered()
            .border_style(Style::new().blue().bold())
            .title(format!("Peers: {}", app.peers.keys().len()))
    } else {
        Block::bordered().title(format!("Peers: {}", app.peers.keys().len()))
    };

    let peers_widget = List::new(list_items).block(peers_block);

    ////////////////////////////////////////////////////////////////////////////

    let list_items: Vec<ListItem> = app
        .peers
        .keys()
        .map(|peer| ListItem::new(format!("{:?}", peer)))
        .collect();

    let dht_block = if app.active_block == ActiveBlock::DHT {
        Block::bordered()
            .border_style(Style::new().blue().bold())
            .title("DHT")
    } else {
        Block::bordered().title("DHT")
    };

    let dht_widget = List::new(list_items).block(dht_block);

    ////////////////////////////////////////////////////////////////////////////

    //let mut rng = rand::rng();
    //let temps: Vec<u8> = (0..24)
    //    .map(|_| rand::Rng::random_range(&mut rng, 50..90))
    //    .collect();
    let bars: Vec<Bar> = app
        .peers
        .iter()
        .enumerate()
        .map(|(hour, (peer, bytes))| horizontal_bar(peer, bytes))
        .collect();
    let title = Line::from("Bytes Downlaoded / Peer");
    let graphs_widget = BarChart::default()
        .block(Block::bordered().title(title))
        .data(BarGroup::default().bars(&bars))
        .bar_width(1)
        .bar_gap(0)
        .direction(Direction::Horizontal);

    ////////////////////////////////////////////////////////////////////////////

    let main_tabs = Layout::horizontal([Fill(1), Fill(1)]);
    let [left, right] = main_tabs.areas(content_area);

    let new = Layout::vertical([Fill(1), Fill(1)]);
    let [top, bottom] = new.areas(right);

    frame.render_stateful_widget(peers_widget, left, &mut app.download_logs.state);
    frame.render_widget(dht_widget, top);
    frame.render_widget(graphs_widget, bottom);
}

fn horizontal_bar(peer: &Peer, bytes: &usize) -> Bar<'static> {
    let (size, style) = bytes_style(*bytes);
    Bar::default()
        .value(bytes.clone().try_into().unwrap())
        // .label(Line::from(format!("{:?}", peer)))
        .text_value(format!("{:?} => {}", peer, size))
        .style(style)
        .value_style(style.reversed())
}
fn bytes_style(value: usize) -> (String, Style) {
    let green = (255.0 * (1.0 - f64::from(value as f64 - 50.0) / 40.0)) as u8;
    let color = Color::Rgb(16, green, 32);
    let style = Style::new().fg(color);

    (readable_size(value as f64), style)
}
