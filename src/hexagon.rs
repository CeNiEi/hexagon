use ratatui::{
    style::Color,
    widgets::canvas::{Line, Shape},
};

use crate::{
    unit::{UnitHexagon, cell::Cell},
    utils::{delta::Delta, file::File, mode::Mode, rank::Rank},
};

use anyhow::Result;

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Hexagon {
    unit: UnitHexagon,
    len: f64,
    mode: Mode,
}

impl Hexagon {
    pub(crate) fn try_new(rank: Rank, file: File, len: f64, mode: Mode) -> Result<Self> {
        let unit = UnitHexagon::try_new(rank, file)?;

        Ok(Self { unit, len, mode })
    }

    pub(crate) fn rank(&self) -> Rank {
        self.cell().rank
    }

    pub(crate) fn file(&self) -> File {
        self.cell().file
    }

    pub(crate) fn cell(&self) -> Cell {
        self.unit.cell()
    }

    pub(crate) fn to_board_index(&self) -> usize {
        self.cell().to_board_index()
    }

    pub(crate) fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub(crate) fn new(rank: Rank, file: File, len: f64, mode: Mode) -> Self {
        Self::try_new(rank, file, len, mode).unwrap()
    }

    pub(crate) fn center(&self) -> Delta<f64> {
        self.center_wrt(Self::default())
    }

    pub(crate) fn center_wrt(&self, other: Self) -> Delta<f64> {
        self.unit.center_wrt(other.unit) * self.len
    }

    fn point_e(&self, len: f64) -> Delta<f64> {
        self.center() + self.unit.delta_point_e() * len
    }
    fn point_se(&self, len: f64) -> Delta<f64> {
        self.center() + self.unit.delta_point_se() * len
    }
    fn point_sw(&self, len: f64) -> Delta<f64> {
        self.center() + self.unit.delta_point_sw() * len
    }
    fn point_w(&self, len: f64) -> Delta<f64> {
        self.center() + self.unit.delta_point_w() * len
    }
    fn point_nw(&self, len: f64) -> Delta<f64> {
        self.center() + self.unit.delta_point_nw() * len
    }
    fn point_ne(&self, len: f64) -> Delta<f64> {
        self.center() + self.unit.delta_point_ne() * len
    }

    fn segment_e_se(&self, len: f64) -> Line {
        let point_e = self.point_e(len);
        let point_se = self.point_se(len);

        Line {
            x1: point_e.x,
            y1: point_e.y,
            x2: point_se.x,
            y2: point_se.y,
            color: Color::Black,
        }
    }

    fn segment_se_sw(&self, len: f64) -> Line {
        let point_se = self.point_se(len);
        let point_sw = self.point_sw(len);

        Line {
            x1: point_se.x,
            y1: point_se.y,
            x2: point_sw.x,
            y2: point_sw.y,
            color: Color::Black,
        }
    }

    fn segment_sw_w(&self, len: f64) -> Line {
        let point_sw = self.point_sw(len);
        let point_w = self.point_w(len);

        Line {
            x1: point_sw.x,
            y1: point_sw.y,
            x2: point_w.x,
            y2: point_w.y,
            color: Color::Black,
        }
    }

    fn segment_w_nw(&self, len: f64) -> Line {
        let point_w = self.point_w(len);
        let point_nw = self.point_nw(len);

        Line {
            x1: point_w.x,
            y1: point_w.y,
            x2: point_nw.x,
            y2: point_nw.y,
            color: Color::Black,
        }
    }

    fn segment_nw_ne(&self, len: f64) -> Line {
        let point_nw = self.point_nw(len);
        let point_ne = self.point_ne(len);

        Line {
            x1: point_nw.x,
            y1: point_nw.y,
            x2: point_ne.x,
            y2: point_ne.y,
            color: Color::Black,
        }
    }

    fn segment_ne_e(&self, len: f64) -> Line {
        let point_ne = self.point_ne(len);
        let point_e = self.point_e(len);

        Line {
            x1: point_ne.x,
            y1: point_ne.y,
            x2: point_e.x,
            y2: point_e.y,
            color: Color::Black,
        }
    }

    pub(crate) fn draw_helper(&self, len: f64, painter: &mut ratatui::widgets::canvas::Painter) {
        let segment_e_se = self.segment_e_se(len);
        let segment_se_sw = self.segment_se_sw(len);
        let segment_sw_w = self.segment_sw_w(len);
        let segment_w_nw = self.segment_w_nw(len);
        let segment_nw_ne = self.segment_nw_ne(len);
        let segment_ne_e = self.segment_ne_e(len);

        segment_e_se.draw(painter);
        segment_se_sw.draw(painter);
        segment_sw_w.draw(painter);
        segment_w_nw.draw(painter);
        segment_nw_ne.draw(painter);
        segment_ne_e.draw(painter);
    }
}

impl Shape for Hexagon {
    fn draw(&self, painter: &mut ratatui::widgets::canvas::Painter) {
        self.draw_helper(self.len, painter);

        match self.mode {
            Mode::Current => {
                self.draw_helper(self.len / 1.5, painter);
            }
            Mode::None => {}
        }
    }
}
