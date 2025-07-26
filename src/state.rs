use crate::{
    board::Board,
    pieces::Piece,
    unit::cell::Cell,
    utils::{
        direction::Direction,
        mode::Status,
        moves::{MoveType, PawnMoveType, RestMoveType},
    },
};

#[derive(Default, Debug)]
pub(crate) struct State {
    turn: Turn,
    current: Cell,
    displaying_valid_moves: Option<Cell>,
}

impl State {
    pub(crate) fn toggle(&mut self) {
        self.turn = self.turn.toggle()
    }

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

    pub(crate) fn toggle_valid_moves(&mut self, board: &mut Board) {
        match self.displaying_valid_moves {
            Some(cell) => {
                self.hide_valid_moves(cell, board);
            }
            None => {
                self.show_valid_moves(board);
            }
        }
    }

    fn hide_valid_moves(&mut self, cell: Cell, board: &mut Board) {
        let Some(occupant) = board[cell].occupant() else {
            return;
        };

        occupant
            .valid_moves(&board, cell)
            .into_iter()
            .for_each(|mov| {
                match mov.move_type {
                    MoveType::Rest(RestMoveType::Capture)
                    | MoveType::Pawn(PawnMoveType::CapturePromotion)
                    | MoveType::Pawn(PawnMoveType::NormalCapture) => {
                        board[mov.move_to].hex_mut().set_status(Status::None);
                    }
                    MoveType::Rest(RestMoveType::NonCapture)
                    | MoveType::Pawn(PawnMoveType::NonCapturePromotion)
                    | MoveType::Pawn(PawnMoveType::NonCapture) => {
                        board[mov.move_to].hex_mut().set_status(Status::None);
                    }
                    MoveType::Pawn(PawnMoveType::EnPassant { remove_piece_on }) => {
                        board[mov.move_to].hex_mut().set_status(Status::None);
                        board[remove_piece_on].hex_mut().set_status(Status::None);
                    }
                };
            });

        self.displaying_valid_moves = None;
    }

    fn show_valid_moves(&mut self, board: &mut Board) {
        let Some(occupant) = board[self.current].occupant() else {
            return;
        };

        let mut set = false;

        occupant
            .valid_moves(&board, self.current)
            .into_iter()
            .for_each(|mov| {
                set = true;

                match mov.move_type {
                    MoveType::Rest(RestMoveType::Capture)
                    | MoveType::Pawn(PawnMoveType::CapturePromotion)
                    | MoveType::Pawn(PawnMoveType::NormalCapture) => {
                        board[mov.move_to].hex_mut().set_status(Status::Capturable);
                    }
                    MoveType::Rest(RestMoveType::NonCapture)
                    | MoveType::Pawn(PawnMoveType::NonCapturePromotion)
                    | MoveType::Pawn(PawnMoveType::NonCapture) => {
                        board[mov.move_to].hex_mut().set_status(Status::Movable);
                    }
                    MoveType::Pawn(PawnMoveType::EnPassant { remove_piece_on }) => {
                        board[mov.move_to].hex_mut().set_status(Status::Movable);
                        board[remove_piece_on]
                            .hex_mut()
                            .set_status(Status::Capturable);
                    }
                };
            });

        self.displaying_valid_moves = set.then_some(self.current);
    }
}

#[derive(Clone, Copy, Debug, Default)]
enum Turn {
    #[default]
    White,
    Black,
}

impl Turn {
    fn toggle(&self) -> Turn {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}
