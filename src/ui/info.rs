use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{self, Span},
    widgets::{Block, Paragraph, Wrap},
    Frame,
};
use Constraint::Percentage;

pub fn draw_info(frame: &mut Frame, title_area: Rect) {
    let main_horizontal = Layout::horizontal([Percentage(50), Percentage(50)]);
    let [left_info, right_info] = main_horizontal.areas(title_area);

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
        .block(Block::bordered().title("torrent info"))
        .wrap(Wrap { trim: true });

    frame.render_widget(title, left_info);

    let text = vec![
        text::Line::from(vec![
            Span::from("infohash: "),
            Span::styled(
                "6fcf7ef136e73f0fb6186b30fe67d741cc260c5c",
                Style::default().fg(Color::Green),
            ),
        ]),
        text::Line::from(vec![
            Span::from("torrent size: "),
            Span::styled("3.9 GiB", Style::default().fg(Color::Green)),
        ]),
    ];
    let title = Paragraph::new(text)
        .block(Block::bordered().title("torrent info"))
        .wrap(Wrap { trim: true });

    frame.render_widget(title, right_info);
}
