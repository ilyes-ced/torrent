use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{self, Span},
    widgets::{Block, Gauge, Paragraph, Wrap},
    Frame,
};

pub fn draw_progress(frame: &mut Frame, progress_area: Rect) {
    let chunks = Layout::vertical([Constraint::Length(1), Constraint::Length(1)])
        .margin(1)
        .split(progress_area);
    let block = Block::bordered().title("Graphs");
    frame.render_widget(block, progress_area);

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
}
