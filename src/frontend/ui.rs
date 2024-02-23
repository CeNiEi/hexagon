use ratatui::{
    layout::{Constraint, Layout},
    prelude::Frame,
};

use super::app::App;

pub fn render_app(app: &mut App, f: &mut Frame) {
    f.render_widget(&app.board, f.size())
}
