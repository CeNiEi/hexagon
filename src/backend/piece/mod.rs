use super::{board::Board, cell::Cell, moves::Move};

pub(crate) mod bishop;
pub(crate) mod king;
pub(crate) mod knight;
pub(crate) mod pawn;
pub(crate) mod queen;
pub(crate) mod rook;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Color {
    Black,
    White,
}

pub(crate) trait Piece {
    fn valid_moves(&self, board: &Board) -> Vec<Move>;
    fn color(&self) -> Color;
    fn location(&self) -> Cell;
}
