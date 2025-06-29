use crate::utils::{Step, direction::Direction, file::File, range::Range, rank::Rank};
use anyhow::{Result, anyhow};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct Cell {
    pub(crate) rank: Rank,
    pub(crate) file: File,
}

impl Cell {
    pub(crate) fn try_new(rank: Rank, file: File) -> Result<Self> {
        if match rank {
            Rank::Rank7 => file < File::FileL && file > File::FileA,
            Rank::Rank8 => file < File::FileK && file > File::FileB,
            Rank::Rank9 => file < File::FileI && file > File::FileC,
            Rank::Rank10 => file < File::FileH && file > File::FileD,
            Rank::Rank11 => file < File::FileG && file > File::FileE,
            _ => true,
        } {
            Ok(Self { rank, file })
        } else {
            Err(anyhow!(
                "Invalid rank and file combination, found rank: {:?}, file: {:?}",
                rank,
                file
            ))
        }
    }

    pub(crate) const unsafe fn from_raw_parts(rank: Rank, file: File) -> Self {
        Self { rank, file }
    }

    pub(crate) fn new(rank: Rank, file: File) -> Self {
        Self::try_new(rank, file).unwrap()
    }

    pub(crate) fn next(&self, direction: Direction) -> Option<Self> {
        let (rank, file) = match direction {
            Direction::Clock1 => {
                let rank = if self.file < File::FileF {
                    self.rank.succ().and_then(|r| r.succ())
                } else {
                    self.rank.succ()
                };

                let file = self.file.succ();

                (rank, file)
            }
            Direction::Clock2 => {
                let rank = if self.file < File::FileF {
                    self.rank.succ()
                } else {
                    Some(self.rank)
                };

                let file = self.file.succ();

                (rank, file)
            }
            Direction::Clock3 => {
                let rank = if self.file == File::FileE {
                    Some(self.rank)
                } else if self.file < File::FileF {
                    self.rank.succ()
                } else {
                    self.rank.pred()
                };

                let file = self.file.succ().and_then(|f| f.succ());

                (rank, file)
            }
            Direction::Clock4 => {
                let rank = if self.file < File::FileF {
                    Some(self.rank)
                } else {
                    self.rank.pred()
                };

                let file = self.file.succ();

                (rank, file)
            }
            Direction::Clock5 => {
                let rank = if self.file < File::FileF {
                    self.rank.pred()
                } else {
                    self.rank.pred().and_then(|r| r.pred())
                };

                let file = self.file.succ();
                (rank, file)
            }
            Direction::Clock6 => {
                let rank = self.rank.pred();
                let file = Some(self.file);

                (rank, file)
            }
            Direction::Clock7 => {
                let rank = if self.file > File::FileF {
                    self.rank.pred()
                } else {
                    self.rank.pred().and_then(|r| r.pred())
                };

                let file = self.file.pred();

                (rank, file)
            }
            Direction::Clock8 => {
                let rank = if self.file > File::FileF {
                    Some(self.rank)
                } else {
                    self.rank.pred()
                };

                let file = self.file.pred();

                (rank, file)
            }
            Direction::Clock9 => {
                let rank = if self.file == File::FileG {
                    Some(self.rank)
                } else if self.file > File::FileF {
                    self.rank.succ()
                } else {
                    self.rank.pred()
                };
                let file = self.file.pred().and_then(|f| f.pred());

                (rank, file)
            }
            Direction::Clock10 => {
                let rank = if self.file > File::FileF {
                    self.rank.succ()
                } else {
                    Some(self.rank)
                };

                let file = self.file.pred();
                (rank, file)
            }
            Direction::Clock11 => {
                let rank = if self.file > File::FileF {
                    self.rank.succ().and_then(|r| r.succ())
                } else {
                    self.rank.succ()
                };

                let file = self.file.pred();

                (rank, file)
            }
            Direction::Clock12 => {
                let rank = self.rank.succ();
                let file = Some(self.file);

                (rank, file)
            }
        };

        match (rank, file) {
            (Some(r), Some(f)) => Self::try_new(r, f).ok(),
            _ => None,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use strum::IntoEnumIterator;
//
//     use crate::utils::file::File;
//
//     use super::Cell;
//
//     #[test]
//     fn test_board_idx() {
//         assert!(
//             File::iter()
//                 .flat_map(|file| file.rank_range().map(move |rank| Cell::new(rank, file)))
//                 .enumerate()
//                 .all(|(idx, cell)| idx == cell.to_board_index())
//         )
//     }
// }
