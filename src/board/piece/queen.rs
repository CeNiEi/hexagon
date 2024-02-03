use super::{Color, Piece};

use crate::board::{
    cell::Cell,
    utils::{
        direction::Direction,
        moves::{Move, MoveType},
    },
    Board,
};

pub(crate) struct Queen {
    location: Cell,
    color: Color,
}

impl Piece for Queen {
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
            .flat_map(|direction| {
                std::iter::successors(self.location.next_cell(direction), |current_cell| {
                    current_cell.next_cell(direction)
                })
                .fold(
                    (vec![], false),
                    |(mut moves_in_curr_direction, encountered): (Vec<Move>, _), cell| {
                        if encountered {
                            (moves_in_curr_direction, encountered)
                        } else {
                            let encountered = match board.inner.get(&cell) {
                                Some(piece) => {
                                    if piece.color() != self.color {
                                        moves_in_curr_direction
                                            .push(Move::new(cell, MoveType::Capture));
                                    }

                                    true
                                }

                                None => {
                                    moves_in_curr_direction.push(Move::new(cell, MoveType::Normal));

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
