use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{self, Span},
    widgets::{Block, Paragraph, Wrap},
    Frame,
};
use Constraint::Percentage;

use crate::app::App;

pub fn draw_info(frame: &mut Frame, title_area: Rect, app: &App) {
    let text = vec![
        text::Line::from(vec![
            Span::from("torrent: "),
            Span::styled(&app.torrent_name, Style::default().fg(Color::Green)),
        ]),
        text::Line::from(vec![
            Span::from("download dir: "),
            Span::styled(&app.download_dir, Style::default().fg(Color::Green)),
        ]),
        text::Line::from(vec![
            Span::from("Peer Id: "),
            Span::styled(&app.peer_id, Style::default().fg(Color::Green)),
        ]),
        text::Line::from(vec![
            Span::from("infohash: "),
            Span::styled(
                app.info_hash
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<String>(),
                Style::default().fg(Color::Green),
            ),
        ]),
        text::Line::from(vec![
            Span::from("torrent size: "),
            // todo: make the MiB or GiB tranformations
            Span::styled(app.size.to_string(), Style::default().fg(Color::Green)),
        ]),
        // text::Line::from(vec![
        //     Span::from("&app.torrent_type"),
        //     Span::styled("&app.torrent_type_value", Style::default().fg(Color::Green)),
        // ]),
    ];

    let title = Paragraph::new(text)
        .block(Block::bordered().title("torrent info"))
        .wrap(Wrap { trim: true });

    frame.render_widget(title, title_area);
}
