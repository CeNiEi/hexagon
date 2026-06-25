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
    state::State,
    unit::cell::Cell,
    utils::{
        consts::{TERM_SCALE_FACTOR, TONE_CANVAS_BG, TONE_HEX_BG1, TONE_HEX_BG2, TONE_HEX_BG3},
        delta::Delta,
        depth::Depth,
        direction::Direction,
        entry::Entry,
        file::File,
        fill_mode::FillMode,
        mark::Mark,
        mode::{HighlightMode, Status},
        moves::{GeneralMoveType, Move, MoveType, PawnMoveType},
        range::Range,
        rank::Rank,
    },
};

use ratatui::{
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{
        Block, Borders, Widget,
        canvas::{Canvas, Shape},
    },
};
use strum::IntoEnumIterator;

#[derive(Clone, Copy, Debug)]
pub(crate) struct EnPassant {
    pub(crate) captured_pawn: Cell,
    pub(crate) capture_move_to: Cell,
    pub(crate) pawn_color: Color,
}

pub(crate) struct Board {
    inner: Vec<Entry>,
    depth: Depth,
    en_passant: Option<EnPassant>,

    hide_highlights: bool,
}

#[macro_export]
macro_rules! board_set {
    ($(
        ($color: expr, $path: ident, $piece: ident $(,)?)
            on
        [$rank: expr, $file: expr $(,)?]),* $(,)?
    ) => {{
        let mut board = $crate::board::Board::empty(
            0.,
            0.,
            $crate::utils::depth::Depth::new(6).unwrap(),
            $crate::utils::fill_mode::FillMode::Wireframe,
            false,
        );

        $(
            let cell = $crate::unit::cell::Cell::try_new($rank, $file).unwrap();

            let piece = $crate::pieces::$path::$piece::new($color);

            board[cell].set_occupant(piece);

        )*

        board
    }};
}

impl Index<Cell> for Board {
    type Output = Entry;
    fn index(&self, index: Cell) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl IndexMut<Cell> for Board {
    fn index_mut(&mut self, index: Cell) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl Board {
    pub(crate) fn empty(
        len: f64,
        padding: f64,
        depth: Depth,
        fill_mode: FillMode,
        hide_highlights: bool,
    ) -> Self {
        let colors = [TONE_HEX_BG1, TONE_HEX_BG2, TONE_HEX_BG3];
        let num_files = depth.file_range().remaining() as usize;

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
                        let hex = Hexagon::new(
                            rank,
                            file,
                            len,
                            padding,
                            color,
                            fill_mode,
                            HighlightMode::new(
                                file == File::default() && rank == Rank::default(),
                                Status::default(),
                            ),
                        );

                        Entry::new(hex, None, hide_highlights)
                    })
            })
            .collect();

        Self {
            inner,
            depth,
            en_passant: None,
            hide_highlights,
        }
    }

    pub(crate) fn new(
        len: f64,
        padding: f64,
        depth: Depth,
        fill_mode: FillMode,
        hide_pieces: bool,
        hide_highlights: bool,
    ) -> Self {
        let mut board = Self::empty(len, padding, depth, fill_mode, hide_highlights);

        if !hide_pieces {
            WHITE_BISHOP_STARTING_CELLS
                .into_iter()
                .for_each(|cell| board[cell].set_occupant(Bishop::new(Color::White)));

            BLACK_BISHOP_STARTING_CELLS
                .into_iter()
                .for_each(|cell| board[cell].set_occupant(Bishop::new(Color::Black)));

            board[WHITE_KING_STARTING_LOCATION].set_occupant(King::new(Color::White));

            board[BLACK_KING_STARTING_LOCATION].set_occupant(King::new(Color::Black));

            board[WHITE_QUEEN_STARTING_LOCATION].set_occupant(Queen::new(Color::White));

            board[BLACK_QUEEN_STARTING_LOCATION].set_occupant(Queen::new(Color::Black));

            WHITE_ROOK_STARTING_CELLS
                .into_iter()
                .for_each(|cell| board[cell].set_occupant(Rook::new(Color::White)));

            BLACK_ROOK_STARTING_CELLS
                .into_iter()
                .for_each(|cell| board[cell].set_occupant((Rook::new(Color::Black))));

            WHITE_KNIGHT_STARTING_CELLS
                .into_iter()
                .for_each(|cell| board[cell].set_occupant(Knight::new(Color::White)));

            BLACK_KNIGHT_STARTING_CELLS
                .into_iter()
                .for_each(|cell| board[cell].set_occupant(Knight::new(Color::Black)));

            WHITE_PAWN_STARTING_CELLS
                .into_iter()
                .for_each(|cell| board[cell].set_occupant(Pawn::new(Color::White)));

            BLACK_PAWN_STARTING_CELLS
                .into_iter()
                .for_each(|cell| board[cell].set_occupant(Pawn::new(Color::Black)));
        }

        board
    }

    pub(crate) fn show_valid_moves(&mut self, cell: Cell) {
        let Some(occupant) = self[cell].occupant() else {
            return;
        };

        occupant
            .valid_moves(&self, cell)
            .into_iter()
            .for_each(|mov| {
                match mov.move_type {
                    MoveType::Rest(GeneralMoveType::Capture)
                    | MoveType::Pawn(PawnMoveType::CapturePromotion)
                    | MoveType::Pawn(PawnMoveType::NormalCapture) => {
                        self[mov.move_to].hex_mut().set_status(Status::Capturable);
                    }
                    MoveType::Rest(GeneralMoveType::NonCapture)
                    | MoveType::Pawn(PawnMoveType::NonCapturePromotion)
                    | MoveType::Pawn(PawnMoveType::NonCapture) => {
                        self[mov.move_to].hex_mut().set_status(Status::Movable);
                    }
                    MoveType::Pawn(PawnMoveType::EnPassant { remove_piece_on }) => {
                        self[mov.move_to].hex_mut().set_status(Status::Movable);
                        self[remove_piece_on]
                            .hex_mut()
                            .set_status(Status::Capturable);
                    }
                };
            });
    }

    pub(crate) fn hide_valid_moves(&mut self, cell: Cell) {
        let Some(occupant) = self[cell].occupant() else {
            return;
        };

        occupant
            .valid_moves(&self, cell)
            .into_iter()
            .for_each(|mov| {
                match mov.move_type {
                    MoveType::Rest(GeneralMoveType::Capture)
                    | MoveType::Pawn(PawnMoveType::CapturePromotion)
                    | MoveType::Pawn(PawnMoveType::NormalCapture) => {
                        self[mov.move_to].hex_mut().set_status(Status::None);
                    }
                    MoveType::Rest(GeneralMoveType::NonCapture)
                    | MoveType::Pawn(PawnMoveType::NonCapturePromotion)
                    | MoveType::Pawn(PawnMoveType::NonCapture) => {
                        self[mov.move_to].hex_mut().set_status(Status::None);
                    }
                    MoveType::Pawn(PawnMoveType::EnPassant { remove_piece_on }) => {
                        self[mov.move_to].hex_mut().set_status(Status::None);
                        self[remove_piece_on].hex_mut().set_status(Status::None);
                    }
                };
            });
    }

    pub(crate) fn move_occupant(&mut self, src: Cell, dest: Cell) -> Option<Box<dyn Piece>> {
        let Some(src_occupant) = self[src].remove_occupant() else {
            return None;
        };

        self[dest].replace_occupant(src_occupant)
    }

    pub(crate) fn clear_en_passant(&mut self) {
        self.en_passant = None;
    }

    pub(crate) fn set_en_passant(&mut self, en_passant: EnPassant) {
        self.en_passant = Some(en_passant);
    }

    pub(crate) fn en_passant_capture(&self, attacker_color: Color, move_to: Cell) -> Option<Cell> {
        self.en_passant.and_then(|en_passant| {
            if en_passant.pawn_color != attacker_color && en_passant.capture_move_to == move_to {
                Some(en_passant.captured_pawn)
            } else {
                None
            }
        })
    }
}

impl Board {
    //TODO: OPTIMISE
    fn board_index(&self, cell: Cell) -> usize {
        (Range::new(self.depth.first_file(), cell.file).fold(0, |accum, file| {
            accum + self.depth.rank_range(file).remaining()
        }) + (cell.rank - self.depth.first_rank())) as usize
    }

    pub(crate) fn get(&self, cell: Cell) -> Option<&Entry> {
        self.inner.get(self.board_index(cell))
    }

    pub(crate) fn get_mut(&mut self, cell: Cell) -> Option<&mut Entry> {
        let idx = self.board_index(cell);
        self.inner.get_mut(idx)
    }

    // pub(crate) fn set_current(&mut self, cell: Cell) {
    //     let current_cell = self.current;
    //
    //     self[current_cell].hex_mut().set_current(false);
    //     self[cell].hex_mut().set_current(true);
    //
    //     self.current = cell;
    // }

    pub(crate) fn next(&self, cell: Cell, direction: Direction) -> Option<Cell> {
        cell.next(direction)
            .map(|cell| {
                if self.depth.file_range().contains(cell.file)
                    && self.depth.rank_range(cell.file).contains(cell.rank)
                {
                    Some(cell)
                } else {
                    None
                }
            })
            .flatten()
    }

    // pub(crate) fn move_current(&mut self, direction: Direction) {
    //     let next = self
    //         .current
    //         .next(direction)
    //         .map(|cell| {
    //             if self.depth.file_range().contains(cell.file)
    //                 && self.depth.rank_range(cell.file).contains(cell.rank)
    //             {
    //                 Some(cell)
    //             } else {
    //                 None
    //             }
    //         })
    //         .flatten();
    //
    //     if let Some(next) = next {
    //         self.set_current(next)
    //     }
    // }
}

impl Widget for &Board {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let y_dim = area.height as f64;
        let x_dim = y_dim * TERM_SCALE_FACTOR;

        Canvas::default()
            .x_bounds([-x_dim / 2., x_dim / 2.])
            .y_bounds([-y_dim / 2., y_dim / 2.])
            .block(Block::default().borders(Borders::ALL))
            .marker(ratatui::symbols::Marker::Braille)
            .background_color(TONE_CANVAS_BG)
            .paint(|ctx| {
                self.inner.iter().for_each(|entry| {
                    ctx.draw(entry);

                    // let hex = entry.hex();
                    //
                    // let base = HexagonBase::from(*hex);
                    // ctx.draw(&base);
                    //
                    // if !self.hide_highlights {
                    //     let base = HexagonHighlights::from(*hex);
                    //     ctx.draw(&base);
                    // }
                    //
                    // let Delta { x, y } = hex.center();

                    // if let Some(piece) = entry.occupant() {
                    // let style = match piece.color() {
                    //     Color::Black => Style::new().white().on_black().bold(),
                    //     Color::White => Style::new().black().on_white().bold(),
                    //     _ => unreachable!(),
                    // };
                    // ctx.print(x, y, Line::styled(piece.mark(), style));

                    // ctx.draw(piece.mark());
                    // };
                });
            })
            .render(area, buf)
    }
}
