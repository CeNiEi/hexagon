use ratatui::{
    style::{Color, Style, Stylize},
    text::Line,
};

use crate::backend::{
    board::Board,
    cell::Cell,
    direction::Direction,
    moves::{Move, MoveType},
};

use super::Piece;

pub(crate) struct Knight {
    color: Color,
}

impl Knight {
    pub(crate) fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Piece for Knight {
    fn mark(&self) -> Line<'static> {
        match self.color {
            Color::White => "N",
            Color::Black => "N",
            _ => unreachable!(),
        }
        .into()
    }

    fn color(&self) -> Color {
        self.color
    }

    fn valid_moves(&self, board: &Board) -> Vec<Move> {
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
                    .next_cell(direction)
                    .map(|next_cell| next_cell.next_cell(direction))
                    .flatten()
                    .map(|cell| {
                        (
                            cell.next_cell(direction.turn_clockwise()),
                            cell.next_cell(direction.turn_counter_clockwise()),
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
                                        Some(Move::new(cell, MoveType::Capture))
                                    } else {
                                        None
                                    }
                                }

                                None => Some(Move::new(cell, MoveType::NonCapture)),
                            })
                            .flatten()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        valid_moves
    }
}
