use std::collections::HashMap;

use ratatui::widgets::{
    canvas::{Canvas, Line},
    Block, Borders, WidgetRef,
};

use crate::{
    backend::piece::Color,
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
        const SIDE: f64 = 2.5;
        const SCALE_FACTOR: f64 = 2.;

        let central_hex = Hexagon {
            x: area.width as f64 / 2.,
            y: area.height as f64 / 2.,
            scale_factor: SCALE_FACTOR,
            side: SIDE,
            color: ratatui::style::Color::White,
        };

        Canvas::default()
            .x_bounds([0., area.width as f64])
            .y_bounds([0., area.height as f64])
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
            })
            .render_ref(area, buf)
    }
}
