use ratatui::style::Color;

use super::cell::{file::File, rank::Rank, Cell};

pub(crate) const WHITE_PAWN_STARTING_LOCATIONS: [Cell; 9] = [
    Cell::new(Rank::Rank1, File::FileB),
    Cell::new(Rank::Rank2, File::FileC),
    Cell::new(Rank::Rank3, File::FileD),
    Cell::new(Rank::Rank4, File::FileE),
    Cell::new(Rank::Rank5, File::FileF),
    Cell::new(Rank::Rank4, File::FileG),
    Cell::new(Rank::Rank3, File::FileH),
    Cell::new(Rank::Rank2, File::FileI),
    Cell::new(Rank::Rank1, File::FileK),
];

pub(crate) const WHITE_ROOK_STARTING_LOCATION: [Cell; 2] = [
    Cell::new(Rank::Rank1, File::FileC),
    Cell::new(Rank::Rank1, File::FileI),
];

pub(crate) const WHITE_KNIGHT_STARTING_LOCATION: [Cell; 2] = [
    Cell::new(Rank::Rank1, File::FileD),
    Cell::new(Rank::Rank1, File::FileH),
];

pub(crate) const WHITE_KING_STARTING_LOCATION: Cell = Cell::new(Rank::Rank1, File::FileG);

pub(crate) const WHITE_QUEEN_STARTING_LOCATION: Cell = Cell::new(Rank::Rank1, File::FileE);

pub(crate) const WHITE_BISHOP_STARTING_LOCATION: [Cell; 3] = [
    Cell::new(Rank::Rank1, File::FileF),
    Cell::new(Rank::Rank2, File::FileF),
    Cell::new(Rank::Rank3, File::FileF),
];

pub(crate) const GRAY_BG: Color = Color::Red;
pub(crate) const BLACK_BG: Color = Color::Blue;
pub(crate) const WHITE_BG: Color = Color::Green;
