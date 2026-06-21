use ratatui::{style::Color, widgets::canvas::Shape};

use crate::{
    board::Board,
    unit::cell::Cell,
    utils::{mark::Mark, moves::Move},
};

pub(crate) mod bishop;
pub(crate) mod king;
pub(crate) mod knight;
pub(crate) mod pawn;
pub(crate) mod queen;
pub(crate) mod rook;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PieceType {
    Bishop,
    King,
    Knight,
    Pawn,
    Queen,
    Rook,
}

pub trait Piece {
    fn color(&self) -> Color;
    fn valid_moves(&self, board: &Board, current: Cell) -> Vec<Move>;
    fn ty(&self) -> PieceType;
}

#[cfg(test)]
mod glinski_move_tests {
    use ratatui::style::Color;
    use strum::IntoEnumIterator;

    use crate::{
        board::Board,
        pieces::{
            bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook,
            Piece,
        },
        unit::cell::Cell,
        utils::{
            depth::Depth,
            file::File,
            fill_mode::FillMode,
            moves::{GeneralMoveType, Move, MoveType, PawnMoveType},
            rank::Rank,
        },
    };

    const ROOK_DELTAS: [(isize, isize); 6] = [(1, 1), (1, -1), (0, -2), (-1, -1), (-1, 1), (0, 2)];
    const BISHOP_DELTAS: [(isize, isize); 6] = [(1, 3), (2, 0), (1, -3), (-1, -3), (-2, 0), (-1, 3)];
    const KING_DELTAS: [(isize, isize); 12] = [
        (1, 3), (1, 1), (2, 0), (1, -1), (1, -3), (0, -2),
        (-1, -3), (-1, -1), (-2, 0), (-1, 1), (-1, 3), (0, 2),
    ];
    const KNIGHT_DELTAS: [(isize, isize); 12] = [
        (-3, -1), (-3, 1), (-2, -4), (-2, 4), (-1, -5), (-1, 5),
        (1, -5), (1, 5), (2, -4), (2, 4), (3, -1), (3, 1),
    ];

    fn empty_board() -> Board {
        Board::empty(0., 0., Depth::new(6).unwrap(), FillMode::Wireframe, false)
    }

    fn file_index(file: File) -> isize {
        file as isize
    }

    fn rank_index(rank: Rank) -> isize {
        rank as isize
    }

    fn to_xy(cell: Cell) -> (isize, isize) {
        let x = file_index(cell.file) - file_index(File::FileF);
        let y = x.abs() + (rank_index(cell.rank) - rank_index(Rank::Rank6)) * 2;
        (x, y)
    }

    fn all_cells() -> Vec<Cell> {
        File::iter()
            .flat_map(|file| {
                Rank::iter().filter_map(move |rank| Cell::try_new(rank, file).ok())
            })
            .collect()
    }

    trait RankIterHack {
        fn iter() -> std::array::IntoIter<Rank, 11>;
    }

    impl RankIterHack for Rank {
        fn iter() -> std::array::IntoIter<Rank, 11> {
            [
                Rank::Rank1,
                Rank::Rank2,
                Rank::Rank3,
                Rank::Rank4,
                Rank::Rank5,
                Rank::Rank6,
                Rank::Rank7,
                Rank::Rank8,
                Rank::Rank9,
                Rank::Rank10,
                Rank::Rank11,
            ]
            .into_iter()
        }
    }

    fn cell_at_xy(x: isize, y: isize) -> Option<Cell> {
        all_cells()
            .into_iter()
            .find(|cell| to_xy(*cell) == (x, y))
    }

    fn expected_slider_moves(start: Cell, directions: &[(isize, isize)]) -> Vec<Cell> {
        let (x, y) = to_xy(start);
        directions
            .iter()
            .flat_map(|(dx, dy)| {
                (1..).map_while(move |scale| cell_at_xy(x + dx * scale, y + dy * scale))
            })
            .collect()
    }

    fn expected_leaper_moves(start: Cell, deltas: &[(isize, isize)]) -> Vec<Cell> {
        let (x, y) = to_xy(start);
        deltas
            .iter()
            .filter_map(|(dx, dy)| cell_at_xy(x + dx, y + dy))
            .collect()
    }

    fn destinations(moves: &[Move]) -> Vec<Cell> {
        moves.iter().map(|mov| mov.move_to).collect()
    }

    fn assert_same_destinations(mut actual: Vec<Cell>, mut expected: Vec<Cell>) {
        actual.sort_by_key(|cell| (file_index(cell.file), rank_index(cell.rank)));
        expected.sort_by_key(|cell| (file_index(cell.file), rank_index(cell.rank)));
        assert_eq!(actual, expected);
    }

    #[test]
    fn rook_empty_board_moves_match_glinski_oracle_from_every_cell() {
        let board = empty_board();
        let rook = Rook::new(Color::White);

        for cell in all_cells() {
            assert_same_destinations(
                destinations(&rook.valid_moves(&board, cell)),
                expected_slider_moves(cell, &ROOK_DELTAS),
            );
        }
    }

    #[test]
    fn bishop_empty_board_moves_match_glinski_oracle_from_every_cell() {
        let board = empty_board();
        let bishop = Bishop::new(Color::White);

        for cell in all_cells() {
            assert_same_destinations(
                destinations(&bishop.valid_moves(&board, cell)),
                expected_slider_moves(cell, &BISHOP_DELTAS),
            );
        }
    }

    #[test]
    fn queen_empty_board_moves_match_glinski_oracle_from_every_cell() {
        let board = empty_board();
        let queen = Queen::new(Color::White);
        let mut queen_deltas = Vec::from(ROOK_DELTAS);
        queen_deltas.extend(BISHOP_DELTAS);

        for cell in all_cells() {
            assert_same_destinations(
                destinations(&queen.valid_moves(&board, cell)),
                expected_slider_moves(cell, &queen_deltas),
            );
        }
    }

    #[test]
    fn king_empty_board_moves_match_glinski_oracle_from_every_cell() {
        let board = empty_board();
        let king = King::new(Color::White);

        for cell in all_cells() {
            assert_same_destinations(
                destinations(&king.valid_moves(&board, cell)),
                expected_leaper_moves(cell, &KING_DELTAS),
            );
        }
    }

    #[test]
    fn knight_empty_board_moves_match_glinski_oracle_from_every_cell() {
        let board = empty_board();
        let knight = Knight::new(Color::White);

        for cell in all_cells() {
            assert_same_destinations(
                destinations(&knight.valid_moves(&board, cell)),
                expected_leaper_moves(cell, &KNIGHT_DELTAS),
            );
        }
    }

    #[test]
    fn sliding_pieces_do_not_include_allied_blockers_and_stop_after_enemy_captures() {
        let start = Cell::new(Rank::Rank6, File::FileF);
        let ally = Cell::new(Rank::Rank6, File::FileH);
        let behind_ally = Cell::new(Rank::Rank6, File::FileI);
        let enemy = Cell::new(Rank::Rank8, File::FileF);
        let behind_enemy = Cell::new(Rank::Rank9, File::FileF);

        let mut board = empty_board();
        board[ally].set_occupant(Pawn::new(Color::White));
        board[enemy].set_occupant(Pawn::new(Color::Black));

        let moves = Rook::new(Color::White).valid_moves(&board, start);

        assert!(!destinations(&moves).contains(&ally));
        assert!(!destinations(&moves).contains(&behind_ally));
        assert!(moves.iter().any(|mov| {
            mov.move_to == enemy && matches!(mov.move_type, MoveType::Rest(GeneralMoveType::Capture))
        }));
        assert!(!destinations(&moves).contains(&behind_enemy));
    }

    #[test]
    fn white_pawn_empty_board_start_and_non_start_moves_match_glinski_rules() {
        let board = empty_board();

        let moves = Pawn::new(Color::White).valid_moves(&board, Cell::new(Rank::Rank5, File::FileF));
        assert_same_destinations(
            destinations(&moves),
            vec![Cell::new(Rank::Rank6, File::FileF), Cell::new(Rank::Rank7, File::FileF)],
        );

        let moves = Pawn::new(Color::White).valid_moves(&board, Cell::new(Rank::Rank6, File::FileF));
        assert_same_destinations(destinations(&moves), vec![Cell::new(Rank::Rank7, File::FileF)]);
    }

    #[test]
    fn black_pawn_empty_board_start_and_non_start_moves_match_glinski_rules() {
        let board = empty_board();

        let moves = Pawn::new(Color::Black).valid_moves(&board, Cell::new(Rank::Rank7, File::FileF));
        assert_same_destinations(
            destinations(&moves),
            vec![Cell::new(Rank::Rank6, File::FileF), Cell::new(Rank::Rank5, File::FileF)],
        );

        let moves = Pawn::new(Color::Black).valid_moves(&board, Cell::new(Rank::Rank6, File::FileF));
        assert_same_destinations(destinations(&moves), vec![Cell::new(Rank::Rank5, File::FileF)]);
    }

    #[test]
    fn pawn_capture_and_promotion_move_types_are_reported() {
        let board = empty_board();
        let moves = Pawn::new(Color::White).valid_moves(&board, Cell::new(Rank::Rank9, File::FileE));

        assert!(moves.iter().any(|mov| {
            mov.move_to == Cell::new(Rank::Rank10, File::FileE)
                && matches!(mov.move_type, MoveType::Pawn(PawnMoveType::NonCapturePromotion))
        }));

        let mut board = empty_board();
        board[Cell::new(Rank::Rank11, File::FileF)].set_occupant(Pawn::new(Color::Black));
        let moves = Pawn::new(Color::White).valid_moves(&board, Cell::new(Rank::Rank10, File::FileE));

        assert!(moves.iter().any(|mov| {
            mov.move_to == Cell::new(Rank::Rank11, File::FileF)
                && matches!(mov.move_type, MoveType::Pawn(PawnMoveType::CapturePromotion))
        }));
    }
}
