use std::ops::Sub;

use super::Step;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug, PartialOrd, Ord)]
pub enum Rank {
    Rank1,
    Rank2,
    Rank3,
    Rank4,
    Rank5,
    #[default]
    Rank6,
    Rank7,
    Rank8,
    Rank9,
    Rank10,
    Rank11,
}

impl Sub for Rank {
    type Output = isize;
    fn sub(self, rhs: Self) -> Self::Output {
        self as isize - rhs as isize
    }
}

impl Step for Rank {
    fn succ(&self) -> Option<Self> {
        match self {
            Rank::Rank1 => Some(Rank::Rank2),
            Rank::Rank2 => Some(Rank::Rank3),
            Rank::Rank3 => Some(Rank::Rank4),
            Rank::Rank4 => Some(Rank::Rank5),
            Rank::Rank5 => Some(Rank::Rank6),
            Rank::Rank6 => Some(Rank::Rank7),
            Rank::Rank7 => Some(Rank::Rank8),
            Rank::Rank8 => Some(Rank::Rank9),
            Rank::Rank9 => Some(Rank::Rank10),
            Rank::Rank10 => Some(Rank::Rank11),
            Rank::Rank11 => None,
        }
    }
    fn pred(&self) -> Option<Self> {
        match self {
            Rank::Rank1 => None,
            Rank::Rank2 => Some(Rank::Rank1),
            Rank::Rank3 => Some(Rank::Rank2),
            Rank::Rank4 => Some(Rank::Rank3),
            Rank::Rank5 => Some(Rank::Rank4),
            Rank::Rank6 => Some(Rank::Rank5),
            Rank::Rank7 => Some(Rank::Rank6),
            Rank::Rank8 => Some(Rank::Rank7),
            Rank::Rank9 => Some(Rank::Rank8),
            Rank::Rank10 => Some(Rank::Rank9),
            Rank::Rank11 => Some(Rank::Rank10),
        }
    }
}
