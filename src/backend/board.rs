use std::{
    collections::{HashMap, HashSet},
    fmt::write,
    ops::{Index, IndexMut},
};

use ratatui::{
    layout::Alignment,
    style::Color,
    text::{Line, Text},
    widgets::{canvas::Canvas, Block, Borders, WidgetRef},
};

use crate::backend::constants::ALL_CELLS;

use super::{
    cell::{file::File, rank::Rank, Cell, HighlightLevel},
    constants::{
        BLACK_BISHOP_STARTING_LOCATION, BLACK_KING_STARTING_LOCATION,
        BLACK_KNIGHT_STARTING_LOCATION, BLACK_PAWN_STARTING_LOCATIONS,
        BLACK_QUEEN_STARTING_LOCATION, BLACK_ROOK_STARTING_LOCATION,
        WHITE_BISHOP_STARTING_LOCATION, WHITE_KING_STARTING_LOCATION,
        WHITE_KNIGHT_STARTING_LOCATION, WHITE_PAWN_STARTING_LOCATIONS,
        WHITE_QUEEN_STARTING_LOCATION, WHITE_ROOK_STARTING_LOCATION,
    },
    direction::Direction,
    moves::MoveType,
    piece::{
        bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook, Piece,
    },
};

#[derive(Clone, Copy)]
pub(crate) struct Entry<P> {
    cell: Cell,
    occupant: Option<P>,
}

impl<P> Entry<P> {
    pub(crate) fn new(cell: Cell, occupant: Option<P>) -> Self {
        Self { cell, occupant }
    }

    pub(crate) fn cell(&self) -> &Cell {
        &self.cell
    }

    pub(crate) fn cell_mut(&mut self) -> &mut Cell {
        &mut self.cell
    }

    pub(crate) fn occupant(&self) -> Option<&P> {
        self.occupant.as_ref()
    }

    pub(crate) fn occupant_mut(&mut self) -> Option<&mut P> {
        self.occupant.as_mut()
    }
}

pub(crate) struct Board {
    pub(crate) inner: [[Option<Entry<Box<dyn Piece>>>; 11]; 11],
    pub(crate) current: Cell,
    pub(crate) highlited: Vec<Cell>,
}

impl Index<Cell> for Board {
    type Output = Entry<Box<dyn Piece>>;
    fn index(&self, index: Cell) -> &Self::Output {
        let (x, y) = index - Cell::new(Rank::Rank1, File::FileA);

        self.inner[x as usize][y as usize]
            .as_ref()
            .expect("[FATAL]: Indexing into the board using a valid cell AFTER initialization should never fail")
    }
}

impl IndexMut<Cell> for Board {
    fn index_mut(&mut self, index: Cell) -> &mut Self::Output {
        let (x, y) = index - Cell::new(Rank::Rank1, File::FileA);

        self.inner[x as usize][y as usize]
            .as_mut()
            .expect("[FATAL]: Indexing into the board using a valid cell AFTER initialization should never fail")
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut inner = std::array::from_fn(|_| std::array::from_fn(|_| None));

        ALL_CELLS.into_iter().for_each(|cell| {
            let (x, y) = cell - Cell::new(Rank::Rank1, File::FileA);

            inner[x as usize][y as usize] = Some(Entry::new(cell, None));
        });

        Self {
            inner,
            current: Cell::default(),
            highlited: Vec::new(),
        }
    }
}

impl Board {
    pub(crate) fn new() -> Self {
        let mut board = Board::default();

        WHITE_ROOK_STARTING_LOCATION.into_iter().for_each(|cell| {
            board[cell] = Entry::new(
                cell,
                Some(Box::new(Rook::new(Color::White)) as Box<dyn Piece>),
            );
        });

        BLACK_ROOK_STARTING_LOCATION.into_iter().for_each(|cell| {
            board[cell] = Entry::new(
                cell,
                Some(Box::new(Rook::new(Color::Black)) as Box<dyn Piece>),
            );
        });

        WHITE_KNIGHT_STARTING_LOCATION.into_iter().for_each(|cell| {
            board[cell] = Entry::new(
                cell,
                Some(Box::new(Knight::new(Color::White)) as Box<dyn Piece>),
            );
        });

        BLACK_KNIGHT_STARTING_LOCATION.into_iter().for_each(|cell| {
            board[cell] = Entry::new(
                cell,
                Some(Box::new(Knight::new(Color::Black)) as Box<dyn Piece>),
            );
        });

        WHITE_BISHOP_STARTING_LOCATION.into_iter().for_each(|cell| {
            board[cell] = Entry::new(
                cell,
                Some(Box::new(Bishop::new(Color::White)) as Box<dyn Piece>),
            );
        });

        BLACK_BISHOP_STARTING_LOCATION.into_iter().for_each(|cell| {
            board[cell] = Entry::new(
                cell,
                Some(Box::new(Bishop::new(Color::Black)) as Box<dyn Piece>),
            );
        });

        WHITE_PAWN_STARTING_LOCATIONS.into_iter().for_each(|cell| {
            board[cell] = Entry::new(
                cell,
                Some(Box::new(Pawn::new(Color::White)) as Box<dyn Piece>),
            );
        });

        BLACK_PAWN_STARTING_LOCATIONS.into_iter().for_each(|cell| {
            board[cell] = Entry::new(
                cell,
                Some(Box::new(Pawn::new(Color::Black)) as Box<dyn Piece>),
            );
        });

        board[WHITE_QUEEN_STARTING_LOCATION] = Entry::new(
            WHITE_QUEEN_STARTING_LOCATION,
            Some(Box::new(Queen::new(Color::White)) as Box<dyn Piece>),
        );

        board[BLACK_QUEEN_STARTING_LOCATION] = Entry::new(
            BLACK_QUEEN_STARTING_LOCATION,
            Some(Box::new(Queen::new(Color::Black)) as Box<dyn Piece>),
        );

        board[WHITE_KING_STARTING_LOCATION] = Entry::new(
            WHITE_KING_STARTING_LOCATION,
            Some(Box::new(King::new(Color::White)) as Box<dyn Piece>),
        );

        board[BLACK_KING_STARTING_LOCATION] = Entry::new(
            BLACK_KING_STARTING_LOCATION,
            Some(Box::new(King::new(Color::Black)) as Box<dyn Piece>),
        );

        board[Cell::default()]
            .cell_mut()
            .set_highlight_level(super::cell::HighlightLevel::Current);

        board
    }

    pub(crate) fn select_current(&mut self) {
        self[self.current]
            .occupant()
            .map(|occupant| occupant.valid_moves(&self))
            .unwrap_or_default()
            .into_iter()
            .for_each(|mv| {
                let highlight_level = match mv.move_type {
                    MoveType::NonCapture => HighlightLevel::Movable,
                    MoveType::Promotion => HighlightLevel::Promotable,
                    MoveType::Capture => HighlightLevel::Caputreable,
                    MoveType::EnPassant(_) => unimplemented!(),
                };

                self[mv.cell]
                    .cell_mut()
                    .set_highlight_level(highlight_level);

                self.highlited.push(mv.cell);
            });
    }

    pub(crate) fn move_current(&mut self, direction: Direction) {
        let next = self.current.next_cell(direction);

        if let Some(next) = next {
            self.set_current(next)
        }
    }

    fn set_current(&mut self, cell: Cell) {
        let current = self.current;

        self[current]
            .cell_mut()
            .set_highlight_level(super::cell::HighlightLevel::None);

        self.highlited.clone().into_iter().for_each(|cell| {
            self[cell]
                .cell_mut()
                .set_highlight_level(super::cell::HighlightLevel::None);
        });

        self[cell]
            .cell_mut()
            .set_highlight_level(super::cell::HighlightLevel::Current);

        self.current = cell;

        self.highlited.clear();
    }

    pub(crate) fn test_directions(&mut self, direction: Direction) {
        let next = self.current.next_cell(direction);

        if let Some(next) = next {
            self[next]
                .cell_mut()
                .set_highlight_level(HighlightLevel::Caputreable);

            self.highlited.push(next);
        }
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
                self.inner.iter().for_each(|row| {
                    row.iter().for_each(|entry| {
                        if let Some(entry) = entry {
                            ctx.draw(entry.cell());

                            if let Some(occupant) = entry.occupant() {
                                let (x, y) = entry.cell().get_center();

                                ctx.print(x, y, occupant.mark());
                            }
                        };
                    });
                });
            })
            .render_ref(area, buf)
    }
}
