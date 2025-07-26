use ratatui::{style::Color, widgets::canvas::Shape};

use crate::{board::TONE_CANVAS_BG, hexagon::Hexagon, pieces::Piece};

use super::{delta::Delta, mark::Mark};

pub(crate) struct Entry {
    hex: Hexagon,
    occupant: Option<Box<dyn Piece>>,
    hide_highlights: bool,
}

impl Entry {
    pub(crate) fn new(
        hex: Hexagon,
        occupant: Option<Box<dyn Piece>>,
        hide_highlights: bool,
    ) -> Self {
        Self {
            hex,
            occupant,
            hide_highlights,
        }
    }

    pub(crate) fn hex(&self) -> &Hexagon {
        &self.hex
    }

    pub(crate) fn is_occupied(&self) -> bool {
        self.occupant.is_some()
    }

    pub(crate) fn hex_mut(&mut self) -> &mut Hexagon {
        &mut self.hex
    }

    pub(crate) fn occupant(&self) -> Option<&Box<dyn Piece>> {
        self.occupant.as_ref()
    }

    pub(crate) fn set_occupant(&mut self, occupant: impl Piece + 'static) {
        self.occupant = Some(Box::new(occupant) as Box<dyn Piece>);
    }

    pub(crate) fn occupant_mut(&mut self) -> Option<&mut Box<dyn Piece>> {
        self.occupant.as_mut()
    }
}

impl Shape for Entry {
    fn draw(&self, painter: &mut ratatui::widgets::canvas::Painter) {
        let Delta { x, y } = self.hex.center();

        self.hex.draw_base(painter);

        if !self.hide_highlights {
            self.hex.draw_highlights(painter);
        }

        let mark_width = self.hex.len() / 2.;
        let mark_height = self.hex.len() / 2.;

        if let Some(piece) = self.occupant() {
            let mark_color = match piece.color() {
                Color::Black => Color::Yellow,
                Color::White => Color::Green,
                _ => unreachable!(),
            };

            match piece.ty() {
                crate::pieces::PieceType::Queen => {
                    Mark::<'Q'>::new(x, y, mark_width, mark_height, mark_color).draw(painter);
                }
                crate::pieces::PieceType::King => {
                    Mark::<'K'>::new(x, y, mark_width, mark_height, mark_color).draw(painter);
                }
                crate::pieces::PieceType::Knight => {
                    Mark::<'N'>::new(x, y, mark_width, mark_height, mark_color).draw(painter);
                }
                crate::pieces::PieceType::Pawn => {
                    Mark::<'P'>::new(x, y, mark_width, mark_height, mark_color).draw(painter);
                }
                crate::pieces::PieceType::Bishop => {
                    Mark::<'B'>::new(x, y, mark_width, mark_height, mark_color).draw(painter);
                }
                crate::pieces::PieceType::Rook => {
                    Mark::<'R'>::new(x, y, mark_width, mark_height, mark_color).draw(painter);
                }
            }
        }
    }
}
