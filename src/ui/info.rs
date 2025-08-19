use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{self, Span},
    widgets::{Block, Paragraph, Wrap},
    Frame,
};
use Constraint::Percentage;

use crate::app::App;

pub fn draw_info(frame: &mut Frame, title_area: Rect, app: &mut App) {
    let main_horizontal = Layout::horizontal([Percentage(50), Percentage(50)]);
    let [left_info, right_info] = main_horizontal.areas(title_area);

    let text = vec![
        text::Line::from(vec![
            Span::from("torrent: "),
            Span::styled(app.torrent_name, Style::default().fg(Color::Green)),
        ]),
        text::Line::from(vec![
            Span::from("download dir: "),
            Span::styled(app.download_dir, Style::default().fg(Color::Green)),
        ]),
    ];
    let title = Paragraph::new(text)
        .block(Block::bordered().title("torrent info"))
        .wrap(Wrap { trim: true });

    frame.render_widget(title, left_info);

    let text = vec![
        text::Line::from(vec![
            Span::from("infohash: "),
            Span::styled(app.info_hash, Style::default().fg(Color::Green)),
        ]),
        text::Line::from(vec![
            Span::from("torrent size: "),
            Span::styled(app.size, Style::default().fg(Color::Green)),
        ]),
    ];
    let title = Paragraph::new(text)
        .block(Block::bordered().title("torrent info"))
        .wrap(Wrap { trim: true });

    frame.render_widget(title, right_info);
}
