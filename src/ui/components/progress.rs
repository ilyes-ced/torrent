use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{self, Span},
    widgets::{Block, Gauge, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

// torrent name + downloade directory
pub fn download_info(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::vertical([Constraint::Length(18), Constraint::Length(2)])
        .margin(1)
        .split(area);
    let block = Block::bordered().title("IO info");
    frame.render_widget(block, area);

    let prog = 0.55;

    let label = format!("{:.2}%", prog * 100.0);
    let gauge = Gauge::default()
        .block(Block::new())
        .gauge_style(
            Style::default()
                .fg(Color::Magenta)
                .bg(Color::Black)
                .add_modifier(Modifier::ITALIC | Modifier::BOLD),
        )
        .use_unicode(app.enhanced_graphics)
        .label(label)
        .ratio(prog);
    frame.render_widget(gauge, chunks[1]);

    let progress_text = vec![text::Line::from(vec![
        Span::styled("download progress: ", Style::default().fg(Color::Green)),
        Span::styled("20.031%/100.0%", Style::default().fg(Color::Green)),
    ])];
    let piecesprogress_text = vec![text::Line::from(vec![
        Span::styled(
            "pieces download progress: ",
            Style::default().fg(Color::Green),
        ),
        Span::styled("20/2054", Style::default().fg(Color::Green)),
    ])];

    let paragraph = Paragraph::new(progress_text).wrap(Wrap { trim: true });
    let paragraph2 = Paragraph::new(piecesprogress_text)
        .alignment(ratatui::layout::Alignment::Right)
        .wrap(Wrap { trim: true });

    let text_chunks = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[0]);

    frame.render_widget(paragraph, text_chunks[0]);
    frame.render_widget(paragraph2, text_chunks[1]);
}
