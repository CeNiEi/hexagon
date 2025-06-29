use crate::hexagon::Hexagon;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Entry<P> {
    hex: Hexagon,
    occupant: Option<P>,
}

impl<P> Default for Entry<P> {
    fn default() -> Self {
        Self {
            hex: Hexagon::default(),
            occupant: None,
        }
    }
}

impl<P> Entry<P> {
    pub(crate) fn new(hex: Hexagon, occupant: Option<P>) -> Self {
        Self { hex, occupant }
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

    pub(crate) fn occupant(&self) -> Option<&P> {
        self.occupant.as_ref()
    }

    pub(crate) fn set_occupant(&mut self, occupant: P) {
        self.occupant = Some(occupant);
    }

    pub(crate) fn occupant_mut(&mut self) -> Option<&mut P> {
        self.occupant.as_mut()
    }
}
