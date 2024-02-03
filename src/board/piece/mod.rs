pub(crate) mod rook;

use crate::board::{utils::moves::Move, Board};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Color {
    Black,
    White,
}

pub(crate) trait Piece {
    fn valid_moves(&self, board: &Board) -> Vec<Move>;
    fn color(&self) -> Color;
} 
