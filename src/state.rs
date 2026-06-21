use ratatui::widgets::ScrollbarOrientation;

use crate::{
    board::{Board, EnPassant},
    pieces::{PieceType, queen::Queen},
    unit::cell::Cell,
    utils::{
        direction::Direction,
        mode::Status,
        moves::{GeneralMoveType, MoveType, PawnMoveType},
        player::Player,
        progression::MoveProgression,
    },
};

#[derive(Default, Debug)]
pub(crate) struct State {
    player: Player,
    current: Cell,
    move_progression: MoveProgression,
}

impl State {
    pub(crate) fn set_current(&mut self, board: &mut Board, cell: Cell) {
        let current_cell = self.current;

        board[current_cell].hex_mut().set_current(false);
        board[cell].hex_mut().set_current(true);

        self.current = cell;
    }

    pub(crate) fn move_current(&mut self, board: &mut Board, direction: Direction) {
        let next = board.next(self.current, direction);

        if let Some(next) = next {
            self.set_current(board, next)
        }
    }

    pub(crate) fn toggle_help_or_move(&mut self, board: &mut Board) {
        match self.move_progression {
            MoveProgression::Navigation => {
                let Some(occupant) = board[self.current].occupant() else {
                    return;
                };

                if occupant.color() != self.player.color() {
                    return;
                }

                board.show_valid_moves(self.current);
                self.move_progression = MoveProgression::PossiblyMoving(self.current);
            }
            MoveProgression::PossiblyMoving(cell) => {
                board.hide_valid_moves(cell);
                if self.possibly_move(cell, self.current, board) {
                    self.player = self.player.toggle();
                }
                self.move_progression = MoveProgression::Navigation;
            }
        }
    }

    fn possibly_move(&mut self, src: Cell, dest: Cell, board: &mut Board) -> bool {
        let Some(src_occupant) = board[src].occupant() else {
            return false;
        };

        if src_occupant.color() != self.player.color() {
            return false;
        }

        let moved_piece_color = src_occupant.color();
        let moved_piece_type = src_occupant.ty();
        let valid_move = src_occupant
            .valid_moves(board, src)
            .iter()
            .find(|mov| mov.move_to == dest)
            .cloned();

        if let Some(mov) = valid_move {
            let new_en_passant = if moved_piece_type == PieceType::Pawn {
                let forward_direction = match moved_piece_color {
                    ratatui::style::Color::White => Direction::Clock12,
                    ratatui::style::Color::Black => Direction::Clock6,
                    _ => unreachable!(),
                };

                src.next(forward_direction).and_then(|passed_over| {
                    passed_over
                        .next(forward_direction)
                        .and_then(|double_step_dest| {
                            if double_step_dest == dest {
                                Some(EnPassant {
                                    captured_pawn: dest,
                                    capture_move_to: passed_over,
                                    pawn_color: moved_piece_color,
                                })
                            } else {
                                None
                            }
                        })
                })
            } else {
                None
            };

            match mov.move_type {
                MoveType::Rest(general_move_type) => match general_move_type {
                    GeneralMoveType::NonCapture => {
                        board.move_occupant(src, dest);
                    }
                    GeneralMoveType::Capture => {
                        board.move_occupant(src, dest);
                    }
                },
                MoveType::Pawn(pawn_move_type) => match pawn_move_type {
                    PawnMoveType::NonCapture => {
                        board.move_occupant(src, dest);
                    }
                    PawnMoveType::NormalCapture => {
                        board.move_occupant(src, dest);
                    }
                    PawnMoveType::EnPassant { remove_piece_on } => {
                        board.move_occupant(src, dest);
                        board[remove_piece_on].remove_occupant();
                    }
                    PawnMoveType::NonCapturePromotion => {
                        board.move_occupant(src, dest);
                        board[dest].set_occupant(Queen::new(moved_piece_color));
                    }
                    PawnMoveType::CapturePromotion => {
                        board.move_occupant(src, dest);
                        board[dest].set_occupant(Queen::new(moved_piece_color));
                    }
                },
            }

            board.clear_en_passant();
            if let Some(en_passant) = new_en_passant {
                board.set_en_passant(en_passant);
            }

            true
        } else {
            false
        }
    }

    // pub(crate) fn toggle_valid_moves(&mut self, board: &mut Board) {
    //     match self.displaying_valid_moves {
    //         Some(cell) => {
    //             self.hide_valid_moves(cell, board);
    //         }
    //         None => {
    //             self.show_valid_moves(board);
    //         }
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use ratatui::style::Color;

    use crate::{
        board::Board,
        pieces::{PieceType, pawn::Pawn},
        unit::cell::Cell,
        utils::{
            depth::Depth,
            file::File,
            fill_mode::FillMode,
            moves::{MoveType, PawnMoveType},
            player::Player,
            progression::MoveProgression,
            rank::Rank,
        },
    };

    use super::State;

    fn empty_board() -> Board {
        Board::empty(0., 0., Depth::new(6).unwrap(), FillMode::Wireframe, false)
    }

    #[test]
    fn white_pawn_non_capture_promotion_becomes_queen() {
        let src = Cell::new(Rank::Rank9, File::FileE);
        let dest = Cell::new(Rank::Rank10, File::FileE);
        let mut board = empty_board();
        board[src].set_occupant(Pawn::new(Color::White));
        let mut state = State {
            player: Player::White,
            current: src,
            move_progression: MoveProgression::Navigation,
        };

        assert!(state.possibly_move(src, dest, &mut board));

        assert!(board[src].occupant().is_none());
        let promoted = board[dest].occupant().expect("promoted piece should exist");
        assert_eq!(promoted.ty(), PieceType::Queen);
        assert_eq!(promoted.color(), Color::White);
    }

    #[test]
    fn white_pawn_capture_promotion_becomes_queen() {
        let src = Cell::new(Rank::Rank10, File::FileE);
        let dest = Cell::new(Rank::Rank11, File::FileF);
        let mut board = empty_board();
        board[src].set_occupant(Pawn::new(Color::White));
        board[dest].set_occupant(Pawn::new(Color::Black));
        let mut state = State {
            player: Player::White,
            current: src,
            move_progression: MoveProgression::Navigation,
        };

        assert!(state.possibly_move(src, dest, &mut board));

        assert!(board[src].occupant().is_none());
        let promoted = board[dest].occupant().expect("promoted piece should exist");
        assert_eq!(promoted.ty(), PieceType::Queen);
        assert_eq!(promoted.color(), Color::White);
    }

    #[test]
    fn pawn_double_step_enables_en_passant_capture() {
        let white_src = Cell::new(Rank::Rank4, File::FileE);
        let white_dest = Cell::new(Rank::Rank6, File::FileE);
        let black_src = Cell::new(Rank::Rank6, File::FileF);
        let black_dest = Cell::new(Rank::Rank5, File::FileE);
        let mut board = empty_board();
        board[white_src].set_occupant(Pawn::new(Color::White));
        board[black_src].set_occupant(Pawn::new(Color::Black));
        let mut state = State {
            player: Player::White,
            current: white_src,
            move_progression: MoveProgression::Navigation,
        };

        assert!(state.possibly_move(white_src, white_dest, &mut board));

        let en_passant_moves = board[black_src]
            .occupant()
            .unwrap()
            .valid_moves(&board, black_src);
        assert!(en_passant_moves.iter().any(|mov| {
            mov.move_to == black_dest
                && matches!(
                    mov.move_type,
                    MoveType::Pawn(PawnMoveType::EnPassant { remove_piece_on })
                        if remove_piece_on == white_dest
                )
        }));

        state.player = Player::Black;
        assert!(state.possibly_move(black_src, black_dest, &mut board));

        assert!(board[white_dest].occupant().is_none());
        assert!(board[black_src].occupant().is_none());
        let capturing_pawn = board[black_dest]
            .occupant()
            .expect("capturing pawn should move");
        assert_eq!(capturing_pawn.ty(), PieceType::Pawn);
        assert_eq!(capturing_pawn.color(), Color::Black);
    }

    #[test]
    fn en_passant_expires_after_any_other_valid_move() {
        let white_src = Cell::new(Rank::Rank4, File::FileE);
        let white_dest = Cell::new(Rank::Rank6, File::FileE);
        let black_en_passant_src = Cell::new(Rank::Rank6, File::FileF);
        let black_other_src = Cell::new(Rank::Rank7, File::FileK);
        let black_other_dest = Cell::new(Rank::Rank6, File::FileK);
        let expired_en_passant_dest = Cell::new(Rank::Rank5, File::FileE);
        let mut board = empty_board();
        board[white_src].set_occupant(Pawn::new(Color::White));
        board[black_en_passant_src].set_occupant(Pawn::new(Color::Black));
        board[black_other_src].set_occupant(Pawn::new(Color::Black));
        let mut state = State {
            player: Player::White,
            current: white_src,
            move_progression: MoveProgression::Navigation,
        };

        assert!(state.possibly_move(white_src, white_dest, &mut board));

        state.player = Player::Black;
        assert!(state.possibly_move(black_other_src, black_other_dest, &mut board));

        let moves_after_expiry = board[black_en_passant_src]
            .occupant()
            .unwrap()
            .valid_moves(&board, black_en_passant_src);
        assert!(!moves_after_expiry.iter().any(|mov| {
            mov.move_to == expired_en_passant_dest
                && matches!(
                    mov.move_type,
                    MoveType::Pawn(PawnMoveType::EnPassant { .. })
                )
        }));
        assert!(board[white_dest].occupant().is_some());
    }
}
