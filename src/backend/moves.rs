use super::cell::Cell;

pub(crate) enum MoveType {
    Promotion,
    NonCapture,
    Capture,
    EnPassant(Cell),
}

pub(crate) struct Move {
    pub(crate) move_type: MoveType,
    pub(crate) cell: Cell,
}

impl Move {
    pub(crate) fn new(cell: Cell, move_type: MoveType) -> Self {
        Self { cell, move_type }
    }
}
