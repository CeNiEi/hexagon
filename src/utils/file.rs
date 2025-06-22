use std::ops::Sub;

use strum::EnumIter;

use super::{Step, range::RangeInc, rank::Rank};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug, PartialOrd, EnumIter)]
pub enum File {
    FileA,
    FileB,
    FileC,
    FileD,
    FileE,
    #[default]
    FileF,
    FileG,
    FileH,
    FileI,
    FileK,
    FileL,
}

impl Sub for File {
    type Output = isize;
    fn sub(self, rhs: Self) -> Self::Output {
        self as isize - rhs as isize
    }
}

impl Step for File {
    fn succ(&self) -> Option<Self> {
        match self {
            File::FileA => Some(File::FileB),
            File::FileB => Some(File::FileC),
            File::FileC => Some(File::FileD),
            File::FileD => Some(File::FileE),
            File::FileE => Some(File::FileF),
            File::FileF => Some(File::FileG),
            File::FileG => Some(File::FileH),
            File::FileH => Some(File::FileI),
            File::FileI => Some(File::FileK),
            File::FileK => Some(File::FileL),
            File::FileL => None,
        }
    }

    fn pred(&self) -> Option<Self> {
        match self {
            File::FileA => None,
            File::FileB => Some(File::FileA),
            File::FileC => Some(File::FileB),
            File::FileD => Some(File::FileC),
            File::FileE => Some(File::FileD),
            File::FileF => Some(File::FileE),
            File::FileG => Some(File::FileF),
            File::FileH => Some(File::FileG),
            File::FileI => Some(File::FileH),
            File::FileK => Some(File::FileI),
            File::FileL => Some(File::FileK),
        }
    }
}

impl File {
    pub(crate) fn rank_range(&self) -> RangeInc<Rank> {
        match self {
            File::FileA => RangeInc::new(Rank::Rank1, Rank::Rank6),
            File::FileB => RangeInc::new(Rank::Rank1, Rank::Rank7),
            File::FileC => RangeInc::new(Rank::Rank1, Rank::Rank8),
            File::FileD => RangeInc::new(Rank::Rank1, Rank::Rank9),
            File::FileE => RangeInc::new(Rank::Rank1, Rank::Rank10),
            File::FileF => RangeInc::new(Rank::Rank1, Rank::Rank11),
            File::FileG => RangeInc::new(Rank::Rank1, Rank::Rank10),
            File::FileH => RangeInc::new(Rank::Rank1, Rank::Rank9),
            File::FileI => RangeInc::new(Rank::Rank1, Rank::Rank8),
            File::FileK => RangeInc::new(Rank::Rank1, Rank::Rank7),
            File::FileL => RangeInc::new(Rank::Rank1, Rank::Rank6),
        }
    }
}
