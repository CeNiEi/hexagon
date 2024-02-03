pub(crate) mod file;
pub(crate) mod rank;

use crate::board::utils::direction::Direction;

pub trait Sucessor: Sized {
    fn next(&self) -> Option<Self>;
}

pub trait Predecessor: Sized {
    fn previous(&self) -> Option<Self>;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Cell {
    rank: rank::Rank,
    file: file::File,
}

impl Cell {
    pub(crate) fn new(rank: rank::Rank, file: file::File) -> Self {
        Self { rank, file }
    }

    pub(crate) fn next_cell(&self, direction: Direction) -> Option<Cell> {
        match direction {
            Direction::Clock1 => {
                let rank = self.rank.next();
                let file = self.file.next();

                match (rank, file) {
                    (Some(r), Some(f)) => Some(Self::new(r, f)),
                    _ => None,
                }
            }
            Direction::Clock2 => {
                let file = self.file.next();

                file.and_then(|f| Some(Cell::new(self.rank, f)))
            }
            Direction::Clock3 => {
                let rank = self.rank.previous();
                let file = self.file.next().and_then(|f| f.next());

                match (rank, file) {
                    (Some(r), Some(f)) => Some(Self::new(r, f)),
                    _ => None,
                }
            }
            Direction::Clock4 => {
                let rank = self.rank.previous();
                let file = self.file.next();

                match (rank, file) {
                    (Some(r), Some(f)) => Some(Self::new(r, f)),
                    _ => None,
                }
            }
            Direction::Clock5 => {
                let rank = self.rank.previous().and_then(|r| r.previous());
                let file = self.file.next();

                match (rank, file) {
                    (Some(r), Some(f)) => Some(Self::new(r, f)),
                    _ => None,
                }
            }
            Direction::Clock6 => {
                let rank = self.rank.previous();

                rank.and_then(|r| Some(Cell::new(r, self.file)))
            }
            Direction::Clock7 => {
                let rank = self.rank.previous().and_then(|r| r.previous());
                let file = self.file.previous();

                match (rank, file) {
                    (Some(r), Some(f)) => Some(Self::new(r, f)),
                    _ => None,
                }
            }
            Direction::Clock8 => {
                let rank = self.rank.previous();
                let file = self.file.previous();

                match (rank, file) {
                    (Some(r), Some(f)) => Some(Self::new(r, f)),
                    _ => None,
                }
            }
            Direction::Clock9 => {
                let rank = self.rank.previous();
                let file = self.file.previous().and_then(|f| f.previous());

                match (rank, file) {
                    (Some(r), Some(f)) => Some(Self::new(r, f)),
                    _ => None,
                }
            }
            Direction::Clock10 => {
                let file = self.file.previous();

                file.and_then(|f| Some(Cell::new(self.rank, f)))
            }
            Direction::Clock11 => {
                let rank = self.rank.next();
                let file = self.file.previous();

                match (rank, file) {
                    (Some(r), Some(f)) => Some(Self::new(r, f)),
                    _ => None,
                }
            }

            Direction::Clock12 => {
                let rank = self.rank.next();

                rank.and_then(|r| Some(Cell::new(r, self.file)))
            }
        }
    }
}
