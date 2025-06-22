use ratatui::style::Color;

use super::Piece;

pub(crate) struct Queen {
    color: Color,
}

impl Piece for Queen {}
