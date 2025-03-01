pub(crate) mod app;
pub(crate) mod components;
pub(crate) mod crossterm;

use components::{fs::files_info, main::draw_charts, progress::download_info};
use ratatui::{
    layout::{Constraint, Layout},
    Frame,
};

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::vertical([
        Constraint::Length(4),
        Constraint::Fill(1),
        Constraint::Length(4),
    ])
    .split(frame.area());

    files_info(frame, chunks[0]);
    draw_charts(frame, app, chunks[1]);
    download_info(frame, app, chunks[2]);
}
