use crate::board::cell::Cell;

pub(crate) enum MoveType {
    Normal,
    Capture,
}

pub(crate) struct Move {
    move_type: MoveType,
    cell: Cell,
}

impl Move {
    pub(crate) fn new(cell: Cell, move_type: MoveType) -> Self {
        Self { cell, move_type }
    }
}
