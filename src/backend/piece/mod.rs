use ratatui::{style::Color, text::Line};

use super::{board::Board, cell::Cell, moves::Move};

pub(crate) mod bishop;
pub(crate) mod king;
pub(crate) mod knight;
pub(crate) mod pawn;
pub(crate) mod queen;
pub(crate) mod rook;

pub(crate) trait Piece {
    fn valid_moves(&self, board: &Board) -> Vec<Move>;
    fn color(&self) -> Color;
    fn mark(&self) -> Line<'static>;
}
