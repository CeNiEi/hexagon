use std::{
    ops::{Index, IndexMut},
    ptr::NonNull,
};

use crate::{
    hexagon::Hexagon,
    pieces::Piece,
    unit::cell::Cell,
    utils::{delta::Delta, direction::Direction, entry::Entry, file::File, mode::Mode, rank::Rank},
};

use ratatui::{
    style::Color,
    text::Line,
    widgets::{Block, Borders, Widget, canvas::Canvas},
};
use strum::IntoEnumIterator;

pub(crate) struct Board<P> {
    inner: [Entry<P>; 91],
    current: Cell,
}

impl<P> Index<Cell> for Board<P> {
    type Output = Entry<P>;
    fn index(&self, index: Cell) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<P> IndexMut<Cell> for Board<P> {
    fn index_mut(&mut self, index: Cell) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl Board<()> {
    pub(crate) fn new(len: f64) -> Self {
        let mut iter = File::iter().flat_map(|file| {
            file.rank_range().map(move |rank| {
                let mode = if file == File::default() && rank == Rank::default() {
                    Mode::Current
                } else {
                    Mode::None
                };

                Entry::new(Hexagon::new(rank, file, len, mode), None)
            })
        });

        let inner = std::array::from_fn::<_, 91, _>(|_| iter.next().unwrap());

        debug_assert!(iter.next().is_none());

        Self {
            inner,
            current: Cell::default(),
        }
    }
}

impl<P> Board<P> {
    pub(crate) fn get(&self, cell: Cell) -> Option<&Entry<P>> {
        self.inner.get(cell.to_board_index())
    }

    pub(crate) fn get_mut(&mut self, cell: Cell) -> Option<&mut Entry<P>> {
        self.inner.get_mut(cell.to_board_index())
    }

    pub(crate) fn set_current(&mut self, cell: Cell) {
        let current_cell = self.current;

        self[current_cell].hex_mut().set_mode(Mode::None);
        self[cell].hex_mut().set_mode(Mode::Current);

        self.current = cell;
    }

    pub(crate) fn move_current(&mut self, direction: Direction) {
        let next = self.current.next(direction);

        if let Some(next) = next {
            self.set_current(next)
        }
    }
}

impl Widget for &Board<()> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        const SCALE_FACTOR: f64 = 2.;

        let y_dim = area.height as f64;
        let x_dim = y_dim * SCALE_FACTOR;

        Canvas::default()
            .x_bounds([-x_dim / 2., x_dim / 2.])
            .y_bounds([-y_dim / 2., y_dim / 2.])
            .block(Block::default().borders(Borders::ALL))
            .marker(ratatui::symbols::Marker::Braille)
            .background_color(Color::DarkGray)
            .paint(|ctx| {
                self.inner.iter().for_each(|entry| {
                    let hex = entry.hex();
                    let Delta { x, y } = hex.center();

                    ctx.draw(hex);
                    ctx.print(x, y, Line::from("A"));
                });
            })
            .render(area, buf)
    }
}
