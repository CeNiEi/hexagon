#[derive(Clone, Copy, Debug)]
pub(crate) struct Stack<T, const N: usize> {
    inner: [T; N],
    ptr: Option<usize>,
}

macro_rules! default_impl {
    ($cap: literal $(,)?) => {
        impl<T: Default + Copy> Default for Stack<T, $cap> {
            fn default() -> Self {
                Self {
                    inner: [T::default(); $cap],
                    ptr: None
                }
            }
        }
    };

    ($($cap: literal),* $(,)?) => {
        $( default_impl!($cap); )*
    }
}

default_impl!(1, 2, 3);

impl<T: Copy, const N: usize> Stack<T, N> {
    fn pos_to_add(&self) -> usize {
        match self.ptr {
            None => 0,
            Some(t) => t + 1,
        }
    }

    pub(crate) fn incr(&mut self) {
        self.ptr = Some(self.pos_to_add());
    }

    pub(crate) fn decr(&mut self) {
        self.ptr = match self.ptr {
            Some(0) => None,
            Some(t) => Some(t - 1),
            _ => unreachable!(),
        }
    }

    pub(crate) fn push(&mut self, val: T) {
        let pos = self.pos_to_add();

        if pos >= N {
            panic!("Reached the end of stack capacity")
        }

        self.inner[pos] = val;
        self.incr();
    }

    pub(crate) fn pop(&mut self) -> Option<T> {
        let Some(pos) = self.ptr else { return None };

        let ret = self.inner[pos];
        self.decr();

        Some(ret)
    }

    pub(crate) fn current(&self) -> Option<T> {
        self.ptr.map(|pos| self.inner[pos])
    }
}
