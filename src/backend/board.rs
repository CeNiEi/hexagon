use std::collections::HashMap;

use ratatui::widgets::{
    canvas::{Canvas, Circle, Line},
    Block, Borders, WidgetRef,
};

use crate::{
    backend::{constants::GRAY_BG, piece::Color},
    frontend::{direction::Direction, shape::Hexagon},
};

use super::{
    cell::Cell,
    constants::{
        WHITE_BISHOP_STARTING_LOCATION, WHITE_KING_STARTING_LOCATION,
        WHITE_KNIGHT_STARTING_LOCATION, WHITE_PAWN_STARTING_LOCATIONS,
        WHITE_QUEEN_STARTING_LOCATION, WHITE_ROOK_STARTING_LOCATION,
    },
    piece::{
        bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook, Piece,
    },
};

pub(crate) struct Board {
    pub(crate) inner: HashMap<Cell, Box<dyn Piece>>,
}

impl Board {
    pub(crate) fn new() -> Self {
        let mut inner = HashMap::new();

        WHITE_PAWN_STARTING_LOCATIONS.into_iter().for_each(|cell| {
            inner.insert(
                cell,
                Box::new(Pawn::new(cell, Color::White)) as Box<dyn Piece>,
            );
        });

        WHITE_KNIGHT_STARTING_LOCATION.into_iter().for_each(|cell| {
            inner.insert(
                cell,
                Box::new(Knight::new(cell, Color::White)) as Box<dyn Piece>,
            );
        });

        WHITE_ROOK_STARTING_LOCATION.into_iter().for_each(|cell| {
            inner.insert(
                cell,
                Box::new(Rook::new(cell, Color::White)) as Box<dyn Piece>,
            );
        });

        WHITE_BISHOP_STARTING_LOCATION.into_iter().for_each(|cell| {
            inner.insert(
                cell,
                Box::new(Bishop::new(cell, Color::White)) as Box<dyn Piece>,
            );
        });

        inner.insert(
            WHITE_QUEEN_STARTING_LOCATION,
            Box::new(Queen::new(WHITE_QUEEN_STARTING_LOCATION, Color::White)) as Box<dyn Piece>,
        );

        inner.insert(
            WHITE_KING_STARTING_LOCATION,
            Box::new(King::new(WHITE_KING_STARTING_LOCATION, Color::White)) as Box<dyn Piece>,
        );

        Self { inner }
    }

    pub(crate) fn reset(&mut self) {
        todo!();
    }
}

impl WidgetRef for Board {
    fn render_ref(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        const SIDE: f64 = 3.;
        const SCALE_FACTOR: f64 = 2.;

        let central_hex = Hexagon {
            x: 0.,
            y: 0.,
            side: SIDE,
            color: GRAY_BG,
        };

        let y_dim = area.height as f64;
        let x_dim = y_dim * SCALE_FACTOR;
        // let x_dim = y_dim * SCALE_FACTOR;

        Canvas::default()
            .x_bounds([-x_dim / 2., x_dim / 2.])
            .y_bounds([-y_dim / 2., y_dim / 2.])
            .block(Block::default().borders(Borders::ALL))
            .marker(ratatui::symbols::Marker::Braille)
            .paint(|ctx| {
                let top_ranks =
                    std::iter::successors(Some((central_hex, 11)), |(hex, count_in_file)| {
                        Some((hex.next(Direction::N), *count_in_file - 2))
                    })
                    .skip(1)
                    .take(5)
                    .flat_map(|(hex, count_in_file)| {
                        let right_files =
                            std::iter::successors(Some(hex), |hex| Some(hex.next(Direction::NE)))
                                .take(count_in_file / 2 + 1);

                        let left_files =
                            std::iter::successors(Some(hex), |hex| Some(hex.next(Direction::NW)))
                                .skip(1)
                                .take(count_in_file / 2);

                        right_files.chain(left_files)
                    });

                let bottom_ranks =
                    std::iter::successors(Some(central_hex), |hex| Some(hex.next(Direction::S)))
                        .take(6)
                        .flat_map(|hex| {
                            let right_files = std::iter::successors(Some(hex), |hex| {
                                Some(hex.next(Direction::NE))
                            })
                            .take(6);

                            let left_files = std::iter::successors(Some(hex), |hex| {
                                Some(hex.next(Direction::NW))
                            })
                            .skip(1)
                            .take(5);

                            right_files.chain(left_files)
                        });

                top_ranks.chain(bottom_ranks).for_each(|hex| {
                    ctx.draw(&hex);
                });

                // ctx.draw(&central_hex);
                // ctx.draw(&central_hex.next(Direction::NW));
                // ctx.draw(&central_hex.next(Direction::NE));
                // ctx.draw(&central_hex.next(Direction::S));
                // ctx.draw(&central_hex.next(Direction::SE));
                // ctx.draw(&central_hex.next(Direction::SW));
                // ctx.draw(&central_hex.next(Direction::N));
            })
            .render_ref(area, buf)
    }
}
