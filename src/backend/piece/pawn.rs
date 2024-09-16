use ratatui::{
    style::{Color, Style, Stylize},
    text::Line,
};

use super::Piece;

use std::any::Any;

use crate::backend::{
    board::Board,
    cell::Cell,
    constants::{BLACK_PAWN_STARTING_LOCATIONS, WHITE_PAWN_STARTING_LOCATIONS},
    direction::Direction,
    moves::{Move, MoveType},
};

pub(crate) struct Pawn {
    color: Color,
    en_passant_able: bool,
}

impl Pawn {
    pub(crate) fn new(color: Color) -> Self {
        Self {
            color,
            en_passant_able: false,
        }
    }
}

impl Piece for Pawn {
    fn color(&self) -> Color {
        self.color
    }

    fn mark(&self) -> Line<'static> {
        match self.color {
            Color::Black => "P",
            Color::White => "P",
            _ => unreachable!(),
        }
        .into()
    }

    fn valid_moves(&self, board: &Board) -> Vec<Move> {
        let forward_direction = if self.color == Color::White {
            Direction::Clock12
        } else {
            Direction::Clock6
        };

        let cell = board.current;

        let at_starting_pos = if self.color == Color::White {
            WHITE_PAWN_STARTING_LOCATIONS.contains(&cell)
        } else {
            BLACK_PAWN_STARTING_LOCATIONS.contains(&cell)
        };

        let iter_fn =
            |current_cell: &Cell| -> Option<Cell> { current_cell.next_cell(forward_direction) };

        let mut valid_moves = if at_starting_pos {
            std::iter::successors(cell.next_cell(forward_direction), iter_fn).take(2)
        } else {
            std::iter::successors(cell.next_cell(forward_direction), iter_fn).take(1)
        }
        .fold(
            (vec![], false),
            |(mut moves_in_curr_direction, encountered): (Vec<Move>, _), cell| {
                if encountered {
                    (moves_in_curr_direction, encountered)
                } else {
                    let encountered = match board[cell].occupant() {
                        Some(_) => true,
                        None => {
                            let promotion_cell = cell.next_cell(forward_direction).is_none();

                            if promotion_cell {
                                moves_in_curr_direction.push(Move::new(cell, MoveType::Promotion));
                            } else {
                                moves_in_curr_direction.push(Move::new(cell, MoveType::NonCapture));
                            };

                            false
                        }
                    };

                    (moves_in_curr_direction, encountered)
                }
            },
        )
        .0;

        let capture_moves = [
            cell.next_cell(forward_direction.turn_clockwise()),
            cell.next_cell(forward_direction.turn_counter_clockwise()),
        ]
        .into_iter()
        .filter_map(|position| {
            position
                .map(|cell| match board[cell].occupant() {
                    Some(piece) => {
                        if piece.color() != self.color {
                            Some(Move::new(cell, MoveType::Capture))
                        } else {
                            None
                        }
                    }
                    None => {
                        let Some(en_passant_cell) = cell.next_cell(forward_direction.reverse())
                        else {
                            return None;
                        };

                        board[en_passant_cell]
                            .occupant()
                            .map(|piece| {
                                if piece.color() != self.color {
                                    (piece as &dyn Any)
                                        .downcast_ref::<Self>()
                                        .map(|pawn_piece| {
                                            if pawn_piece.en_passant_able {
                                                Some(Move::new(
                                                    cell,
                                                    MoveType::EnPassant(en_passant_cell),
                                                ))
                                            } else {
                                                None
                                            }
                                        })
                                        .flatten()
                                } else {
                                    None
                                }
                            })
                            .flatten()
                    }
                })
                .flatten()
        })
        .collect::<Vec<_>>();

        valid_moves.extend(capture_moves);

        valid_moves
    }
}
