use ratatui::{
    style::Color,
    widgets::canvas::{Circle, Line, Painter, Shape},
};

use super::delta::Delta;

#[derive(Clone, Copy)]
struct Grid {
    height: f64,
    width: f64,
    x: f64,
    y: f64,
    color: Color,
}

impl Grid {
    fn new(x: f64, y: f64, width: f64, height: f64, color: Color) -> Self {
        Self {
            x,
            y,
            height,
            color,
            width,
        }
    }

    fn point_e(&self) -> Delta<f64> {
        Delta::new(self.x + self.width / 2., self.y)
    }

    fn point_w(&self) -> Delta<f64> {
        Delta::new(self.x - self.width / 2., self.y)
    }

    fn point_n(&self) -> Delta<f64> {
        Delta::new(self.x, self.y + self.height / 2.)
    }

    fn point_s(&self) -> Delta<f64> {
        Delta::new(self.x, self.y - self.height / 2.)
    }

    fn point_ne(&self) -> Delta<f64> {
        Delta::new(self.x + self.width / 2., self.y + self.height / 2.)
    }

    fn point_se(&self) -> Delta<f64> {
        Delta::new(self.x + self.width / 2., self.y - self.height / 2.)
    }

    fn point_sw(&self) -> Delta<f64> {
        Delta::new(self.x - self.width / 2., self.y - self.height / 2.)
    }

    fn point_nw(&self) -> Delta<f64> {
        Delta::new(self.x - self.width / 2., self.y + self.height / 2.)
    }

    fn segment_center_se(&self) -> Line {
        let point = self.point_se();

        Line {
            x1: self.x,
            y1: self.y,
            x2: point.x,
            y2: point.y,
            color: self.color,
        }
    }

    fn segment_center_e(&self) -> Line {
        let point = self.point_e();

        Line {
            x1: self.x,
            y1: self.y,
            x2: point.x,
            y2: point.y,
            color: self.color,
        }
    }

    fn segment_center_s(&self) -> Line {
        let point = self.point_s();

        Line {
            x1: self.x,
            y1: self.y,
            x2: point.x,
            y2: point.y,
            color: self.color,
        }
    }

    fn segment_center_sw(&self) -> Line {
        let point = self.point_sw();

        Line {
            x1: self.x,
            y1: self.y,
            x2: point.x,
            y2: point.y,
            color: self.color,
        }
    }
    fn segment_center_w(&self) -> Line {
        let point = self.point_w();

        Line {
            x1: self.x,
            y1: self.y,
            x2: point.x,
            y2: point.y,
            color: self.color,
        }
    }

    fn segment_center_nw(&self) -> Line {
        let point = self.point_nw();

        Line {
            x1: self.x,
            y1: self.y,
            x2: point.x,
            y2: point.y,
            color: self.color,
        }
    }

    fn segment_center_n(&self) -> Line {
        let point = self.point_n();

        Line {
            x1: self.x,
            y1: self.y,
            x2: point.x,
            y2: point.y,
            color: self.color,
        }
    }

    fn segment_center_ne(&self) -> Line {
        let point = self.point_ne();

        Line {
            x1: self.x,
            y1: self.y,
            x2: point.x,
            y2: point.y,
            color: self.color,
        }
    }

    fn segment_e_ne(&self) -> Line {
        let point_e = self.point_e();
        let point_ne = self.point_ne();

        Line {
            x1: point_e.x,
            y1: point_e.y,
            x2: point_ne.x,
            y2: point_ne.y,
            color: self.color,
        }
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

    fn segment_s_se(&self) -> Line {
        let point_s = self.point_s();
        let point_se = self.point_se();

        Line {
            x1: point_s.x,
            y1: point_s.y,
            x2: point_se.x,
            y2: point_se.y,
            color: self.color,
        }
    }

    fn segment_s_sw(&self) -> Line {
        let point_s = self.point_s();
        let point_sw = self.point_sw();

        Line {
            x1: point_s.x,
            y1: point_s.y,
            x2: point_sw.x,
            y2: point_sw.y,
            color: self.color,
        }
    }

    fn segment_w_sw(&self) -> Line {
        let point_w = self.point_w();
        let point_sw = self.point_sw();

        Line {
            x1: point_w.x,
            y1: point_w.y,
            x2: point_sw.x,
            y2: point_sw.y,
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

    fn segment_n_nw(&self) -> Line {
        let point_n = self.point_n();
        let point_nw = self.point_nw();

        Line {
            x1: point_n.x,
            y1: point_n.y,
            x2: point_nw.x,
            y2: point_nw.y,
            color: self.color,
        }
    }

    fn segment_n_ne(&self) -> Line {
        let point_n = self.point_n();
        let point_ne = self.point_ne();

        Line {
            x1: point_n.x,
            y1: point_n.y,
            x2: point_ne.x,
            y2: point_ne.y,
            color: self.color,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Mark<const S: char> {
    grid: Grid,
}

impl<const S: char> Mark<S> {
    pub(crate) fn new(x: f64, y: f64, width: f64, height: f64, color: Color) -> Self {
        let grid = Grid::new(x, y, width, height, color);

        Mark { grid }
    }
}

impl Shape for Mark<'Q'> {
    fn draw(&self, painter: &mut Painter) {
        let segment_e_se = self.grid.segment_e_se();
        let segment_s_se = self.grid.segment_s_se();
        let segment_s_sw = self.grid.segment_s_sw();
        let segment_w_sw = self.grid.segment_w_sw();
        let segment_w_nw = self.grid.segment_w_nw();
        let segment_n_nw = self.grid.segment_n_nw();
        let segment_n_ne = self.grid.segment_n_ne();
        let segment_e_ne = self.grid.segment_e_ne();
        let segment_center_se = self.grid.segment_center_se();

        segment_e_se.draw(painter);
        segment_s_se.draw(painter);
        segment_s_sw.draw(painter);
        segment_w_sw.draw(painter);
        segment_w_nw.draw(painter);
        segment_n_nw.draw(painter);
        segment_n_ne.draw(painter);
        segment_e_ne.draw(painter);
        segment_center_se.draw(painter);
    }
}

impl Shape for Mark<'K'> {
    fn draw(&self, painter: &mut Painter) {
        let segment_w_nw = self.grid.segment_w_nw();
        let segment_w_sw = self.grid.segment_w_sw();
        let segment_center_ne = self.grid.segment_center_ne();
        let segment_center_se = self.grid.segment_center_se();
        let segment_center_w = self.grid.segment_center_w();

        segment_w_nw.draw(painter);
        segment_w_sw.draw(painter);
        segment_center_ne.draw(painter);
        segment_center_se.draw(painter);
        segment_center_w.draw(painter);
    }
}

impl Shape for Mark<'B'> {
    fn draw(&self, painter: &mut Painter) {
        let segment_n_ne = self.grid.segment_n_ne();
        let segment_n_nw = self.grid.segment_n_nw();

        let segment_s_se = self.grid.segment_s_se();
        let segment_s_sw = self.grid.segment_s_sw();

        let segment_center_e = self.grid.segment_center_e();
        let segment_center_n = self.grid.segment_center_n();
        let segment_center_s = self.grid.segment_center_s();

        let segment_e_ne = self.grid.segment_e_ne();
        let segment_e_se = self.grid.segment_e_se();

        segment_s_se.draw(painter);
        segment_s_sw.draw(painter);

        segment_n_ne.draw(painter);
        segment_n_nw.draw(painter);

        segment_center_e.draw(painter);
        segment_center_n.draw(painter);
        segment_center_s.draw(painter);

        segment_e_ne.draw(painter);
        segment_e_se.draw(painter);
    }
}

impl Shape for Mark<'P'> {
    fn draw(&self, painter: &mut Painter) {
        let segment_n_ne = self.grid.segment_n_ne();
        let segment_n_nw = self.grid.segment_n_nw();

        let segment_center_e = self.grid.segment_center_e();
        let segment_center_w = self.grid.segment_center_w();

        let segment_e_ne = self.grid.segment_e_ne();

        let segment_w_nw = self.grid.segment_w_nw();
        let segment_w_sw = self.grid.segment_w_sw();

        segment_n_ne.draw(painter);
        segment_n_nw.draw(painter);

        segment_center_e.draw(painter);
        segment_center_w.draw(painter);

        segment_e_ne.draw(painter);

        segment_w_nw.draw(painter);
        segment_w_sw.draw(painter);
    }
}

impl Shape for Mark<'N'> {
    fn draw(&self, painter: &mut Painter) {
        let segment_w_nw = self.grid.segment_w_nw();
        let segment_w_sw = self.grid.segment_w_sw();

        let segment_e_ne = self.grid.segment_e_ne();
        let segment_e_se = self.grid.segment_e_se();

        let segment_center_se = self.grid.segment_center_se();
        let segment_center_nw = self.grid.segment_center_nw();

        segment_w_nw.draw(painter);
        segment_w_sw.draw(painter);

        segment_e_ne.draw(painter);
        segment_e_se.draw(painter);

        segment_center_se.draw(painter);
        segment_center_nw.draw(painter);
    }
}

impl Shape for Mark<'R'> {
    fn draw(&self, painter: &mut Painter) {
        let segment_n_ne = self.grid.segment_n_ne();
        let segment_n_nw = self.grid.segment_n_nw();

        let segment_center_e = self.grid.segment_center_e();
        let segment_center_w = self.grid.segment_center_w();

        let segment_e_ne = self.grid.segment_e_ne();

        let segment_w_nw = self.grid.segment_w_nw();
        let segment_w_sw = self.grid.segment_w_sw();

        let segment_center_se = self.grid.segment_center_se();

        segment_n_ne.draw(painter);
        segment_n_nw.draw(painter);

        segment_center_e.draw(painter);
        segment_center_w.draw(painter);

        segment_e_ne.draw(painter);

        segment_w_nw.draw(painter);
        segment_w_sw.draw(painter);

        segment_center_se.draw(painter);
    }
}
