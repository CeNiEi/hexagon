use anyhow::Result;

use super::{file::File, range::RangeInc, rank::Rank};

pub(crate) struct Depth(u8);

impl Depth {
    pub(crate) fn new(raw: u8) -> Result<Self> {
        if (1..=6).contains(&raw) {
            Ok(Self(raw))
        } else {
            anyhow::bail!("depth must be between 1 and 6; found: {}", raw)
        }
    }

    pub(crate) fn file_range(&self) -> RangeInc<File> {
        match self.0 {
            1 => RangeInc::new(File::FileF, File::FileF),
            2 => RangeInc::new(File::FileE, File::FileG),
            3 => RangeInc::new(File::FileD, File::FileH),
            4 => RangeInc::new(File::FileC, File::FileI),
            5 => RangeInc::new(File::FileB, File::FileK),
            6 => RangeInc::new(File::FileA, File::FileL),
            _ => unreachable!(),
        }
    }

    pub(crate) fn rank_range(&self, file: File) -> RangeInc<Rank> {
        match self.0 {
            1 => match file {
                File::FileF => RangeInc::new(Rank::Rank6, Rank::Rank6),
                _ => unreachable!(),
            },

            2 => match file {
                File::FileE => RangeInc::new(Rank::Rank5, Rank::Rank6),
                File::FileF => RangeInc::new(Rank::Rank5, Rank::Rank7),
                File::FileG => RangeInc::new(Rank::Rank5, Rank::Rank6),
                _ => unreachable!(),
            },

            3 => match file {
                File::FileD => RangeInc::new(Rank::Rank4, Rank::Rank6),
                File::FileE => RangeInc::new(Rank::Rank4, Rank::Rank7),
                File::FileF => RangeInc::new(Rank::Rank4, Rank::Rank8),
                File::FileG => RangeInc::new(Rank::Rank4, Rank::Rank7),
                File::FileH => RangeInc::new(Rank::Rank4, Rank::Rank6),
                _ => unreachable!(),
            },

            4 => match file {
                File::FileC => RangeInc::new(Rank::Rank3, Rank::Rank6),
                File::FileD => RangeInc::new(Rank::Rank3, Rank::Rank7),
                File::FileE => RangeInc::new(Rank::Rank3, Rank::Rank8),
                File::FileF => RangeInc::new(Rank::Rank3, Rank::Rank9),
                File::FileG => RangeInc::new(Rank::Rank3, Rank::Rank8),
                File::FileH => RangeInc::new(Rank::Rank3, Rank::Rank7),
                File::FileI => RangeInc::new(Rank::Rank3, Rank::Rank6),
                _ => unreachable!(),
            },

            5 => match file {
                File::FileB => RangeInc::new(Rank::Rank2, Rank::Rank6),
                File::FileC => RangeInc::new(Rank::Rank2, Rank::Rank7),
                File::FileD => RangeInc::new(Rank::Rank2, Rank::Rank8),
                File::FileE => RangeInc::new(Rank::Rank2, Rank::Rank9),
                File::FileF => RangeInc::new(Rank::Rank2, Rank::Rank10),
                File::FileG => RangeInc::new(Rank::Rank2, Rank::Rank9),
                File::FileH => RangeInc::new(Rank::Rank2, Rank::Rank8),
                File::FileI => RangeInc::new(Rank::Rank2, Rank::Rank7),
                File::FileK => RangeInc::new(Rank::Rank2, Rank::Rank6),
                _ => unreachable!(),
            },

            6 => match file {
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
            },

            _ => unreachable!(),
        }
    }

    pub(crate) fn first_file(&self) -> File {
        match self.0 {
            1 => File::FileF,
            2 => File::FileE,
            3 => File::FileD,
            4 => File::FileC,
            5 => File::FileB,
            6 => File::FileA,
            _ => unreachable!(),
        }
    }

    pub(crate) fn first_rank(&self) -> Rank {
        match self.0 {
            1 => Rank::Rank6,
            2 => Rank::Rank5,
            3 => Rank::Rank4,
            4 => Rank::Rank3,
            5 => Rank::Rank2,
            6 => Rank::Rank1,
            _ => unreachable!(),
        }
    }
}
