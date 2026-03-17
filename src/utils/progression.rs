use crate::unit::cell::Cell;

#[derive(Debug, Default)]
pub(crate) enum MoveProgression {
    #[default]
    Navigation,
    PossiblyMoving(Cell),
}
