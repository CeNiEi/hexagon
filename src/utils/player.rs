#[derive(Clone, Copy, Debug, Default)]
pub(crate) enum Player {
    #[default]
    White,
    Black,
}

impl Player {
    fn toggle(&self) -> Player {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}
