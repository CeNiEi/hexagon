use std::{ops::Sub, process::Output};

use super::Step;

pub(crate) struct RangeInc<T> {
    lo: T,
    hi: T,
    ended: bool,
}

impl<T> RangeInc<T> {
    pub(crate) fn new(lo: T, hi: T) -> Self {
        Self {
            lo,
            hi,
            ended: false,
        }
    }

    pub(crate) fn lo(&self) -> T
    where
        T: Copy,
    {
        self.lo
    }

    pub(crate) fn hi(&self) -> T
    where
        T: Copy,
    {
        self.hi
    }

    pub(crate) fn clamp(&self, rhs: RangeInc<T>) -> RangeInc<T>
    where
        T: Copy + Ord,
    {
        RangeInc {
            ended: self.ended,
            lo: self.lo.max(rhs.lo),
            hi: self.hi.min(rhs.hi),
        }
    }

    pub(crate) fn remaning(&self) -> isize
    where
        T: Copy + Sub<Output = isize>,
    {
        self.hi - self.lo + 1
    }

    pub(crate) fn contains(&self, val: T) -> bool
    where
        T: PartialOrd,
    {
        self.lo <= val && val <= self.hi
    }
}

impl<T: Step + PartialEq + PartialOrd + Copy> Iterator for RangeInc<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ended {
            None
        } else if self.lo == self.hi {
            self.ended = true;
            Some(self.lo)
        } else if self.lo > self.hi {
            self.ended = true;
            None
        } else {
            match self.lo.succ() {
                None => {
                    self.ended = true;
                    None
                }
                Some(succ) => {
                    let curr = self.lo;
                    self.lo = succ;
                    Some(curr)
                }
            }
        }
    }
}

pub(crate) struct Range<T> {
    lo: T,
    hi: T,
    ended: bool,
}

impl<T> Range<T> {
    pub(crate) fn new(lo: T, hi: T) -> Self {
        Self {
            lo,
            hi,
            ended: false,
        }
    }

    pub(crate) fn remaning(&self) -> isize
    where
        T: Copy + Sub<Output = isize>,
    {
        self.hi - self.lo
    }
}

impl<T: Step + PartialEq + PartialOrd + Copy> Iterator for Range<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ended {
            None
        } else if self.lo >= self.hi {
            self.ended = true;
            None
        } else {
            match self.lo.succ() {
                None => {
                    self.ended = true;
                    None
                }
                Some(succ) => {
                    let curr = self.lo;
                    self.lo = succ;
                    Some(curr)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::range::Range;

    use super::{RangeInc, Step};

    impl Step for isize {
        fn succ(&self) -> Option<Self> {
            Some(self + 1)
        }
        fn pred(&self) -> Option<Self> {
            Some(self - 1)
        }
    }

    #[test]
    fn test_inc() {
        assert_eq!(RangeInc::new(0_isize, 1).collect::<Vec<_>>(), vec![0, 1]);
        assert_eq!(RangeInc::new(0_isize, 0).collect::<Vec<_>>(), vec![0]);
        assert_eq!(RangeInc::new(0_isize, -1).collect::<Vec<_>>(), vec![]);
    }

    #[test]
    fn test_non_inc() {
        assert_eq!(Range::new(0_isize, 2).collect::<Vec<_>>(), vec![0, 1]);
        assert_eq!(Range::new(0_isize, 1).collect::<Vec<_>>(), vec![0]);
        assert_eq!(Range::new(0_isize, 0).collect::<Vec<_>>(), vec![]);
        assert_eq!(Range::new(0_isize, -1).collect::<Vec<_>>(), vec![]);
    }
}
