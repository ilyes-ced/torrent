use ratatui::{
    layout::{Constraint, Layout, Margin, Rect},
    style::{palette::tailwind, Color, Modifier, Style, Stylize},
    text::{self, Span, Text},
    widgets::{
        Block, BorderType, Cell, HighlightSpacing, Paragraph, Row, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Table, TableState, Wrap,
    },
    Frame,
};

use crate::app::App;

pub fn draw_files_tab(frame: &mut Frame, main_area: Rect, app: &mut App) {
    render_table(frame, main_area);

    // frame.render_widget(title, main_area);
}

//fn render_table(frame: &mut Frame, area: Rect) {}
//fn render_scrollbar(frame: &mut Frame, area: Rect) {}
//fn render_footer(frame: &mut Frame, area: Rect) {}
////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////
fn render_table(frame: &mut Frame, area: Rect) {
    let mut table_state = TableState::default();
    let rows = [
        Row::new(vec!["Row11", "Row12", "Row13"]),
        Row::new(vec!["Row21", "Row22", "Row23"]),
        Row::new(vec!["Row31", "Row32", "Row33"]),
    ];
    let widths = [
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Length(10),
    ];
    let table = Table::new(rows, widths)
        .block(Block::new().title("Table"))
        .row_highlight_style(Style::new().reversed())
        .highlight_symbol(">>");

    frame.render_stateful_widget(table, area, &mut table_state);
}
////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////
