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

pub(crate) struct King {
    color: Color,
}

impl King {
    pub(crate) fn new(color: Color) -> Self {
        Self { color }
    }
}

pub(crate) const WHITE_KING_STARTING_LOCATION: Cell =
    unsafe { Cell::from_raw_parts(Rank::Rank1, File::FileG) };

pub(crate) const BLACK_KING_STARTING_LOCATION: Cell =
    unsafe { Cell::from_raw_parts(Rank::Rank10, File::FileG) };

impl Piece for King {
    fn mark(&self) -> &'static str {
        "K"
    }

    fn color(&self) -> Color {
        self.color
    }

    fn valid_moves(&self, board: &Board<Box<dyn Piece>>, current: Cell) -> Vec<Move> {
        const DIRECTIONS: [Direction; 12] = [
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

        let valid_moves = DIRECTIONS
            .into_iter()
            .filter_map(|direction| {
                let Some(next_cell) = current.next(direction) else {
                    return None;
                };

                match board[next_cell].occupant() {
                    Some(piece) => {
                        if piece.color() != self.color {
                            Some(Move::new(next_cell, MoveType::Rest(RestMoveType::Capture)))
                        } else {
                            None
                        }
                    }

                    None => Some(Move::new(
                        next_cell,
                        MoveType::Rest(RestMoveType::NonCapture),
                    )),
                }
            })
            .collect::<Vec<_>>();

        valid_moves
    }
}
