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

pub(crate) struct Bishop {
    color: Color,
}

impl Bishop {
    pub(crate) fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Piece for Bishop {
    fn mark(&self) -> Line<'static> {
        match self.color {
            Color::Black => "B",
            Color::White => "B",
            _ => unreachable!(),
        }
        .into()
    }

    fn color(&self) -> Color {
        self.color
    }

    fn valid_moves(&self, board: &Board) -> Vec<Move> {
        const DIRECTIONS: [Direction; 6] = [
            Direction::Clock1,
            Direction::Clock3,
            Direction::Clock5,
            Direction::Clock7,
            Direction::Clock9,
            Direction::Clock11,
        ];

        let cell = board.current;

        let valid_moves = DIRECTIONS
            .into_iter()
            .flat_map(|direction| {
                std::iter::successors(cell.next_cell(direction), |current_cell: &Cell| {
                    current_cell.next_cell(direction)
                })
                .fold(
                    (vec![], false),
                    |(mut moves_in_curr_direction, encountered): (Vec<Move>, _), cell| {
                        if encountered {
                            (moves_in_curr_direction, encountered)
                        } else {
                            let encountered = match board[cell].occupant() {
                                Some(piece) => {
                                    if piece.color() != self.color {
                                        moves_in_curr_direction
                                            .push(Move::new(cell, MoveType::Capture));
                                    }

                                    true
                                }
                                None => {
                                    moves_in_curr_direction
                                        .push(Move::new(cell, MoveType::NonCapture));

                                    false
                                }
                            };

                            (moves_in_curr_direction, encountered)
                        }
                    },
                )
                .0
            })
            .collect::<Vec<_>>();

        valid_moves
    }
}
