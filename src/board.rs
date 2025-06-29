use std::{
    borrow::Borrow,
    ops::{Index, IndexMut},
    ptr::NonNull,
};

use crate::{
    hexagon::Hexagon,
    pieces::{
        Piece,
        bishop::{BLACK_BISHOP_STARTING_CELLS, Bishop, WHITE_BISHOP_STARTING_CELLS},
        king::{BLACK_KING_STARTING_LOCATION, King, WHITE_KING_STARTING_LOCATION},
        knight::{BLACK_KNIGHT_STARTING_CELLS, Knight, WHITE_KNIGHT_STARTING_CELLS},
        pawn::{BLACK_PAWN_STARTING_CELLS, Pawn, WHITE_PAWN_STARTING_CELLS},
        queen::{BLACK_QUEEN_STARTING_LOCATION, Queen, WHITE_QUEEN_STARTING_LOCATION},
        rook::{BLACK_ROOK_STARTING_CELLS, Rook, WHITE_ROOK_STARTING_CELLS},
    },
    unit::cell::Cell,
    utils::{
        Mode,
        color_mode::ColorMode,
        delta::Delta,
        depth::Depth,
        direction::Direction,
        entry::Entry,
        file::File,
        moves::{Move, MoveType, PawnMoveType, RestMoveType},
        range::Range,
        rank::Rank,
    },
};

use ratatui::{
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Widget, canvas::Canvas},
};
use strum::IntoEnumIterator;

pub(crate) const TONE_BG1: Color = Color::Yellow;
pub(crate) const TONE_BG2: Color = Color::LightYellow;
pub(crate) const TONE_BG3: Color = Color::LightGreen;

pub(crate) struct Board<P> {
    inner: Vec<Entry<P>>,
    depth: Depth,
    pub(crate) current: Cell,
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

impl Board<Box<dyn Piece>> {
    fn empty(len: f64, padding: f64, depth: Depth, color_mode: ColorMode) -> Self {
        let colors = [TONE_BG1, TONE_BG2, TONE_BG3];
        let num_files = depth.file_range().remaning() as usize;

        let inner = depth
            .file_range()
            .enumerate()
            .flat_map(|(idx, file)| {
                depth
                    .rank_range(file)
                    .zip(
                        colors
                            .into_iter()
                            .cycle()
                            .skip(idx.min((num_files - 1) - idx)),
                    )
                    .map(move |(rank, color)| {
                        let mode = if file == File::default() && rank == Rank::default() {
                            Mode::Current
                        } else {
                            Mode::None
                        };

                        Entry::new(
                            Hexagon::new(rank, file, len, padding, mode, color, color_mode),
                            None,
                        )
                    })
            })
            .collect();

        Self {
            inner,
            depth,
            current: Cell::default(),
        }
    }

    pub(crate) fn new(len: f64, padding: f64, depth: Depth, color_mode: ColorMode) -> Self {
        let mut board = Self::empty(len, padding, depth, color_mode);

        // WHITE_BISHOP_STARTING_CELLS.into_iter().for_each(|cell| {
        //     board[cell].set_occupant(Box::new(Bishop::new(Color::White)) as Box<dyn Piece>)
        // });
        //
        // BLACK_BISHOP_STARTING_CELLS.into_iter().for_each(|cell| {
        //     board[cell].set_occupant(Box::new(Bishop::new(Color::Black)) as Box<dyn Piece>)
        // });
        //
        // board[WHITE_KING_STARTING_LOCATION]
        //     .set_occupant(Box::new(King::new(Color::White)) as Box<dyn Piece>);
        //
        // board[BLACK_KING_STARTING_LOCATION]
        //     .set_occupant(Box::new(King::new(Color::Black)) as Box<dyn Piece>);
        //
        // board[WHITE_QUEEN_STARTING_LOCATION]
        //     .set_occupant(Box::new(Queen::new(Color::White)) as Box<dyn Piece>);
        //
        // board[BLACK_QUEEN_STARTING_LOCATION]
        //     .set_occupant(Box::new(Queen::new(Color::Black)) as Box<dyn Piece>);
        //
        // WHITE_ROOK_STARTING_CELLS.into_iter().for_each(|cell| {
        //     board[cell].set_occupant(Box::new(Rook::new(Color::White)) as Box<dyn Piece>)
        // });
        //
        // BLACK_ROOK_STARTING_CELLS.into_iter().for_each(|cell| {
        //     board[cell].set_occupant(Box::new(Rook::new(Color::Black)) as Box<dyn Piece>)
        // });
        //
        // WHITE_KNIGHT_STARTING_CELLS.into_iter().for_each(|cell| {
        //     board[cell].set_occupant(Box::new(Knight::new(Color::White)) as Box<dyn Piece>)
        // });
        //
        // BLACK_KNIGHT_STARTING_CELLS.into_iter().for_each(|cell| {
        //     board[cell].set_occupant(Box::new(Knight::new(Color::Black)) as Box<dyn Piece>)
        // });
        //
        // WHITE_PAWN_STARTING_CELLS.into_iter().for_each(|cell| {
        //     board[cell].set_occupant(Box::new(Pawn::new(Color::White)) as Box<dyn Piece>)
        // });
        //
        // BLACK_PAWN_STARTING_CELLS.into_iter().for_each(|cell| {
        //     board[cell].set_occupant(Box::new(Pawn::new(Color::Black)) as Box<dyn Piece>)
        // });
        //
        board
    }

    pub(crate) fn start_move(&mut self) {
        let Some(occupant) = self[self.current].occupant() else {
            return;
        };

        occupant.valid_moves(&self).into_iter().for_each(|mov| {
            match mov.move_type {
                MoveType::Rest(RestMoveType::Capture)
                | MoveType::Pawn(PawnMoveType::CapturePromotion)
                | MoveType::Pawn(PawnMoveType::NormalCapture) => {
                    self[mov.move_to].hex_mut().set_mode(Mode::Capturable);
                }
                MoveType::Rest(RestMoveType::NonCapture)
                | MoveType::Pawn(PawnMoveType::NonCapturePromotion)
                | MoveType::Pawn(PawnMoveType::NonCapture) => {
                    self[mov.move_to].hex_mut().set_mode(Mode::Movable);
                }
                MoveType::Pawn(PawnMoveType::EnPassant { remove_piece_on }) => {
                    self[mov.move_to].hex_mut().set_mode(Mode::Movable);
                    self[remove_piece_on].hex_mut().set_mode(Mode::Capturable);
                }
            };
        });
    }

    pub(crate) fn abort_move(&mut self) {}
}

impl<P> Board<P> {
    //TODO: OPTIMISE
    fn board_index(&self, cell: Cell) -> usize {
        (Range::new(self.depth.first_file(), cell.file)
            .fold(0, |accum, file| accum + file.rank_range().remaning())
            + (cell.rank - self.depth.first_rank())) as usize
    }

    pub(crate) fn get(&self, cell: Cell) -> Option<&Entry<P>> {
        self.inner.get(self.board_index(cell))
    }

    pub(crate) fn get_mut(&mut self, cell: Cell) -> Option<&mut Entry<P>> {
        let idx = self.board_index(cell);
        self.inner.get_mut(idx)
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

impl Widget for &Board<Box<dyn Piece>> {
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
            .background_color(Color::Black)
            .paint(|ctx| {
                self.inner.iter().for_each(|entry| {
                    let hex = entry.hex();
                    ctx.draw(hex);

                    // let Delta { x, y } = hex.center();

                    // if let Some(piece) = entry.occupant() {
                    //     let style = match piece.color() {
                    //         Color::Black => Style::new().black().bold(),
                    //         Color::White => Style::new().white().bold(),
                    //         _ => unreachable!(),
                    //     };
                    //
                    //     ctx.print(x, y, Line::styled(piece.mark(), style));
                    // };
                });
            })
            .render(area, buf)
    }
}
