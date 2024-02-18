use ratatui::{
    style::Color,
    widgets::canvas::{Line, Shape},
};

use crate::backend::constants::{BLACK_BG, GRAY_BG, WHITE_BG};

use super::direction::Direction;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Hexagon {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) side: f64,
    pub(crate) color: Color,
}

impl Hexagon {
    pub(crate) fn new(x: f64, y: f64, side: f64, color: Color) -> Self {
        Self { x, y, side, color }
    }

    pub(crate) fn next(&self, direction: Direction) -> Self {
        let color = match (self.color, direction) {
            (GRAY_BG, Direction::NE | Direction::NW | Direction::S) => BLACK_BG,
            (GRAY_BG, Direction::SE | Direction::SW | Direction::N) => WHITE_BG,

            (BLACK_BG, Direction::NE | Direction::NW | Direction::S) => WHITE_BG,
            (BLACK_BG, Direction::SE | Direction::SW | Direction::N) => GRAY_BG,

            (WHITE_BG, Direction::NE | Direction::NW | Direction::S) => GRAY_BG,
            (WHITE_BG, Direction::SE | Direction::SW | Direction::N) => BLACK_BG,

            _ => panic!("[FATAL]: Impossible Background color encountered"),
        };

        match direction {
            Direction::S => Self {
                y: self.y - (self.side * 3_f64.sqrt()),
                x: self.x,
                side: self.side,
                color,
            },
            Direction::N => Self {
                y: self.y + (self.side * 3_f64.sqrt()),
                x: self.x,
                side: self.side,
                color,
            },
            Direction::SE => Self {
                y: self.y - (self.side * 3_f64.sqrt() / 2.),
                x: self.x + (self.side * 1.5),
                side: self.side,
                color,
            },
            Direction::SW => Self {
                y: self.y - (self.side * 3_f64.sqrt() / 2.),
                x: self.x - (self.side * 1.5),
                side: self.side,
                color,
            },
            Direction::NW => Self {
                y: self.y + (self.side * 3_f64.sqrt() / 2.),
                x: self.x - (self.side * 1.5),
                side: self.side,
                color,
            },
            Direction::NE => Self {
                y: self.y + (self.side * 3_f64.sqrt() / 2.),
                x: self.x + (self.side * 1.5),
                side: self.side,
                color,
            },
        }
    }
}

impl Shape for Hexagon {
    fn draw(&self, painter: &mut ratatui::widgets::canvas::Painter) {
        let padding = self.side / 5.;

        let side = self.side - padding;

        let point_e = (self.x + side, self.y);

        let point_se = (self.x + (side / 2.), self.y - side * (3_f64.sqrt() / 2.));

        let point_sw = (self.x - (side / 2.), self.y - side * (3_f64.sqrt() / 2.));

        let point_w = (self.x - side, self.y);

        let point_nw = (self.x - (side / 2.), self.y + side * (3_f64.sqrt() / 2.));

        let point_ne = (self.x + (side / 2.), self.y + side * (3_f64.sqrt() / 2.));

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
