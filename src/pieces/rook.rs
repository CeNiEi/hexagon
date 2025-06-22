use ratatui::style::Color;

use super::Piece;

pub(crate) struct Rook {
    color: Color,
}

impl Piece for Rook {}
