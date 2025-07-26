use ratatui::{style::Color, widgets::canvas::Shape};

use crate::{
    unit::cell::Cell,
    utils::{
        direction::Direction, file::File, mark::Mark, moves::single_direction_moves, rank::Rank,
    },
};

use super::{Board, Move, Piece};

pub(crate) struct Bishop {
    color: Color,
}

impl Bishop {
    pub(crate) fn new(color: Color) -> Self {
        Self { color }
    }
}

pub(crate) const WHITE_BISHOP_STARTING_CELLS: [Cell; 3] = [
    unsafe { Cell::from_raw_parts(Rank::Rank1, File::FileF) },
    unsafe { Cell::from_raw_parts(Rank::Rank2, File::FileF) },
    unsafe { Cell::from_raw_parts(Rank::Rank3, File::FileF) },
];

pub(crate) const BLACK_BISHOP_STARTING_CELLS: [Cell; 3] = [
    unsafe { Cell::from_raw_parts(Rank::Rank9, File::FileF) },
    unsafe { Cell::from_raw_parts(Rank::Rank10, File::FileF) },
    unsafe { Cell::from_raw_parts(Rank::Rank11, File::FileF) },
];

impl Piece for Bishop {
    fn ty(&self) -> super::PieceType {
        super::PieceType::Bishop
    }

    fn color(&self) -> Color {
        self.color
    }

    fn valid_moves(&self, board: &Board , current: Cell) -> Vec<Move> {
        const ALLOWED_DIRECTIONS: [Direction; 6] = [
            Direction::Clock1,
            Direction::Clock3,
            Direction::Clock5,
            Direction::Clock7,
            Direction::Clock9,
            Direction::Clock11,
        ];

        let valid_moves = ALLOWED_DIRECTIONS
            .into_iter()
            .flat_map(|direction| single_direction_moves(current, self.color, direction, board))
            .collect::<Vec<_>>();

        valid_moves
    }
}
