use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style, Stylize},
    symbols::{self, block},
    text::{self, Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Tabs, Wrap},
    Frame,
};
use Constraint::{Fill, Length, Min, Percentage};

pub fn draw_files_tab(frame: &mut Frame) {
    let vertical = Layout::vertical([Length(4), Min(0), Length(4)]);
    let [title_area, main_area, status_area] = vertical.areas(frame.area());

    // let main_horizontal = Layout::horizontal([Percentage(50), Percentage(50)]);
    // let [top, bottom] = main_horizontal.areas(main_area);
    //
    // let main_vertical = Layout::vertical([Fill(1), Fill(1)]);
    // let [top_left, bottom_left] = main_vertical.areas(top);
    // let [top_right, bottom_right] = main_vertical.areas(bottom);

    ///top title bar///////////////////////////////////////////////////////////////
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
    ///////////////////////////////////////////////////////////////////////////////
    ///bottom gauge bar////////////////////////////////////////////////////////////
    let chunks = Layout::vertical([Constraint::Length(1), Constraint::Length(1)])
        .margin(1)
        .split(status_area);
    let block = Block::bordered().title("Graphs");
    frame.render_widget(block, status_area);

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
    ///////////////////////////////////////////////////////////////////////////////
    let logs: Vec<(&str, &str)> = vec![
        ("INFO", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("INFO", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("INFO", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("ERROR", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("ERROR", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("ERROR", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("ERROR", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
    ];

    let mut text = vec![];

    for log in logs {
        let color = match &log.0 {
            &"INFO" => Color::Blue,
            &"DEBUG" => Color::Green,
            &"WARNING" => Color::Yellow,
            &"ERROR" => Color::Red,
            _ => Color::White,
        };
        text.push(text::Line::from(vec![
            Span::styled(
                format!("dd-mm-yyyy hh-mm-ss-mmm [{}] ", log.0),
                Style::default().fg(color),
            ),
            Span::from(log.1),
        ]));
    }

    let log = Paragraph::new(text)
        .block(Block::bordered().title("Connections Log"))
        .wrap(Wrap { trim: false });

    let logs: Vec<(&str, &str)> = vec![
        ("INFO", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("INFO", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("INFO", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("ERROR", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("ERROR", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("ERROR", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("WARNING", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("DEBUG", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
        ("ERROR", "lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
    ];

    let mut text = vec![];

    for log in logs {
        let color = match &log.0 {
            &"INFO" => Color::Blue,
            &"DEBUG" => Color::Green,
            &"WARNING" => Color::Yellow,
            &"ERROR" => Color::Red,
            _ => Color::White,
        };
        text.push(text::Line::from(vec![
            Span::styled(
                format!("dd-mm-yyyy hh-mm-ss-mmm [{}] ", log.0),
                Style::default().fg(color),
            ),
            Span::from(log.1),
        ]));
    }

    let tasks: Vec<ListItem> = text
        .iter()
        .map(|line| ListItem::new(line.clone()))
        .collect();
    let log2 = List::new(tasks)
        .block(Block::bordered().title("Download Logs (List)"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    // "dd-mm-yyyy hh-mm-ss-mmm [INFO] lorem ipsum lorem ipsum"

    let main_tabs = Layout::vertical([Length(1), Fill(1)]);
    let [top, bottom] = main_tabs.areas(main_area);

    let titles: Vec<Line> = vec!["Download", "Peers", "Files"]
        .iter()
        .map(|t| {
            let title = format!(" {} ", t); // add padding
            Line::from(Span::styled(
                title,
                Style::default().fg(Color::White).bg(Color::Black), // simulate block background
            ))
        })
        .collect();
    let tabs = Tabs::new(titles)
        .style(Style::default().white())
        .highlight_style(Style::default().red())
        .select(0)
        .divider(block::FULL)
        .padding(" ", " ");

    frame.render_widget(tabs, top);

    let main_tabs = Layout::horizontal([Fill(1), Fill(1)]);
    let [left, right] = main_tabs.areas(bottom);

    frame.render_widget(log, left);
    frame.render_widget(log2, right);
    // frame.render_widget(log, top_right);
    // frame.render_widget(Block::bordered().title("Bottom Left"), bottom_left);
    // frame.render_widget(Block::bordered().title("Bottom Right"), bottom_right);
}
