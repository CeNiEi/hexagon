pub(crate) mod cell;
pub(crate) mod mover;
pub(crate) mod piece;
pub(crate) mod utils;

use std::collections::HashMap;

use crate::board::piece::Piece;

pub(crate) struct Board {
    inner: HashMap<cell::Cell, Box<dyn Piece>>,
}

impl Board {
    fn directional_moves(&self) -> () {
        todo!()
    }
}
