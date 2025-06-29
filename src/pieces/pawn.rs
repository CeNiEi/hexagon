use std::any::Any;

use ratatui::style::Color;

use crate::{
    unit::cell::Cell,
    utils::{
        direction::Direction,
        file::File,
        moves::{MoveType, PawnFirstMoveState, PawnMoveState, PawnMoveType},
        rank::Rank,
    },
};

use super::{Board, Move, Piece};

pub(crate) const WHITE_PAWN_STARTING_CELLS: [Cell; 9] = [
    unsafe { Cell::from_raw_parts(Rank::Rank1, File::FileB) },
    unsafe { Cell::from_raw_parts(Rank::Rank2, File::FileC) },
    unsafe { Cell::from_raw_parts(Rank::Rank3, File::FileD) },
    unsafe { Cell::from_raw_parts(Rank::Rank4, File::FileE) },
    unsafe { Cell::from_raw_parts(Rank::Rank5, File::FileF) },
    unsafe { Cell::from_raw_parts(Rank::Rank4, File::FileG) },
    unsafe { Cell::from_raw_parts(Rank::Rank3, File::FileH) },
    unsafe { Cell::from_raw_parts(Rank::Rank2, File::FileI) },
    unsafe { Cell::from_raw_parts(Rank::Rank1, File::FileK) },
];

pub(crate) const BLACK_PAWN_STARTING_CELLS: [Cell; 9] = [
    unsafe { Cell::from_raw_parts(Rank::Rank7, File::FileB) },
    unsafe { Cell::from_raw_parts(Rank::Rank7, File::FileC) },
    unsafe { Cell::from_raw_parts(Rank::Rank7, File::FileD) },
    unsafe { Cell::from_raw_parts(Rank::Rank7, File::FileE) },
    unsafe { Cell::from_raw_parts(Rank::Rank7, File::FileF) },
    unsafe { Cell::from_raw_parts(Rank::Rank7, File::FileG) },
    unsafe { Cell::from_raw_parts(Rank::Rank7, File::FileH) },
    unsafe { Cell::from_raw_parts(Rank::Rank7, File::FileI) },
    unsafe { Cell::from_raw_parts(Rank::Rank7, File::FileK) },
];

pub(crate) const WHITE_PAWN_PROMOTION_CELLS: [Cell; 11] = [
    unsafe { Cell::from_raw_parts(Rank::Rank6, File::FileA) },
    unsafe { Cell::from_raw_parts(Rank::Rank7, File::FileB) },
    unsafe { Cell::from_raw_parts(Rank::Rank8, File::FileC) },
    unsafe { Cell::from_raw_parts(Rank::Rank9, File::FileD) },
    unsafe { Cell::from_raw_parts(Rank::Rank10, File::FileE) },
    unsafe { Cell::from_raw_parts(Rank::Rank11, File::FileF) },
    unsafe { Cell::from_raw_parts(Rank::Rank10, File::FileG) },
    unsafe { Cell::from_raw_parts(Rank::Rank9, File::FileH) },
    unsafe { Cell::from_raw_parts(Rank::Rank8, File::FileI) },
    unsafe { Cell::from_raw_parts(Rank::Rank7, File::FileK) },
    unsafe { Cell::from_raw_parts(Rank::Rank6, File::FileL) },
];

pub(crate) const BLACK_PAWN_PROMOTION_CELLS: [Cell; 11] = [
    unsafe { Cell::from_raw_parts(Rank::Rank1, File::FileA) },
    unsafe { Cell::from_raw_parts(Rank::Rank1, File::FileB) },
    unsafe { Cell::from_raw_parts(Rank::Rank1, File::FileC) },
    unsafe { Cell::from_raw_parts(Rank::Rank1, File::FileD) },
    unsafe { Cell::from_raw_parts(Rank::Rank1, File::FileE) },
    unsafe { Cell::from_raw_parts(Rank::Rank1, File::FileF) },
    unsafe { Cell::from_raw_parts(Rank::Rank1, File::FileG) },
    unsafe { Cell::from_raw_parts(Rank::Rank1, File::FileH) },
    unsafe { Cell::from_raw_parts(Rank::Rank1, File::FileI) },
    unsafe { Cell::from_raw_parts(Rank::Rank1, File::FileK) },
    unsafe { Cell::from_raw_parts(Rank::Rank1, File::FileL) },
];

pub(crate) struct Pawn {
    color: Color,
    en_passant_state: PawnMoveState,
}

impl Pawn {
    pub(crate) fn en_passant_able(&self) -> bool {
        self.en_passant_state == PawnMoveState::First(PawnFirstMoveState::Double)
    }

    pub(crate) fn new(color: Color) -> Self {
        Self {
            color,
            en_passant_state: PawnMoveState::Before,
        }
    }
}

impl Piece for Pawn {
    fn mark(&self) -> &'static str {
        "P"
    }
    fn color(&self) -> Color {
        self.color
    }

    fn valid_moves(&self, board: &Board<Box<dyn Piece>>) -> Vec<Move> {
        let forward_direction = match self.color {
            Color::White => Direction::Clock12,
            Color::Black => Direction::Clock6,
            _ => unreachable!(),
        };

        let cell = board.current;

        let at_starting_pos = if self.color == Color::White {
            WHITE_PAWN_STARTING_CELLS.contains(&cell)
        } else {
            BLACK_PAWN_STARTING_CELLS.contains(&cell)
        };

        let num_steps_allowed = if at_starting_pos { 2 } else { 1 };

        enum PawnMove {
            NonCapture(Cell),
            Capture(Cell),
            EnPassant { move_to: Cell, captures: Cell },
        }

        let non_capture_moves =
            std::iter::successors(cell.next(forward_direction), |current_cell| {
                current_cell.next(forward_direction)
            })
            .take(num_steps_allowed)
            .scan(false, |encountered, cell| {
                if *encountered {
                    return None;
                };

                match board[cell].occupant() {
                    Some(_) => {
                        *encountered = true;
                        None
                    }
                    None => Some(cell),
                }
            })
            .map(|cell| PawnMove::NonCapture(cell));

        let capture_moves = [
            cell.next(forward_direction.turn_clockwise()),
            cell.next(forward_direction.turn_counter_clockwise()),
        ]
        .into_iter()
        .filter_map(|position| {
            position
                .map(|cell| match board[cell].occupant() {
                    Some(piece) => {
                        if piece.color() != self.color {
                            Some(PawnMove::Capture(cell))
                        } else {
                            None
                        }
                    }
                    None => {
                        let Some(en_passant_cell) = cell.next(forward_direction.reverse()) else {
                            return None;
                        };

                        board[en_passant_cell]
                            .occupant()
                            .map(|piece| {
                                if piece.color() != self.color {
                                    (piece as &dyn Any)
                                        .downcast_ref::<Self>()
                                        .map(|pawn_piece| {
                                            if pawn_piece.en_passant_able() {
                                                Some(PawnMove::EnPassant {
                                                    move_to: cell,
                                                    captures: en_passant_cell,
                                                })
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
        });

        non_capture_moves
            .chain(capture_moves)
            .map(|mov| match mov {
                PawnMove::NonCapture(cell) => {
                    let at_promotion_cell = if self.color == Color::Black {
                        BLACK_PAWN_PROMOTION_CELLS.contains(&cell)
                    } else {
                        WHITE_PAWN_PROMOTION_CELLS.contains(&cell)
                    };

                    if at_promotion_cell {
                        Move::new(cell, MoveType::Pawn(PawnMoveType::NonCapturePromotion))
                    } else {
                        Move::new(cell, MoveType::Pawn(PawnMoveType::NonCapture))
                    }
                }
                PawnMove::EnPassant { move_to, captures } => Move::new(
                    move_to,
                    MoveType::Pawn(PawnMoveType::EnPassant {
                        remove_piece_on: captures,
                    }),
                ),
                PawnMove::Capture(cell) => {
                    let at_promotion_cell = if self.color == Color::Black {
                        BLACK_PAWN_PROMOTION_CELLS.contains(&cell)
                    } else {
                        WHITE_PAWN_PROMOTION_CELLS.contains(&cell)
                    };

                    if at_promotion_cell {
                        Move::new(cell, MoveType::Pawn(PawnMoveType::CapturePromotion))
                    } else {
                        Move::new(cell, MoveType::Pawn(PawnMoveType::NormalCapture))
                    }
                }
            })
            .collect()
    }
}
