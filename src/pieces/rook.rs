use ratatui::style::Color;

use crate::{
    board::Board,
    unit::cell::Cell,
    utils::{
        direction::Direction,
        file::File,
        moves::{Move, single_direction_moves},
        rank::Rank,
    },
};

use super::Piece;

pub(crate) const BLACK_ROOK_STARTING_CELLS: [Cell; 2] = [
    unsafe { Cell::from_raw_parts(Rank::Rank8, File::FileC) },
    unsafe { Cell::from_raw_parts(Rank::Rank8, File::FileI) },
];

pub(crate) const WHITE_ROOK_STARTING_CELLS: [Cell; 2] = [
    unsafe { Cell::from_raw_parts(Rank::Rank1, File::FileC) },
    unsafe { Cell::from_raw_parts(Rank::Rank1, File::FileI) },
];

pub(crate) struct Rook {
    color: Color,
}

impl Rook {
    pub(crate) fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Piece for Rook {
    fn mark(&self) -> &'static str {
        "R"
    }

    fn color(&self) -> Color {
        self.color
    }

    fn valid_moves(&self, board: &Board<Box<dyn Piece>>, current: Cell) -> Vec<Move> {
        const ALLOWED_DIRECTIONS: [Direction; 6] = [
            Direction::Clock2,
            Direction::Clock4,
            Direction::Clock6,
            Direction::Clock8,
            Direction::Clock10,
            Direction::Clock12,
        ];

        let valid_moves = ALLOWED_DIRECTIONS
            .into_iter()
            .flat_map(|direction| single_direction_moves(current, self.color, direction, board))
            .collect::<Vec<_>>();

        valid_moves
    }
}
