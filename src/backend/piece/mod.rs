use ratatui::style::Color;

use super::{board::Board, cell::Cell, moves::Move};

pub(crate) mod bishop;
pub(crate) mod king;
pub(crate) mod knight;
pub(crate) mod pawn;
pub(crate) mod queen;
pub(crate) mod rook;

pub(crate) trait Piece {
    fn valid_moves(&self, location: &Cell, board: &Board) -> Vec<Move>;
    fn color(&self) -> Color;
}
