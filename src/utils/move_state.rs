use super::Cell;

pub(crate) struct MoveState {
    active: Cell,
}

impl MoveState {
    pub(crate) fn new(active: Cell) -> Self {
        Self { active }
    }
}
