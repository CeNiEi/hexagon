use crate::board::cell::{file::File, rank::Rank, Cell};
use std::{collections::HashSet, sync::OnceLock};

pub(crate) const WHITE_PAWN_STARTING_LOCATIONS: OnceLock<HashSet<Cell>> = OnceLock::new();

pub(crate) const BLACK_PAWN_STARTING_LOCATIONS: OnceLock<HashSet<Cell>> = OnceLock::new();
