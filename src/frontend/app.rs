use crate::backend::board::Board;

pub(crate) struct App {
    pub(crate) terminate: bool,
    pub(crate) board: Board,
}

impl App {
    pub(crate) fn new() -> Self {
        let board = Board::new();

        Self {
            board,
            terminate: false,
        }
    }
}
