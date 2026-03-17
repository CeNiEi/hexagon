use ratatui::widgets::ScrollbarOrientation;

use crate::{
    board::Board,
    pieces::Piece,
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
                board.show_valid_moves(self.current);
                self.move_progression = MoveProgression::PossiblyMoving(self.current);
            }
            MoveProgression::PossiblyMoving(cell) => {
                board.hide_valid_moves(cell);
                self.possibly_move(cell, self.current, board);
                self.move_progression = MoveProgression::Navigation;
            }
        }
    }

    fn possibly_move(&mut self, src: Cell, dest: Cell, board: &mut Board) {
        let Some(src_occupant) = board[src].occupant() else {
            return;
        };

        let valid_move = src_occupant
            .valid_moves(board, src)
            .iter()
            .find(|mov| mov.move_to == dest)
            .cloned();

        if let Some(mov) = valid_move {
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
                        board[dest].remove_occupant();
                    }
                    PawnMoveType::NonCapturePromotion => {
                        board.move_occupant(src, dest);
                    }
                    PawnMoveType::CapturePromotion => {
                        board.move_occupant(src, dest);
                    }
                },
            }
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
