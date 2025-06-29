use ratatui::style::Color;

use crate::unit::cell::Cell;

pub(crate) mod color_mode;
pub(crate) mod delta;
pub(crate) mod depth;
pub(crate) mod direction;
pub(crate) mod entry;
pub(crate) mod file;
mod mark;
mod move_state;
pub(crate) mod moves;
pub(crate) mod range;
pub(crate) mod rank;

pub(crate) trait Step: Sized {
    fn succ(&self) -> Option<Self>;
    fn pred(&self) -> Option<Self>;
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(crate) enum Mode {
    #[default]
    None,
    Current,
    Capturable,
    Movable,
}

impl Mode {
    pub(crate) fn color(&self) -> Color {
        match self {
            Self::None => Color::Red,
            Self::Current => Color::Red,
            Self::Movable => Color::Green,
            Self::Capturable => Color::Red,
        }
    }
}
