use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::{self, block},
    text::{self, Line, Span, Text},
    widgets::{
        Bar, BarChart, BarGroup, Block, Borders, Gauge, List, ListItem, Paragraph, Tabs, Wrap,
    },
    Frame,
};
use Constraint::{Fill, Length, Min, Percentage};

use crate::app::App;

pub fn draw_peers_tab(frame: &mut Frame, content_area: Rect, app: &mut App) {
    let list_items: Vec<ListItem> = app
        .peers
        .items
        .iter()
        .map(|peer| ListItem::new(format!("{:?}", peer)))
        .collect();

    let peers_widget = List::new(list_items)
        .block(Block::bordered().title(format!("Peers: {}", app.peers.items.len())));

    ////////////////////////////////////////////////////////////////////////////

    let list_items: Vec<ListItem> = app
        .peers
        .items
        .iter()
        .map(|peer| ListItem::new(format!("{:?}", peer)))
        .collect();

    let dht_widget = List::new(list_items).block(Block::bordered().title("DHT"));

    ////////////////////////////////////////////////////////////////////////////

    let mut rng = rand::rng();
    let temps: Vec<u8> = (0..24)
        .map(|_| rand::Rng::random_range(&mut rng, 50..90))
        .collect();
    let bars: Vec<Bar> = temps
        .iter()
        .enumerate()
        .map(|(hour, value)| horizontal_bar(hour, value))
        .collect();
    let title = Line::from("Weather (Horizontal)");
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
    frame.render_stateful_widget(dht_widget, top, &mut app.peers.state);
    frame.render_widget(graphs_widget, bottom);
}

fn horizontal_bar(hour: usize, temperature: &u8) -> Bar {
    let style = temperature_style(*temperature);
    Bar::default()
        .value(u64::from(*temperature))
        .label(Line::from(format!("{hour:>02}:00")))
        .text_value(format!("{temperature:>3}°"))
        .style(style)
        .value_style(style.reversed())
}
fn temperature_style(value: u8) -> Style {
    let green = (255.0 * (1.0 - f64::from(value - 50) / 40.0)) as u8;
    let color = Color::Rgb(255, green, 0);
    Style::new().fg(color)
}
