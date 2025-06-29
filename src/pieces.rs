use ratatui::style::Color;

use crate::{board::Board, utils::moves::Move};

pub(crate) mod bishop;
pub(crate) mod king;
pub(crate) mod knight;
pub(crate) mod pawn;
pub(crate) mod queen;
pub(crate) mod rook;

pub trait Piece {
    fn color(&self) -> Color;
    fn valid_moves(&self, board: &Board<Box<dyn Piece>>) -> Vec<Move>;
    fn mark(&self) -> &'static str;
}
