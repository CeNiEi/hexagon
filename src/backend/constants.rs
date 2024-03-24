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

pub(crate) const BLACK_PAWN_STARTING_LOCATIONS: [Cell; 9] = [
    Cell::new(Rank::Rank7, File::FileB),
    Cell::new(Rank::Rank7, File::FileC),
    Cell::new(Rank::Rank7, File::FileD),
    Cell::new(Rank::Rank7, File::FileE),
    Cell::new(Rank::Rank7, File::FileF),
    Cell::new(Rank::Rank7, File::FileG),
    Cell::new(Rank::Rank7, File::FileH),
    Cell::new(Rank::Rank7, File::FileI),
    Cell::new(Rank::Rank7, File::FileK),
];

pub(crate) const BLACK_ROOK_STARTING_LOCATION: [Cell; 2] = [
    Cell::new(Rank::Rank8, File::FileC),
    Cell::new(Rank::Rank8, File::FileI),
];

pub(crate) const WHITE_ROOK_STARTING_LOCATION: [Cell; 2] = [
    Cell::new(Rank::Rank1, File::FileC),
    Cell::new(Rank::Rank1, File::FileI),
];

pub(crate) const WHITE_KNIGHT_STARTING_LOCATION: [Cell; 2] = [
    Cell::new(Rank::Rank1, File::FileD),
    Cell::new(Rank::Rank1, File::FileH),
];

pub(crate) const BLACK_KNIGHT_STARTING_LOCATION: [Cell; 2] = [
    Cell::new(Rank::Rank9, File::FileD),
    Cell::new(Rank::Rank9, File::FileH),
];

pub(crate) const WHITE_KING_STARTING_LOCATION: Cell = Cell::new(Rank::Rank1, File::FileG);

pub(crate) const BLACK_KING_STARTING_LOCATION: Cell = Cell::new(Rank::Rank10, File::FileG);

pub(crate) const WHITE_QUEEN_STARTING_LOCATION: Cell = Cell::new(Rank::Rank1, File::FileE);

pub(crate) const BLACK_QUEEN_STARTING_LOCATION: Cell = Cell::new(Rank::Rank10, File::FileE);

pub(crate) const WHITE_BISHOP_STARTING_LOCATION: [Cell; 3] = [
    Cell::new(Rank::Rank1, File::FileF),
    Cell::new(Rank::Rank2, File::FileF),
    Cell::new(Rank::Rank3, File::FileF),
];

pub(crate) const BLACK_BISHOP_STARTING_LOCATION: [Cell; 3] = [
    Cell::new(Rank::Rank9, File::FileF),
    Cell::new(Rank::Rank10, File::FileF),
    Cell::new(Rank::Rank11, File::FileF),
];

pub(crate) const ALL_CELLS: [Cell; 91] = [
    Cell::new(Rank::Rank1, File::FileA),
    Cell::new(Rank::Rank1, File::FileB),
    Cell::new(Rank::Rank1, File::FileC),
    Cell::new(Rank::Rank1, File::FileD),
    Cell::new(Rank::Rank1, File::FileE),
    Cell::new(Rank::Rank1, File::FileF),
    Cell::new(Rank::Rank1, File::FileG),
    Cell::new(Rank::Rank1, File::FileH),
    Cell::new(Rank::Rank1, File::FileI),
    Cell::new(Rank::Rank1, File::FileK),
    Cell::new(Rank::Rank1, File::FileL),
    Cell::new(Rank::Rank2, File::FileA),
    Cell::new(Rank::Rank2, File::FileB),
    Cell::new(Rank::Rank2, File::FileC),
    Cell::new(Rank::Rank2, File::FileD),
    Cell::new(Rank::Rank2, File::FileE),
    Cell::new(Rank::Rank2, File::FileF),
    Cell::new(Rank::Rank2, File::FileG),
    Cell::new(Rank::Rank2, File::FileH),
    Cell::new(Rank::Rank2, File::FileI),
    Cell::new(Rank::Rank2, File::FileK),
    Cell::new(Rank::Rank2, File::FileL),
    Cell::new(Rank::Rank3, File::FileA),
    Cell::new(Rank::Rank3, File::FileB),
    Cell::new(Rank::Rank3, File::FileC),
    Cell::new(Rank::Rank3, File::FileD),
    Cell::new(Rank::Rank3, File::FileE),
    Cell::new(Rank::Rank3, File::FileF),
    Cell::new(Rank::Rank3, File::FileG),
    Cell::new(Rank::Rank3, File::FileH),
    Cell::new(Rank::Rank3, File::FileI),
    Cell::new(Rank::Rank3, File::FileK),
    Cell::new(Rank::Rank3, File::FileL),
    Cell::new(Rank::Rank4, File::FileA),
    Cell::new(Rank::Rank4, File::FileB),
    Cell::new(Rank::Rank4, File::FileC),
    Cell::new(Rank::Rank4, File::FileD),
    Cell::new(Rank::Rank4, File::FileE),
    Cell::new(Rank::Rank4, File::FileF),
    Cell::new(Rank::Rank4, File::FileG),
    Cell::new(Rank::Rank4, File::FileH),
    Cell::new(Rank::Rank4, File::FileI),
    Cell::new(Rank::Rank4, File::FileK),
    Cell::new(Rank::Rank4, File::FileL),
    Cell::new(Rank::Rank5, File::FileA),
    Cell::new(Rank::Rank5, File::FileB),
    Cell::new(Rank::Rank5, File::FileC),
    Cell::new(Rank::Rank5, File::FileD),
    Cell::new(Rank::Rank5, File::FileE),
    Cell::new(Rank::Rank5, File::FileF),
    Cell::new(Rank::Rank5, File::FileG),
    Cell::new(Rank::Rank5, File::FileH),
    Cell::new(Rank::Rank5, File::FileI),
    Cell::new(Rank::Rank5, File::FileK),
    Cell::new(Rank::Rank5, File::FileL),
    Cell::new(Rank::Rank6, File::FileA),
    Cell::new(Rank::Rank6, File::FileB),
    Cell::new(Rank::Rank6, File::FileC),
    Cell::new(Rank::Rank6, File::FileD),
    Cell::new(Rank::Rank6, File::FileE),
    Cell::new(Rank::Rank6, File::FileF),
    Cell::new(Rank::Rank6, File::FileG),
    Cell::new(Rank::Rank6, File::FileH),
    Cell::new(Rank::Rank6, File::FileI),
    Cell::new(Rank::Rank6, File::FileK),
    Cell::new(Rank::Rank6, File::FileL),
    Cell::new(Rank::Rank7, File::FileB),
    Cell::new(Rank::Rank7, File::FileC),
    Cell::new(Rank::Rank7, File::FileD),
    Cell::new(Rank::Rank7, File::FileE),
    Cell::new(Rank::Rank7, File::FileF),
    Cell::new(Rank::Rank7, File::FileG),
    Cell::new(Rank::Rank7, File::FileH),
    Cell::new(Rank::Rank7, File::FileI),
    Cell::new(Rank::Rank7, File::FileK),
    Cell::new(Rank::Rank8, File::FileC),
    Cell::new(Rank::Rank8, File::FileD),
    Cell::new(Rank::Rank8, File::FileE),
    Cell::new(Rank::Rank8, File::FileF),
    Cell::new(Rank::Rank8, File::FileG),
    Cell::new(Rank::Rank8, File::FileH),
    Cell::new(Rank::Rank8, File::FileI),
    Cell::new(Rank::Rank9, File::FileD),
    Cell::new(Rank::Rank9, File::FileE),
    Cell::new(Rank::Rank9, File::FileF),
    Cell::new(Rank::Rank9, File::FileG),
    Cell::new(Rank::Rank9, File::FileH),
    Cell::new(Rank::Rank10, File::FileE),
    Cell::new(Rank::Rank10, File::FileF),
    Cell::new(Rank::Rank10, File::FileG),
    Cell::new(Rank::Rank11, File::FileF),
];
