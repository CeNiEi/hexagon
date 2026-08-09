use std::{
    borrow::Borrow,
    ops::{Index, IndexMut},
    ptr::NonNull,
};

use crate::{
    hexagon::Hexagon,
    pieces::{
        Piece, PieceType,
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

struct MoveUndo {
    src: Cell,
    dest: Cell,
    move_type: MoveType,
    moved_color: Color,
    captured_piece: Option<Box<dyn Piece>>,
    en_passant_captured: Option<(Cell, Box<dyn Piece>)>,
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

    pub(crate) fn new(len: f64, padding: f64, fill_mode: FillMode, hide_highlights: bool) -> Self {
        let mut board = Self::empty(len, padding, Depth::default(), fill_mode, hide_highlights);

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
            .for_each(|cell| board[cell].set_occupant(Rook::new(Color::Black)));

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

        board
    }

    pub(crate) fn preview(
        len: f64,
        padding: f64,
        depth: Depth,
        fill_mode: FillMode,
        hide_highlights: bool,
    ) -> Self {
        Self::empty(len, padding, depth, fill_mode, hide_highlights)
    }

    pub(crate) fn legal_moves(&mut self, cell: Cell) -> Vec<Move> {
        let (color, moves) = {
            let Some(occupant) = self[cell].occupant() else {
                return vec![];
            };
            (occupant.color(), occupant.valid_moves(self, cell))
        };

        moves
            .into_iter()
            .filter(|mov| {
                if self[mov.move_to]
                    .occupant()
                    .is_some_and(|piece| piece.ty() == PieceType::King)
                {
                    return false;
                }

                let Some(undo) = self.apply_for_legality(cell, *mov) else {
                    return false;
                };
                let legal = !self.is_in_check(color);
                self.undo_for_legality(undo);
                legal
            })
            .collect()
    }

    pub(crate) fn is_in_check(&self, color: Color) -> bool {
        let Some(king_cell) = self.inner.iter().find_map(|entry| {
            let piece = entry.occupant()?;
            (piece.color() == color && piece.ty() == PieceType::King).then(|| entry.hex().cell())
        }) else {
            return false;
        };

        self.inner.iter().any(|entry| {
            let Some(piece) = entry.occupant() else {
                return false;
            };

            piece.color() != color
                && piece
                    .valid_moves(self, entry.hex().cell())
                    .iter()
                    .any(|mov| mov.move_to == king_cell)
        })
    }

    pub(crate) fn has_legal_move(&mut self, color: Color) -> bool {
        let cells = self
            .inner
            .iter()
            .filter_map(|entry| {
                let piece = entry.occupant()?;
                (piece.color() == color).then(|| entry.hex().cell())
            })
            .collect::<Vec<_>>();

        cells
            .into_iter()
            .any(|cell| !self.legal_moves(cell).is_empty())
    }

    fn apply_for_legality(&mut self, src: Cell, mov: Move) -> Option<MoveUndo> {
        let moved_piece = self[src].remove_occupant()?;
        let moved_color = moved_piece.color();
        let captured_piece = self[mov.move_to].replace_occupant(moved_piece);

        let en_passant_captured = match mov.move_type {
            MoveType::Pawn(PawnMoveType::EnPassant { remove_piece_on }) => self[remove_piece_on]
                .remove_occupant()
                .map(|piece| (remove_piece_on, piece)),
            _ => None,
        };

        if mov.move_type.is_promotion() {
            self[mov.move_to].remove_occupant();
            self[mov.move_to].set_occupant(Queen::new(moved_color));
        }

        Some(MoveUndo {
            src,
            dest: mov.move_to,
            move_type: mov.move_type,
            moved_color,
            captured_piece,
            en_passant_captured,
        })
    }

    fn undo_for_legality(&mut self, undo: MoveUndo) {
        let moved_piece = self[undo.dest].remove_occupant();

        if undo.move_type.is_promotion() {
            self[undo.src].set_occupant(Pawn::new(undo.moved_color));
        } else if let Some(moved_piece) = moved_piece {
            self[undo.src].replace_occupant(moved_piece);
        } else {
            unreachable!("simulated move destination is empty");
        }

        if let Some(captured_piece) = undo.captured_piece {
            self[undo.dest].replace_occupant(captured_piece);
        }
        if let Some((cell, captured_piece)) = undo.en_passant_captured {
            self[cell].replace_occupant(captured_piece);
        }
    }

    pub(crate) fn show_valid_moves(&mut self, cell: Cell) {
        self.legal_moves(cell).into_iter().for_each(|mov| {
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
        self.legal_moves(cell).into_iter().for_each(|mov| {
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

pub(crate) struct BoardView<'a> {
    pub(crate) board: &'a Board,
    pub(crate) div: f64,
}

impl<'a> Widget for &'a BoardView<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        // let mut width = area.width;
        // let mut height = (width as f64 / TERM_SCALE_FACTOR).floor() as u16;
        // log::info!("{width}, {height}");
        log::info!("{}, {}", area.width, area.height);

        // if height == 0 || height > area.height {
        //     height = area.height;
        //     width = (height as f64 * TERM_SCALE_FACTOR).floor() as u16;
        // }

        // let render_area = ratatui::prelude::Rect {
        //     x: area.x + area.width.saturating_sub(width) / 2,
        //     y: area.y + area.height.saturating_sub(height) / 2,
        //     width,
        //     height,
        // };

        // let y_dim = render_area.height as f64;
        // let x_dim = y_dim * TERM_SCALE_FACTOR;

        let scale_factor = area.width as f64 / area.height as f64;
        log::info!("Scale Factor: {scale_factor}");
        let y_dim = area.height as f64;
        let x_dim = y_dim as f64 * TERM_SCALE_FACTOR;

        // let x_dim = area.width as f64;
        // let y_dim = x_dim as f64 / TERM_SCALE_FACTOR;

        Canvas::default()
            .x_bounds([-x_dim / self.div, x_dim / self.div])
            .y_bounds([-y_dim / 2., y_dim / 2.])
            .block(Block::default().borders(Borders::ALL))
            .marker(ratatui::symbols::Marker::Braille)
            .background_color(TONE_CANVAS_BG)
            .paint(|ctx| {
                self.board.inner.iter().for_each(|entry| {
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

#[cfg(test)]
mod tests {
    use ratatui::style::Color;

    use crate::{
        pieces::{Piece, king::King, pawn::Pawn, rook::Rook},
        unit::cell::Cell,
        utils::{
            depth::Depth,
            file::File,
            fill_mode::FillMode,
            moves::{MoveType, PawnMoveType},
            rank::Rank,
        },
    };

    use super::{Board, EnPassant};

    #[test]
    fn new_creates_full_depth_board_with_starting_pieces() {
        let board = Board::new(0., 0., FillMode::Wireframe, false);

        assert_eq!(board.inner.len(), 91);
        assert_eq!(
            board
                .inner
                .iter()
                .filter(|entry| entry.occupant().is_some())
                .count(),
            36
        );
    }

    #[test]
    fn preview_uses_requested_depth_without_pieces() {
        let board = Board::preview(0., 0., Depth::new(1).unwrap(), FillMode::Wireframe, false);

        assert_eq!(board.inner.len(), 1);
        assert!(board.inner.iter().all(|entry| entry.occupant().is_none()));
    }

    fn empty_board() -> Board {
        Board::empty(0., 0., Depth::default(), FillMode::Wireframe, false)
    }

    #[test]
    fn king_cannot_move_into_rook_attack() {
        let king_cell = Cell::new(Rank::Rank6, File::FileF);
        let target = Cell::new(Rank::Rank7, File::FileG);
        let rook_cell = Cell::new(Rank::Rank9, File::FileG);
        let mut board = empty_board();

        board[king_cell].set_occupant(King::new(Color::White));
        board[rook_cell].set_occupant(Rook::new(Color::Black));

        assert!(!board.is_in_check(Color::White));
        assert!(
            !board
                .legal_moves(king_cell)
                .iter()
                .any(|mov| mov.move_to == target)
        );
    }

    #[test]
    fn pinned_piece_cannot_expose_king() {
        let king_cell = Cell::new(Rank::Rank6, File::FileF);
        let pinned_cell = Cell::new(Rank::Rank7, File::FileF);
        let sideways_target = Cell::new(Rank::Rank7, File::FileG);
        let attacker_cell = Cell::new(Rank::Rank9, File::FileF);
        let mut board = empty_board();

        board[king_cell].set_occupant(King::new(Color::White));
        board[pinned_cell].set_occupant(Rook::new(Color::White));
        board[attacker_cell].set_occupant(Rook::new(Color::Black));

        assert!(!board.is_in_check(Color::White));
        assert!(
            !board
                .legal_moves(pinned_cell)
                .iter()
                .any(|mov| mov.move_to == sideways_target)
        );
    }

    #[test]
    fn legal_moves_cannot_capture_a_king() {
        let king_cell = Cell::new(Rank::Rank1, File::FileA);
        let rook_cell = Cell::new(Rank::Rank6, File::FileF);
        let enemy_king_cell = Cell::new(Rank::Rank9, File::FileF);
        let mut board = empty_board();

        board[king_cell].set_occupant(King::new(Color::White));
        board[rook_cell].set_occupant(Rook::new(Color::White));
        board[enemy_king_cell].set_occupant(King::new(Color::Black));

        assert!(
            board[rook_cell]
                .occupant()
                .unwrap()
                .valid_moves(&board, rook_cell)
                .iter()
                .any(|mov| mov.move_to == enemy_king_cell)
        );
        assert!(
            !board
                .legal_moves(rook_cell)
                .iter()
                .any(|mov| mov.move_to == enemy_king_cell)
        );
    }

    #[test]
    fn en_passant_is_illegal_when_it_exposes_the_king() {
        let white_king = Cell::new(Rank::Rank3, File::FileF);
        let white_pawn = Cell::new(Rank::Rank6, File::FileF);
        let black_pawn = Cell::new(Rank::Rank5, File::FileG);
        let black_rook = Cell::new(Rank::Rank9, File::FileF);
        let capture_move_to = Cell::new(Rank::Rank6, File::FileG);
        let mut board = empty_board();

        board[white_king].set_occupant(King::new(Color::White));
        board[white_pawn].set_occupant(Pawn::new(Color::White));
        board[black_pawn].set_occupant(Pawn::new(Color::Black));
        board[black_rook].set_occupant(Rook::new(Color::Black));
        board.set_en_passant(EnPassant {
            captured_pawn: black_pawn,
            capture_move_to,
            pawn_color: Color::Black,
        });

        assert!(
            board[white_pawn]
                .occupant()
                .unwrap()
                .valid_moves(&board, white_pawn)
                .iter()
                .any(|mov| {
                    mov.move_to == capture_move_to
                        && matches!(
                            mov.move_type,
                            MoveType::Pawn(PawnMoveType::EnPassant { .. })
                        )
                })
        );
        assert!(
            !board
                .legal_moves(white_pawn)
                .iter()
                .any(|mov| mov.move_to == capture_move_to)
        );
    }
}
