use ratatui::{
    style::{Color, Style, Stylize},
    text::Line,
};

use super::Piece;

use crate::backend::{
    board::Board,
    cell::Cell,
    direction::Direction,
    moves::{Move, MoveType},
};

pub(crate) struct King {
    color: Color,
}

impl King {
    pub(crate) fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Piece for King {
    fn mark(&self) -> Line<'static> {
        match self.color {
            Color::White => "♔",
            Color::Black => "♚",
            _ => unreachable!(),
        }
        .into()
    }

    fn color(&self) -> Color {
        self.color
    }

    fn valid_moves(&self, cell: &Cell, board: &Board) -> Vec<Move> {
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
                let Some(next_cell) = cell.next_cell(direction) else {
                    return None;
                };

                match board[next_cell].occupant() {
                    Some(piece) => {
                        if piece.color() != self.color {
                            Some(Move::new(next_cell, MoveType::Capture))
                        } else {
                            None
                        }
                    }

                    None => Some(Move::new(next_cell, MoveType::NonCapture)),
                }
            })
            .collect::<Vec<_>>();

        valid_moves
    }
}
