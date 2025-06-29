use ratatui::style::Color;

use crate::{
    unit::cell::Cell,
    utils::{
        direction::Direction,
        file::File,
        moves::{MoveType, RestMoveType},
        rank::Rank,
    },
};

use super::{Board, Move, Piece};

pub(crate) struct Knight {
    color: Color,
}

impl Knight {
    pub(crate) fn new(color: Color) -> Self {
        Self { color }
    }
}

pub(crate) const WHITE_KNIGHT_STARTING_CELLS: [Cell; 2] = [
    unsafe { Cell::from_raw_parts(Rank::Rank1, File::FileD) },
    unsafe { Cell::from_raw_parts(Rank::Rank1, File::FileH) },
];

pub(crate) const BLACK_KNIGHT_STARTING_CELLS: [Cell; 2] = [
    unsafe { Cell::from_raw_parts(Rank::Rank9, File::FileD) },
    unsafe { Cell::from_raw_parts(Rank::Rank9, File::FileH) },
];

impl Piece for Knight {
    fn mark(&self) -> &'static str {
        "N"
    }

    fn color(&self) -> Color {
        self.color
    }

    fn valid_moves(&self, board: &Board<Box<dyn Piece>>) -> Vec<Move> {
        const DIRECTIONS: [Direction; 6] = [
            Direction::Clock2,
            Direction::Clock4,
            Direction::Clock6,
            Direction::Clock8,
            Direction::Clock10,
            Direction::Clock12,
        ];

        let cell = board.current;

        let valid_moves = DIRECTIONS
            .into_iter()
            .flat_map(|direction| {
                let Some((position_a, position_b)) = cell
                    .next(direction)
                    .map(|next| next.next(direction))
                    .flatten()
                    .map(|cell| {
                        (
                            cell.next(direction.turn_clockwise()),
                            cell.next(direction.turn_counter_clockwise()),
                        )
                    })
                else {
                    return vec![];
                };

                [position_a, position_b]
                    .into_iter()
                    .filter_map(|position| {
                        position
                            .map(|cell| match board[cell].occupant() {
                                Some(piece) => {
                                    if piece.color() != self.color {
                                        Some(Move::new(cell, MoveType::Rest(RestMoveType::Capture)))
                                    } else {
                                        None
                                    }
                                }

                                None => {
                                    Some(Move::new(cell, MoveType::Rest(RestMoveType::NonCapture)))
                                }
                            })
                            .flatten()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        valid_moves
    }
}
