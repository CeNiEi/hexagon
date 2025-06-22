use std::ops::Sub;

use cell::Cell;
use repr::Repr;

use crate::utils::{delta::Delta, file::File, rank::Rank};
use anyhow::Result;

pub(crate) mod cell;
pub(crate) mod repr;

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct UnitHexagon {
    cell: Cell,
}

impl Sub for UnitHexagon {
    type Output = Delta<isize>;
    fn sub(self, rhs: Self) -> Self::Output {
        Repr::from(self.cell) - Repr::from(rhs.cell)
    }
}

impl UnitHexagon {
    pub(crate) fn try_new(rank: Rank, file: File) -> Result<Self> {
        Ok(Self {
            cell: Cell::try_new(rank, file)?,
        })
    }

    pub(crate) fn cell(&self) -> Cell {
        self.cell
    }

    pub(crate) fn new(rank: Rank, file: File) -> Self {
        Self::try_new(rank, file).unwrap()
    }

    pub(crate) fn center(&self) -> Delta<f64> {
        self.center_wrt(Self::default())
    }

    pub(crate) fn center_wrt(&self, other: Self) -> Delta<f64> {
        let delta = Repr::from(self.cell) - Repr::from(other.cell);

        delta.cast() * (1.5, 3_f64.sqrt() / 2.)
    }

    pub(crate) fn delta_point_e(&self) -> Delta<f64> {
        (1., 0.).into()
    }
    pub(crate) fn delta_point_se(&self) -> Delta<f64> {
        (0.5, -3_f64.sqrt() / 2.).into()
    }
    pub(crate) fn delta_point_sw(&self) -> Delta<f64> {
        (-0.5, -3_f64.sqrt() / 2.).into()
    }
    pub(crate) fn delta_point_w(&self) -> Delta<f64> {
        (-1., 0.).into()
    }
    pub(crate) fn delta_point_nw(&self) -> Delta<f64> {
        (-0.5, 3_f64.sqrt() / 2.).into()
    }
    pub(crate) fn delta_point_ne(&self) -> Delta<f64> {
        (0.5, 3_f64.sqrt() / 2.).into()
    }
}
