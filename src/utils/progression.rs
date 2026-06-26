use ratatui::text::Line;

use crate::unit::cell::Cell;

#[derive(Debug, Default)]
pub(crate) enum MoveProgression {
    #[default]
    Navigation,
    PossiblyMoving(Cell),
}

impl MoveProgression {
    pub(crate) fn line(&self) -> Line<'static> {
        match self {
            Self::Navigation => Line::from("M: NAV"),
            Self::PossiblyMoving(cell) => Line::from(format!("M: SEL {}", cell.label())),
        }
    }
}
