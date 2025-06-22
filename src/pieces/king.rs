use ratatui::style::Color;

use super::Piece;

pub(crate) struct King {
    color: Color,
}

impl Piece for King {}
