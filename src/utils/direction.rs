#[derive(Clone, Copy, Debug)]
pub(crate) enum Direction {
    Clock1,
    Clock2,
    Clock3,
    Clock4,
    Clock5,
    Clock6,
    Clock7,
    Clock8,
    Clock9,
    Clock10,
    Clock11,
    Clock12,
}

impl Direction {
    pub(crate) fn reverse(&self) -> Direction {
        match self {
            Direction::Clock1 => Direction::Clock7,
            Direction::Clock3 => Direction::Clock9,
            Direction::Clock5 => Direction::Clock11,
            Direction::Clock7 => Direction::Clock1,
            Direction::Clock9 => Direction::Clock3,
            Direction::Clock11 => Direction::Clock5,

            Direction::Clock2 => Direction::Clock8,
            Direction::Clock4 => Direction::Clock10,
            Direction::Clock6 => Direction::Clock12,
            Direction::Clock8 => Direction::Clock2,
            Direction::Clock10 => Direction::Clock4,
            Direction::Clock12 => Direction::Clock6,
        }
    }

    pub(crate) fn turn_clockwise(&self) -> Direction {
        match self {
            Direction::Clock1 => Direction::Clock3,
            Direction::Clock3 => Direction::Clock5,
            Direction::Clock5 => Direction::Clock7,
            Direction::Clock7 => Direction::Clock9,
            Direction::Clock9 => Direction::Clock11,
            Direction::Clock11 => Direction::Clock1,

            Direction::Clock2 => Direction::Clock4,
            Direction::Clock4 => Direction::Clock6,
            Direction::Clock6 => Direction::Clock8,
            Direction::Clock8 => Direction::Clock10,
            Direction::Clock10 => Direction::Clock12,
            Direction::Clock12 => Direction::Clock2,
        }
    }

    pub(crate) fn turn_counter_clockwise(&self) -> Direction {
        match self {
            Direction::Clock1 => Direction::Clock11,
            Direction::Clock3 => Direction::Clock1,
            Direction::Clock5 => Direction::Clock3,
            Direction::Clock7 => Direction::Clock5,
            Direction::Clock9 => Direction::Clock7,
            Direction::Clock11 => Direction::Clock9,

            Direction::Clock2 => Direction::Clock12,
            Direction::Clock4 => Direction::Clock2,
            Direction::Clock6 => Direction::Clock4,
            Direction::Clock8 => Direction::Clock6,
            Direction::Clock10 => Direction::Clock8,
            Direction::Clock12 => Direction::Clock10,
        }
    }
}
