use super::{Color, Piece};

use crate::backend::{
    board::Board,
    cell::Cell,
    direction::Direction,
    moves::{Move, MoveType},
};

pub(crate) struct King {
    location: Cell,
    color: Color,
}

impl King {
    pub(crate) fn new(location: Cell, color: Color) -> Self {
        Self { location, color }
    }
}

impl Piece for King {
    fn color(&self) -> Color {
        self.color
    }

    fn location(&self) -> Cell {
        self.location
    }

    fn valid_moves(&self, board: &Board) -> Vec<Move> {
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
                let Some(next_cell) = self.location.next_cell(direction) else {
                    return None;
                };

                match board.inner.get(&next_cell) {
                    Some(piece) => {
                        if piece.color() != self.color {
                            Some(Move::new(next_cell, MoveType::Capture))
                        } else {
                            None
                        }
                    }

                    None => Some(Move::new(next_cell, MoveType::Normal)),
                }
            })
            .collect::<Vec<_>>();

        valid_moves
    }
}
