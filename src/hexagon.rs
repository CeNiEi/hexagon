use ratatui::{
    style::Color,
    widgets::canvas::{Circle, Line, Painter, Shape},
};

use crate::{
    unit::{UnitHexagon, cell::Cell},
    utils::{
        delta::Delta,
        file::File,
        fill_mode::FillMode,
        mode::{HighlightMode, Status},
        rank::Rank,
        stack::Stack,
    },
};

use anyhow::Result;

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Hexagon {
    unit: UnitHexagon,
    len: f64,
    mode: HighlightMode,
    color: Color,
    fill_mode: FillMode,
    padding: f64,
}

impl Hexagon {
    pub(crate) fn try_new(
        rank: Rank,
        file: File,
        len: f64,
        padding: f64,
        color: Color,
        color_mode: FillMode,
        highlight_mode: HighlightMode,
    ) -> Result<Self> {
        let unit = UnitHexagon::try_new(rank, file)?;

        Ok(Self {
            unit,
            len,
            color,
            mode: highlight_mode,
            fill_mode: color_mode,
            padding,
        })
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

    pub(crate) fn len(&self) -> f64 {
        self.len
    }

    pub(crate) fn set_current(&mut self, current: bool) {
        self.mode.set_current(current);
    }

    pub(crate) fn set_status(&mut self, status: Status) {
        self.mode.set_status(status);
    }

    pub(crate) fn new(
        rank: Rank,
        file: File,
        len: f64,
        padding: f64,
        color: Color,
        color_mode: FillMode,
        highlight_mode: HighlightMode,
    ) -> Self {
        Self::try_new(rank, file, len, padding, color, color_mode, highlight_mode).unwrap()
    }

    pub(crate) fn center(&self) -> Delta<f64> {
        self.center_wrt(Self::default())
    }

    pub(crate) fn center_wrt(&self, other: Self) -> Delta<f64> {
        self.unit.center_wrt(other.unit) * self.len
    }

    fn point_e(&self) -> Delta<f64> {
        self.center() + self.unit.delta_point_e() * self.len * self.padding
    }
    fn point_se(&self) -> Delta<f64> {
        self.center() + self.unit.delta_point_se() * self.len * self.padding
    }
    fn point_sw(&self) -> Delta<f64> {
        self.center() + self.unit.delta_point_sw() * self.len * self.padding
    }
    fn point_w(&self) -> Delta<f64> {
        self.center() + self.unit.delta_point_w() * self.len * self.padding
    }
    fn point_nw(&self) -> Delta<f64> {
        self.center() + self.unit.delta_point_nw() * self.len * self.padding
    }
    fn point_ne(&self) -> Delta<f64> {
        self.center() + self.unit.delta_point_ne() * self.len * self.padding
    }

    fn segment_e_se(&self) -> Line {
        let point_e = self.point_e();
        let point_se = self.point_se();

        Line {
            x1: point_e.x,
            y1: point_e.y,
            x2: point_se.x,
            y2: point_se.y,
            color: self.color,
        }
    }

    fn segment_se_sw(&self) -> Line {
        let point_se = self.point_se();
        let point_sw = self.point_sw();

        Line {
            x1: point_se.x,
            y1: point_se.y,
            x2: point_sw.x,
            y2: point_sw.y,
            color: self.color,
        }
    }

    fn segment_sw_w(&self) -> Line {
        let point_sw = self.point_sw();
        let point_w = self.point_w();

        Line {
            x1: point_sw.x,
            y1: point_sw.y,
            x2: point_w.x,
            y2: point_w.y,
            color: self.color,
        }
    }

    fn segment_w_nw(&self) -> Line {
        let point_w = self.point_w();
        let point_nw = self.point_nw();

        Line {
            x1: point_w.x,
            y1: point_w.y,
            x2: point_nw.x,
            y2: point_nw.y,
            color: self.color,
        }
    }

    fn segment_nw_ne(&self) -> Line {
        let point_nw = self.point_nw();
        let point_ne = self.point_ne();

        Line {
            x1: point_nw.x,
            y1: point_nw.y,
            x2: point_ne.x,
            y2: point_ne.y,
            color: self.color,
        }
    }

    fn segment_ne_e(&self) -> Line {
        let point_ne = self.point_ne();
        let point_e = self.point_e();

        Line {
            x1: point_ne.x,
            y1: point_ne.y,
            x2: point_e.x,
            y2: point_e.y,
            color: self.color,
        }
    }

    fn draw_boundaries(&self, painter: &mut Painter) {
        let segment_e_se = self.segment_e_se();
        let segment_se_sw = self.segment_se_sw();
        let segment_sw_w = self.segment_sw_w();
        let segment_w_nw = self.segment_w_nw();
        let segment_nw_ne = self.segment_nw_ne();
        let segment_ne_e = self.segment_ne_e();

        segment_e_se.draw(painter);
        segment_se_sw.draw(painter);
        segment_sw_w.draw(painter);
        segment_w_nw.draw(painter);
        segment_nw_ne.draw(painter);
        segment_ne_e.draw(painter);
    }

    fn contains_raycasting(&self, point: Delta<f64>) -> bool {
        let points = [
            self.point_e(),
            self.point_se(),
            self.point_sw(),
            self.point_w(),
            self.point_nw(),
            self.point_ne(),
        ];

        points
            .iter()
            .zip(points.iter().cycle().skip(1))
            .fold(false, |accum, (pi, pj)| {
                let intersects = (pi.y > point.y) != (pj.y > point.y)
                    && point.x
                        < (pj.x - pi.x) * (point.y - pi.y) / (pj.y - pi.y + f64::EPSILON) + pi.x;

                accum ^ intersects
            })
    }

    fn contains_signed_area(&self, point: Delta<f64>) -> bool {
        let points = [
            self.point_e(),
            self.point_se(),
            self.point_sw(),
            self.point_w(),
            self.point_nw(),
            self.point_ne(),
        ];

        points
            .iter()
            .zip(points.iter().cycle().skip(1))
            .map(|(a, b)| (b.x - a.x) * (point.y - a.y) - (b.y - a.y) * (point.x - a.x))
            .all(|cross| cross <= 0.0)
    }

    fn draw_filled(&self, painter: &mut Painter) {
        let (min_x, max_x) = (self.point_w().x, self.point_e().x);
        let (min_y, max_y) = (self.point_se().y, self.point_ne().y);

        const STEP: f64 = 0.1;

        let mut y = min_y;
        while y <= max_y {
            let mut x = min_x;
            while x <= max_x {
                let point = Delta::new(x, y);
                if self.contains_raycasting(point) {
                    let Some((x1, y1)) = painter.get_point(x, y) else {
                        return;
                    };

                    painter.paint(x1, y1, self.color);
                }
                x += STEP;
            }
            y += STEP;
        }
    }

    fn draw_filled_alt(&self, painter: &mut Painter) {
        let mut len = self.len;

        const STEP: f64 = 0.1;

        while len > 0. {
            let hex = Hexagon::new(
                self.rank(),
                self.file(),
                len,
                1.,
                self.color,
                FillMode::default(),
                HighlightMode::default(),
            );

            hex.draw_boundaries(painter);
            len -= STEP;
        }
    }

    pub(crate) fn draw_highlights(&self, painter: &mut Painter) {
        match self.mode.current() {
            true => {
                let hex = Hexagon::new(
                    self.rank(),
                    self.file(),
                    self.len,
                    1.,
                    Color::White,
                    FillMode::default(),
                    HighlightMode::default(),
                );

                hex.draw_boundaries(painter);
            }
            false => match self.mode.status() {
                Status::Capturable => {
                    let hex = Hexagon::new(
                        self.rank(),
                        self.file(),
                        self.len,
                        1.,
                        Color::Red,
                        FillMode::default(),
                        HighlightMode::default(),
                    );

                    hex.draw_boundaries(painter);
                }
                Status::Movable => {
                    let hex = Hexagon::new(
                        self.rank(),
                        self.file(),
                        self.len,
                        1.,
                        Color::Blue,
                        FillMode::default(),
                        HighlightMode::default(),
                    );

                    hex.draw_boundaries(painter);
                }
                Status::None => {}
            },
        }
    }

    pub(crate) fn draw_base(&self, painter: &mut Painter) {
        match self.fill_mode {
            FillMode::Filled => {
                self.draw_filled(painter);
            }
            FillMode::Wireframe => {
                self.draw_boundaries(painter);
            }
        }
    }
}

// pub(crate) struct HexagonBase(Hexagon);
//
// impl Shape for HexagonBase {
//     fn draw(&self, painter: &mut Painter) {
//         match self.0.fill_mode {
//             FillMode::Filled => {
//                 self.0.draw_filled(painter);
//             }
//             FillMode::Wireframe => {
//                 self.0.draw_boundaries(painter);
//             }
//         }
//     }
// }
//
// impl From<Hexagon> for HexagonBase {
//     fn from(value: Hexagon) -> Self {
//         Self(value)
//     }
// }
//
// pub(crate) struct HexagonHighlights(Hexagon);
//
// impl Shape for HexagonHighlights {
//     fn draw(&self, painter: &mut Painter) {
//         self.0.draw_highlights(painter);
//     }
// }
//
// impl From<Hexagon> for HexagonHighlights {
//     fn from(value: Hexagon) -> Self {
//         Self(value)
//     }
// }

// pub(crate) struct HexagonFiller(Hexagon);
//
// impl From<Hexagon> for HexagonFiller {
//     fn from(value: Hexagon) -> Self {
//         Self(value)
//     }
// }
//
// impl Shape for HexagonFiller {
//     fn draw(&self, painter: &mut ratatui::widgets::canvas::Painter) {
//         self.0.draw_filled(painter);
//     }
// }
//
// pub(crate) struct HexagonBoundaries(Hexagon);
//
// impl From<Hexagon> for HexagonBoundaries {
//     fn from(value: Hexagon) -> Self {
//         Self(value)
//     }
// }
//
// impl Shape for HexagonBoundaries {
//     fn draw(&self, painter: &mut ratatui::widgets::canvas::Painter) {
//         self.0.draw_boundaries(painter);
//     }
// }
