use ratatui::widgets::canvas::{Painter, Shape};

pub(crate) struct Mark<const S: char>;

impl Mark<'Q'> {
    pub fn draw(x: f64, y: f64, painter: &mut Painter) {}
}
