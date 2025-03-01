use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{self, Span},
    widgets::{Block, Paragraph, Wrap},
    Frame,
};

// downloade percentage and pieces downloaded
pub fn files_info(frame: &mut Frame, area: Rect) {
    let text = vec![
        text::Line::from(vec![
            Span::styled("torrent: ", Style::default().fg(Color::Green)),
            Span::styled("[Anime Time] One Piece (0001-1071+Movies+Specials) [BD+CR] [Dual Audio] [1080p][HEVC 10bit x265][AAC][Multi Sub]", Style::default().fg(Color::Green)),
        ]),
        text::Line::from(vec![
            Span::styled("download directory: ", Style::default().fg(Color::Green)),
            Span::styled("~/downloads", Style::default().fg(Color::Green)),
        
        ]),
    ];
    let block = Block::bordered().title(Span::styled(
        "Downloade progress info",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}
