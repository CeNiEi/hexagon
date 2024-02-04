use std::collections::HashMap;

use ratatui::widgets::{
    canvas::{Canvas, Line},
    Block, Borders, WidgetRef,
};

use crate::{backend::piece::Color, frontend::shape::Hexagon};

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

    pub(crate) fn reset(&mut self) {}
}

impl WidgetRef for Board {
    fn render_ref(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let centre_x = area.width as f64 / 2.;
        let centre_y = area.height as f64 / 2.;

        Canvas::default()
            .x_bounds([0., area.width as f64])
            .y_bounds([0., area.height as f64])
            .paint(|ctx| {
                ctx.draw(&Hexagon {
                    x: centre_x,
                    y: centre_y,
                    side: 10.,
                    color: ratatui::style::Color::White,
                })
            })
            .render_ref(area, buf)
    }
}
