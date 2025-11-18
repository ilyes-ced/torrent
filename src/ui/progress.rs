use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{self, Span},
    widgets::{Block, Gauge, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

pub fn draw_progress(frame: &mut Frame, progress_area: Rect, app: &mut App) {
    let chunks = Layout::vertical([Constraint::Length(1), Constraint::Length(1)])
        .margin(1)
        .split(progress_area);
    let block = Block::bordered().title("Progress");
    frame.render_widget(block, progress_area);

    let label = format!(
        "{:.2}%",
        (app.downloaded_pieces as f64 / app.pieces as f64) * 100.0
    );
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
        .ratio(app.downloaded_pieces as f64 / app.pieces as f64);

    let downloaded_perc = format!(
        "{:.2}%",
        (app.downloaded_pieces as f64 / app.pieces as f64) * 100.0
    );
    let downloaded_pieces = format!("{}/{}", app.downloaded_pieces, app.pieces);

    //TODO: make the spacing size dynamic
    let total_width = chunks[0].width as usize;
    let spacing =
        total_width.saturating_sub(10 + 18 + downloaded_perc.len() + downloaded_pieces.len()) - 1;

    let text = vec![text::Line::from(vec![
        Span::from("progress: "),
        Span::styled(downloaded_perc, Style::default().fg(Color::Blue)),
        Span::raw(" ".repeat(spacing)).into(),
        Span::from("Pieces downloaded: "),
        Span::styled(downloaded_pieces, Style::default().fg(Color::Blue)),
    ])];

    let progress = Paragraph::new(text)
        .block(Block::new())
        .wrap(Wrap { trim: false });

    frame.render_widget(progress, chunks[0]);
    frame.render_widget(gauge, chunks[1]);
}
