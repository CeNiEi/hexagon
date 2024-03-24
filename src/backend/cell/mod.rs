use std::{hash::Hash, ops::Sub};

use ratatui::{
    style::Color,
    widgets::canvas::{Line, Shape},
};

use self::{file::File, rank::Rank};

use super::direction::Direction;

pub(crate) mod file;
pub(crate) mod rank;

pub trait Sucessor: Sized {
    fn next(&self) -> Option<Self>;
}

pub trait Predecessor: Sized {
    fn previous(&self) -> Option<Self>;
}

#[derive(Copy, Clone, Default, Debug)]
pub(crate) enum HighlightLevel {
    #[default]
    None,
    Current,
    Caputreable,
    Movable,
}

#[derive(Clone, Copy, Default, Debug)]
pub(crate) struct Cell {
    rank: rank::Rank,
    file: file::File,

    highlight_level: HighlightLevel,
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank && self.file == other.file
    }
}

impl Sub for Cell {
    type Output = (i8, i8);
    fn sub(self, rhs: Self) -> Self::Output {
        (self.file - rhs.file, self.rank - rhs.rank)
    }
}

impl Shape for Cell {
    fn draw(&self, painter: &mut ratatui::widgets::canvas::Painter) {
        let (x, y) = self.get_center();
        let side = 1. * Self::SIDE;

        let (sides, color) = match self.highlight_level {
            HighlightLevel::None => (vec![side].into_iter(), Color::Black),
            HighlightLevel::Current => (vec![side, 0.7 * side].into_iter(), Color::Black),
            HighlightLevel::Caputreable => (vec![side, 0.7 * side].into_iter(), Color::Red),
            HighlightLevel::Movable => (vec![side, 0.7 * side].into_iter(), Color::Green),
        };

        for side in sides {
            let point_e = (x + side, y);

            let point_se = (x + (side / 2.), y - side * (3_f64.sqrt() / 2.));

            let point_sw = (x - (side / 2.), y - side * (3_f64.sqrt() / 2.));

            let point_w = (x - side, y);

            let point_nw = (x - (side / 2.), y + side * (3_f64.sqrt() / 2.));

            let point_ne = (x + (side / 2.), y + side * (3_f64.sqrt() / 2.));

            let segment_e_se = Line {
                x1: point_e.0,
                y1: point_e.1,
                x2: point_se.0,
                y2: point_se.1,
                color,
            };

            let segment_se_sw = Line {
                x1: point_se.0,
                y1: point_se.1,
                x2: point_sw.0,
                y2: point_sw.1,
                color,
            };

            let segment_sw_w = Line {
                x1: point_sw.0,
                y1: point_sw.1,
                x2: point_w.0,
                y2: point_w.1,
                color,
            };

            let segment_w_nw = Line {
                x1: point_w.0,
                y1: point_w.1,
                x2: point_nw.0,
                y2: point_nw.1,
                color,
            };

            let segment_nw_ne = Line {
                x1: point_nw.0,
                y1: point_nw.1,
                x2: point_ne.0,
                y2: point_ne.1,
                color,
            };

            let segment_ne_e = Line {
                x1: point_ne.0,
                y1: point_ne.1,
                x2: point_e.0,
                y2: point_e.1,
                color,
            };

            segment_e_se.draw(painter);
            segment_se_sw.draw(painter);
            segment_sw_w.draw(painter);
            segment_w_nw.draw(painter);
            segment_nw_ne.draw(painter);
            segment_ne_e.draw(painter);
        }
    }
}

impl Cell {
    const SIDE: f64 = 3.;

    pub(crate) fn get_center(&self) -> (f64, f64) {
        let (x_diff, y_diff) = *self - Cell::default();

        let x = x_diff as f64 * Self::SIDE * 1.5;
        let y = y_diff as f64 * Self::SIDE * 3_f64.sqrt()
            + x_diff.abs() as f64 * Self::SIDE * (3_f64.sqrt() / 2.);

        (x, y)
    }

    pub(crate) const fn new(rank: rank::Rank, file: file::File) -> Self {
        Self {
            rank,
            file,
            highlight_level: HighlightLevel::None,
        }
    }

    pub(crate) fn try_new(rank: rank::Rank, file: file::File) -> Option<Self> {
        match rank {
            Rank::Rank7 => file < File::FileL && file > File::FileA,
            Rank::Rank8 => file < File::FileK && file > File::FileB,
            Rank::Rank9 => file < File::FileI && file > File::FileC,
            Rank::Rank10 => file < File::FileH && file > File::FileD,
            Rank::Rank11 => file < File::FileG && file > File::FileE,
            _ => true,
        }
        .then(|| Self::new(rank, file))
    }

    pub(crate) fn set_highlight_level(&mut self, highlight_level: HighlightLevel) {
        self.highlight_level = highlight_level
    }

    pub(crate) fn next_cell(&self, direction: Direction) -> Option<Cell> {
        match direction {
            Direction::Clock1 => {
                let rank = self.rank.next();
                let file = self.file.next();

                match (rank, file) {
                    (Some(r), Some(f)) => Self::try_new(r, f),
                    _ => None,
                }
            }
            Direction::Clock2 => {
                let file = self.file.next();

                file.and_then(|f| Self::try_new(self.rank, f))
            }
            Direction::Clock3 => {
                let rank = self.rank.previous();
                let file = self.file.next().and_then(|f| f.next());

                match (rank, file) {
                    (Some(r), Some(f)) => Self::try_new(r, f),
                    _ => None,
                }
            }
            Direction::Clock4 => {
                let rank = self.rank.previous();
                let file = self.file.next();

                match (rank, file) {
                    (Some(r), Some(f)) => Self::try_new(r, f),
                    _ => None,
                }
            }
            Direction::Clock5 => {
                let rank = self.rank.previous().and_then(|r| r.previous());
                let file = self.file.next();

                match (rank, file) {
                    (Some(r), Some(f)) => Self::try_new(r, f),
                    _ => None,
                }
            }
            Direction::Clock6 => {
                let rank = self.rank.previous();

                rank.and_then(|r| Self::try_new(r, self.file))
            }
            Direction::Clock7 => {
                let rank = self.rank.previous().and_then(|r| r.previous());
                let file = self.file.previous();

                match (rank, file) {
                    (Some(r), Some(f)) => Self::try_new(r, f),
                    _ => None,
                }
            }
            Direction::Clock8 => {
                let rank = self.rank.previous();
                let file = self.file.previous();

                match (rank, file) {
                    (Some(r), Some(f)) => Self::try_new(r, f),
                    _ => None,
                }
            }
            Direction::Clock9 => {
                let rank = self.rank.previous();
                let file = self.file.previous().and_then(|f| f.previous());

                match (rank, file) {
                    (Some(r), Some(f)) => Self::try_new(r, f),
                    _ => None,
                }
            }
            Direction::Clock10 => {
                let file = self.file.previous();

                file.and_then(|f| Self::try_new(self.rank, f))
            }
            Direction::Clock11 => {
                let rank = self.rank.next();
                let file = self.file.previous();

                match (rank, file) {
                    (Some(r), Some(f)) => Self::try_new(r, f),
                    _ => None,
                }
            }
            Direction::Clock12 => {
                let rank = self.rank.next();

                rank.and_then(|r| Cell::try_new(r, self.file))
            }
        }
    }
}
