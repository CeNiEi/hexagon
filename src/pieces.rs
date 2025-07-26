use ratatui::{style::Color, widgets::canvas::Shape};

use crate::{
    board::Board,
    unit::cell::Cell,
    utils::{mark::Mark, moves::Move},
};

pub(crate) mod bishop;
pub(crate) mod king;
pub(crate) mod knight;
pub(crate) mod pawn;
pub(crate) mod queen;
pub(crate) mod rook;

pub(crate) enum PieceType {
    Bishop,
    King,
    Knight,
    Pawn,
    Queen,
    Rook,
}

pub trait Piece {
    fn color(&self) -> Color;
    fn valid_moves(&self, board: &Board , current: Cell) -> Vec<Move>;
    fn ty(&self) -> PieceType;
}
