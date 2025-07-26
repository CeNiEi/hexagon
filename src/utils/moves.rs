use ratatui::style::Color;

use crate::{board::Board, pieces::Piece};

use super::{Cell, direction::Direction};

#[derive(Clone, Copy, Debug)]
pub(crate) struct Move {
    pub(crate) move_to: Cell,
    pub(crate) move_type: MoveType,
}

impl Move {
    pub(crate) fn new(move_to: Cell, move_type: MoveType) -> Self {
        Self { move_to, move_type }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum MoveType {
    Rest(RestMoveType),
    Pawn(PawnMoveType),
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum PawnMoveType {
    NonCapture,
    NormalCapture,
    EnPassant { remove_piece_on: Cell },
    NonCapturePromotion,
    CapturePromotion,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum RestMoveType {
    NonCapture,
    Capture,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum PawnMoveState {
    Before,
    First(PawnFirstMoveState),
    After,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum PawnFirstMoveState {
    Single,
    Double,
}

pub(crate) fn single_direction_moves(
    cell: Cell,
    color: Color,
    direction: Direction,
    board: &Board ,
) -> Vec<Move> {
    std::iter::successors(cell.next(direction), |current_cell: &Cell| {
        current_cell.next(direction)
    })
    .fold(
        (vec![], false),
        |(mut moves_in_curr_direction, encountered): (Vec<Move>, _), cell| {
            if encountered {
                (moves_in_curr_direction, encountered)
            } else {
                let encountered = match board[cell].occupant() {
                    Some(piece) => {
                        if piece.color() != color {
                            moves_in_curr_direction
                                .push(Move::new(cell, MoveType::Rest(RestMoveType::Capture)));
                        }

                        true
                    }
                    None => {
                        moves_in_curr_direction
                            .push(Move::new(cell, MoveType::Rest(RestMoveType::NonCapture)));

                        false
                    }
                };

                (moves_in_curr_direction, encountered)
            }
        },
    )
    .0
}
