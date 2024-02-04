use super::{Predecessor, Sucessor};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum File {
    FileA,
    FileB,
    FileC,
    FileD,
    FileE,
    FileF,
    FileG,
    FileH,
    FileI,
    FileK,
    FileL,
}

impl Predecessor for File {
    fn previous(&self) -> Option<Self> {
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
}

impl Sucessor for File {
    fn next(&self) -> Option<Self> {
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
