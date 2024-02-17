use ratatui::{
    style::Color,
    widgets::canvas::{Line, Shape},
};

use super::direction::Direction;

#[derive(Clone, Copy)]
pub(crate) struct Hexagon {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) side: f64,
    pub(crate) color: Color,
    pub(crate) scale_factor: f64,
}

impl Hexagon {
    pub(crate) fn new(x: f64, y: f64, side: f64, scale_factor: f64, color: Color) -> Self {
        Self {
            x,
            y,
            side,
            color,
            scale_factor,
        }
    }

    pub(crate) fn next(&self, direction: Direction) -> Self {
        let padding = self.side / 10.;

        match direction {
            Direction::S => Self {
                y: self.y - self.side * 3_f64.sqrt() - padding,
                x: self.x,
                side: self.side,
                scale_factor: self.scale_factor,
                color: self.color,
            },
            Direction::N => Self {
                y: self.y + self.side * 3_f64.sqrt() + padding,
                x: self.x,
                side: self.side,
                scale_factor: self.scale_factor,
                color: self.color,
            },
            Direction::SE => Self {
                y: self.y - self.side + padding,
                x: self.x + (self.side * 1.5 + padding) * self.scale_factor,
                side: self.side,
                scale_factor: self.scale_factor,
                color: self.color,
            },
            Direction::SW => Self {
                y: self.y - self.side + padding,
                x: self.x - (self.side * 1.5 + padding) * self.scale_factor,
                side: self.side,
                scale_factor: self.scale_factor,
                color: self.color,
            },
            Direction::NW => Self {
                y: self.y + self.side - padding,
                x: self.x - (self.side * 1.5 + padding) * self.scale_factor,
                side: self.side,
                scale_factor: self.scale_factor,
                color: self.color,
            },
            Direction::NE => Self {
                y: self.y + self.side - padding,
                x: self.x + (self.side * 1.5 + padding) * self.scale_factor,
                side: self.side,
                scale_factor: self.scale_factor,
                color: self.color,
            },
        }
    }
}

impl Shape for Hexagon {
    fn draw(&self, painter: &mut ratatui::widgets::canvas::Painter) {
        // let padding = 2;

        let point_e = (self.x + self.side * self.scale_factor, self.y);

        let point_se = (
            self.x + (self.side / 2.) * self.scale_factor,
            self.y - self.side * (3_f64.sqrt() / 2.),
        );

        let point_sw = (
            self.x - (self.side / 2.) * self.scale_factor,
            self.y - self.side * (3_f64.sqrt() / 2.),
        );

        let point_w = (self.x - self.side * self.scale_factor, self.y);

        let point_nw = (
            self.x - (self.side / 2.) * self.scale_factor,
            self.y + self.side * (3_f64.sqrt() / 2.),
        );

        let point_ne = (
            self.x + (self.side / 2.) * self.scale_factor,
            self.y + self.side * (3_f64.sqrt() / 2.),
        );

        let segment_e_se = Line {
            x1: point_e.0,
            y1: point_e.1,
            x2: point_se.0,
            y2: point_se.1,
            color: self.color,
        };

        let segment_se_sw = Line {
            x1: point_se.0,
            y1: point_se.1,
            x2: point_sw.0,
            y2: point_sw.1,
            color: self.color,
        };

        let segment_sw_w = Line {
            x1: point_sw.0,
            y1: point_sw.1,
            x2: point_w.0,
            y2: point_w.1,
            color: self.color,
        };

        let segment_w_nw = Line {
            x1: point_w.0,
            y1: point_w.1,
            x2: point_nw.0,
            y2: point_nw.1,
            color: self.color,
        };

        let segment_nw_ne = Line {
            x1: point_nw.0,
            y1: point_nw.1,
            x2: point_ne.0,
            y2: point_ne.1,
            color: self.color,
        };

        let segment_ne_e = Line {
            x1: point_ne.0,
            y1: point_ne.1,
            x2: point_e.0,
            y2: point_e.1,
            color: self.color,
        };

        segment_e_se.draw(painter);
        segment_se_sw.draw(painter);
        segment_sw_w.draw(painter);
        segment_w_nw.draw(painter);
        segment_nw_ne.draw(painter);
        segment_ne_e.draw(painter);
    }
}
