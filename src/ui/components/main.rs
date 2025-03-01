use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{self, Span},
    widgets::{Axis, BarChart, Block, Chart, Dataset, List, ListItem},
    Frame,
};

use crate::app::App;

#[allow(clippy::too_many_lines)]
pub fn draw_charts(frame: &mut Frame, app: &mut App, area: Rect) {
    let constraints = vec![Constraint::Percentage(50), Constraint::Percentage(50)];
    let main_chunks = Layout::horizontal(constraints).split(area);

    {
        {
            let chunks = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(main_chunks[0]);

            let barchart = BarChart::default().block(Block::bordered().title("torrent data here"));
            frame.render_widget(barchart, chunks[0]);

            let barchart = BarChart::default().block(Block::bordered().title("Bar chart"));
            //   .data(&app.barchart)
            //   .bar_width(3)
            //   .bar_gap(2)
            //   .bar_set(if app.enhanced_graphics {
            //       symbols::bar::NINE_LEVELS
            //   } else {
            //       symbols::bar::THREE_LEVELS
            //   })
            //   .value_style(
            //       Style::default()
            //           .fg(Color::Black)
            //           .bg(Color::Green)
            //           .add_modifier(Modifier::ITALIC),
            //   )
            //   .label_style(Style::default().fg(Color::Yellow))
            //   .bar_style(Style::default().fg(Color::Green));
            frame.render_widget(barchart, chunks[1]);
        }

        {
            let chunks = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(main_chunks[1]);

            // Draw downloads tasks
            // Draw connections logs
            let info_style = Style::default().fg(Color::Blue);
            let warning_style = Style::default().fg(Color::Yellow);
            let error_style = Style::default().fg(Color::Magenta);
            let critical_style = Style::default().fg(Color::Red);
            let logs: Vec<ListItem> = app
                .downloads_logs
                .items
                .iter()
                .map(|&(evt, level)| {
                    let s = match level {
                        "ERROR" => error_style,
                        "CRITICAL" => critical_style,
                        "WARNING" => warning_style,
                        _ => info_style,
                    };
                    let content = vec![text::Line::from(vec![
                        Span::styled(format!("{level:<9}"), s),
                        Span::raw(evt),
                    ])];
                    ListItem::new(content)
                })
                .collect();
            let logs = List::new(logs).block(Block::bordered().title("downloads logs"));
            frame.render_stateful_widget(logs, chunks[0], &mut app.downloads_logs.state);

            // Draw connections logs
            let info_style = Style::default().fg(Color::Blue);
            let warning_style = Style::default().fg(Color::Yellow);
            let error_style = Style::default().fg(Color::Magenta);
            let critical_style = Style::default().fg(Color::Red);
            let logs: Vec<ListItem> = app
                .connections_logs
                .items
                .iter()
                .map(|&(evt, level)| {
                    let s = match level {
                        "ERROR" => error_style,
                        "CRITICAL" => critical_style,
                        "WARNING" => warning_style,
                        _ => info_style,
                    };
                    let content = vec![text::Line::from(vec![
                        Span::styled(format!("{level:<9}"), s),
                        Span::raw(evt),
                    ])];
                    ListItem::new(content)
                })
                .collect();
            let logs = List::new(logs).block(Block::bordered().title("connections logs"));
            frame.render_stateful_widget(logs, chunks[1], &mut app.connections_logs.state);
        }
    }
}
