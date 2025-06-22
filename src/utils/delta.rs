use std::{
    ops::{Add, Mul},
    process::Output,
};

pub(crate) struct Delta<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

impl<T> From<(T, T)> for Delta<T> {
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}

impl<T: Copy> Delta<T> {
    pub(crate) fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub(crate) fn splat(val: T) -> Self {
        Self { x: val, y: val }
    }
}

impl<T> Delta<T> {
    pub(crate) fn cast<F: Copy + 'static>(&self) -> Delta<F>
    where
        T: num::cast::AsPrimitive<F>,
    {
        Delta {
            x: self.x.as_(),
            y: self.y.as_(),
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Delta<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Delta {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul<(T, T)> for Delta<T> {
    type Output = Self;
    fn mul(self, (x, y): (T, T)) -> Self::Output {
        Delta {
            x: self.x * x,
            y: self.y * y,
        }
    }
}

impl<T: Add<Output = T> + Copy> Add<Delta<T>> for Delta<T> {
    type Output = Self;
    fn add(self, rhs: Delta<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
