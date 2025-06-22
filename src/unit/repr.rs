use std::ops::Sub;

use anyhow::{Result, anyhow};

use crate::utils::{delta::Delta, direction::Direction, file::File, rank::Rank};

use super::Cell;

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Repr {
    x: isize,
    y: isize,
}

impl Sub for Repr {
    type Output = Delta<isize>;
    fn sub(self, rhs: Self) -> Self::Output {
        (self.x - rhs.x, self.y - rhs.y).into()
    }
}

impl From<Cell> for Repr {
    fn from(value: Cell) -> Self {
        let x = value.file - File::default();
        let y = x.abs() + (value.rank - Rank::default()) * 2;

        Self::new(x, y)
    }
}

impl Repr {
    pub(crate) fn try_new(x: isize, y: isize) -> Result<Self> {
        if x.abs() % 2 != y.abs() % 2 {
            Err(anyhow!(
                "x and y must be of same parity, found x: {x}, y: {y}"
            ))
        } else {
            Ok(Self { x, y })
        }
    }

    pub(crate) fn new(x: isize, y: isize) -> Self {
        Self::try_new(x, y).unwrap()
    }

    pub(crate) fn next(&self, direction: Direction) -> Self {
        let (x, y) = match direction {
            Direction::Clock1 => (self.x + 1, self.y + 3),
            Direction::Clock2 => (self.x + 1, self.y + 1),
            Direction::Clock3 => (self.x + 2, self.y),
            Direction::Clock4 => (self.x + 1, self.y - 1),
            Direction::Clock5 => (self.x + 1, self.y - 3),
            Direction::Clock6 => (self.x, self.y - 2),
            Direction::Clock7 => (self.x - 1, self.y - 3),
            Direction::Clock8 => (self.x - 1, self.y - 1),
            Direction::Clock9 => (self.x - 2, self.y),
            Direction::Clock10 => (self.x - 1, self.y + 1),
            Direction::Clock11 => (self.x - 1, self.y + 3),
            Direction::Clock12 => (self.x, self.y + 2),
        };

        Self::new(x, y)
    }
}
