use ratatui::{
    layout::{Constraint, Layout},
    prelude::Frame,
};

use super::app::App;

pub fn render_app(app: &mut App, f: &mut Frame) {
    let horizontal = Layout::horizontal([
        Constraint::Percentage(25),
        Constraint::Percentage(75),
        Constraint::Percentage(25),
    ]);

    let [_, area, _] = horizontal.areas(f.size());

    f.render_widget(&app.board, f.size())
}
