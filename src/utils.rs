pub(crate) mod delta;
pub(crate) mod direction;
pub(crate) mod entry;
pub(crate) mod file;
pub(crate) mod mode;
pub(crate) mod range;
pub(crate) mod rank;

pub(crate) trait Step: Sized {
    fn succ(&self) -> Option<Self>;
    fn pred(&self) -> Option<Self>;
}
