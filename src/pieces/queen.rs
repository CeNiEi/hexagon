use ratatui::style::Color;

use crate::{
    unit::cell::Cell,
    utils::{direction::Direction, file::File, moves::single_direction_moves, rank::Rank},
};

use super::{Board, Move, Piece};

pub(crate) struct Queen {
    color: Color,
}

impl Queen {
    pub(crate) fn new(color: Color) -> Self {
        Self { color }
    }
}

pub(crate) const WHITE_QUEEN_STARTING_LOCATION: Cell =
    unsafe { Cell::from_raw_parts(Rank::Rank1, File::FileE) };

pub(crate) const BLACK_QUEEN_STARTING_LOCATION: Cell =
    unsafe { Cell::from_raw_parts(Rank::Rank10, File::FileE) };

impl Piece for Queen {
    fn mark(&self) -> &'static str {
        "Q"
    }

    fn color(&self) -> Color {
        self.color
    }

    fn valid_moves(&self, board: &Board<Box<dyn Piece>>) -> Vec<Move> {
        const ALLOWED_DIRECTIONS: [Direction; 12] = [
            Direction::Clock1,
            Direction::Clock2,
            Direction::Clock3,
            Direction::Clock4,
            Direction::Clock5,
            Direction::Clock6,
            Direction::Clock7,
            Direction::Clock8,
            Direction::Clock9,
            Direction::Clock10,
            Direction::Clock11,
            Direction::Clock12,
        ];

        let cell = board.current;

        let valid_moves = ALLOWED_DIRECTIONS
            .into_iter()
            .flat_map(|direction| single_direction_moves(cell, self.color, direction, board))
            .collect::<Vec<_>>();

        valid_moves
    }
}
