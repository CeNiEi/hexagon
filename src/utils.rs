use ratatui::style::Color;

use crate::unit::cell::Cell;

pub(crate) mod delta;
pub(crate) mod depth;
pub(crate) mod direction;
pub(crate) mod entry;
pub(crate) mod file;
pub(crate) mod fill_mode;
mod mark;
pub(crate) mod mode;
mod move_state;
pub(crate) mod moves;
pub(crate) mod range;
pub(crate) mod rank;
pub(crate) mod stack;

pub(crate) trait Step: Sized {
    fn succ(&self) -> Option<Self>;
    fn pred(&self) -> Option<Self>;
}
