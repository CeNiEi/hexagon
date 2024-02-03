use super::{Color, Piece};
use crate::board::{
    cell::Cell,
    utils::{
        direction::Direction,
        moves::{Move, MoveType},
    },
    Board,
};

pub(crate) struct Rook {
    location: Cell,
    color: Color,
}

impl Piece for Rook {
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

        let valid_moves = DIRECTIONS
            .into_iter()
            .flat_map(|direction| {
                let current_cell = self.location;

                std::iter::successors(Some(current_cell), |current_cell| {
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
