use ratatui::style::Color;

use super::Piece;

pub(crate) struct Pawn {
    color: Color,
}

impl Piece for Pawn {}
