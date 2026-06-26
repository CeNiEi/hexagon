use ratatui::style::Color;

use crate::unit::cell::Cell;

pub(crate) mod consts;
pub(crate) mod delta;
pub(crate) mod depth;
pub(crate) mod direction;
pub(crate) mod entry;
pub(crate) mod file;
pub(crate) mod fill_mode;
pub(crate) mod history;
pub(crate) mod mark;
pub(crate) mod mode;
pub(crate) mod moves;
pub(crate) mod player;
pub(crate) mod progression;
pub(crate) mod range;
pub(crate) mod rank;
pub(crate) mod stack;

pub(crate) trait Step: Sized {
    fn succ(&self) -> Option<Self>;
    fn pred(&self) -> Option<Self>;
}
