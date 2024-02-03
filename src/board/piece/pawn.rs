use super::{Color, Piece};

use std::any::Any;

use crate::board::{
    cell::Cell,
    utils::{
        constants::{BLACK_PAWN_STARTING_LOCATIONS, WHITE_PAWN_STARTING_LOCATIONS},
        direction::Direction,
        moves::{Move, MoveType},
    },
    Board,
};

pub(crate) struct Pawn {
    location: Cell,
    color: Color,
    en_passant_able: bool,
}

impl Piece for Pawn {
    fn color(&self) -> Color {
        self.color
    }

    fn location(&self) -> Cell {
        self.location
    }

    fn valid_moves(&self, board: &Board) -> Vec<Move> {
        let forward_direction = Direction::Clock12;

        let at_starting_pos = WHITE_PAWN_STARTING_LOCATIONS
            .get()
            .map(|starting_locations| starting_locations.contains(&self.location))
            .expect("[Fatal]: STARTING_LOCATIONS should be already initialized");

        let iter_fn =
            |current_cell: &Cell| -> Option<Cell> { current_cell.next_cell(forward_direction) };

        let mut valid_moves = if at_starting_pos {
            std::iter::successors(self.location.next_cell(forward_direction), iter_fn).take(2)
        } else {
            std::iter::successors(self.location.next_cell(forward_direction), iter_fn).take(1)
        }
        .fold(
            (vec![], false),
            |(mut moves_in_curr_direction, encountered): (Vec<Move>, _), cell| {
                if encountered {
                    (moves_in_curr_direction, encountered)
                } else {
                    let encountered = match board.inner.get(&cell) {
                        Some(_) => true,
                        None => {
                            let promotion_cell = cell.next_cell(forward_direction).is_none();

                            if promotion_cell {
                                moves_in_curr_direction.push(Move::new(cell, MoveType::Promotion));
                            } else {
                                moves_in_curr_direction.push(Move::new(cell, MoveType::Normal));
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
            self.location.next_cell(forward_direction.turn_clockwise()),
            self.location
                .next_cell(forward_direction.turn_counter_clockwise()),
        ]
        .into_iter()
        .filter_map(|position| {
            position
                .map(|cell| match board.inner.get(&cell) {
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

                        board
                            .inner
                            .get(&en_passant_cell)
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
