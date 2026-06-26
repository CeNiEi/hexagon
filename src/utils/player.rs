use ratatui::style::Color;

#[derive(Clone, Copy, Debug, Default)]
pub(crate) enum Player {
    #[default]
    White,
    Black,
}

impl Player {
    pub(crate) fn color(&self) -> Color {
        match self {
            Self::White => Color::White,
            Self::Black => Color::Black,
        }
    }

    pub(crate) fn toggle(&self) -> Player {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }

    pub(crate) fn label(&self) -> &'static str {
        match self {
            Self::White => "W",
            Self::Black => "B",
        }
    }
}
