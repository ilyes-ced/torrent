use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::{self, block},
    text::{self, Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Tabs, Wrap},
    Frame,
};
use Constraint::{Fill, Length, Min, Percentage};

use crate::app::App;

pub fn draw_files_tab(frame: &mut Frame, main_area: Rect, app: &mut App) {}
