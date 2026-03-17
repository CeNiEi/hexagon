use ratatui::{style::Color, widgets::canvas::Shape};

use crate::{
    unit::cell::Cell,
    utils::{
        direction::Direction, file::File, mark::Mark, moves::single_direction_moves, rank::Rank,
    },
};

use super::{Board, Move, Piece};

pub(crate) struct Bishop {
    color: Color,
}

impl Bishop {
    pub(crate) fn new(color: Color) -> Self {
        Self { color }
    }
}

pub(crate) const WHITE_BISHOP_STARTING_CELLS: [Cell; 3] = [
    unsafe { Cell::from_raw_parts(Rank::Rank1, File::FileF) },
    unsafe { Cell::from_raw_parts(Rank::Rank2, File::FileF) },
    unsafe { Cell::from_raw_parts(Rank::Rank3, File::FileF) },
];

pub(crate) const BLACK_BISHOP_STARTING_CELLS: [Cell; 3] = [
    unsafe { Cell::from_raw_parts(Rank::Rank9, File::FileF) },
    unsafe { Cell::from_raw_parts(Rank::Rank10, File::FileF) },
    unsafe { Cell::from_raw_parts(Rank::Rank11, File::FileF) },
];

impl Piece for Bishop {
    fn ty(&self) -> super::PieceType {
        super::PieceType::Bishop
    }

    fn color(&self) -> Color {
        self.color
    }

    fn valid_moves(&self, board: &Board, current: Cell) -> Vec<Move> {
        const ALLOWED_DIRECTIONS: [Direction; 6] = [
            Direction::Clock1,
            Direction::Clock3,
            Direction::Clock5,
            Direction::Clock7,
            Direction::Clock9,
            Direction::Clock11,
        ];

        let valid_moves = ALLOWED_DIRECTIONS
            .into_iter()
            .flat_map(|direction| single_direction_moves(current, self.color, direction, board))
            .collect::<Vec<_>>();

        valid_moves
    }
}

#[cfg(test)]
mod tests {
    use ratatui::style::Color;

    use crate::{
        board_set,
        pieces::Piece,
        unit::cell::Cell,
        utils::{
            Step,
            file::File,
            moves::{GeneralMoveType, MoveType},
            rank::Rank,
        },
    };

    use super::Bishop;

    #[test]
    fn test_bishop_moves_empty_board() {
        let board = board_set!();
        let bishop = Bishop::new(Color::White);
        let start_cell = Cell::new(Rank::Rank6, File::FileF);

        let moves = bishop.valid_moves(&board, start_cell);

        assert!(!moves.is_empty(), "Bishop should have moves on empty board");

        // All moves on empty board should be NonCapture
        for m in &moves {
            assert!(
                matches!(m.move_type, MoveType::Rest(GeneralMoveType::NonCapture)),
                "All moves on empty board should be NonCapture, found {:?}",
                m.move_type
            );
        }

        let destinations: Vec<Cell> = moves.iter().map(|m| m.move_to).collect();

        assert!(destinations.contains(&Cell::new(Rank::Rank7, File::FileG)));
        assert!(destinations.contains(&Cell::new(Rank::Rank8, File::FileH)));

        assert!(destinations.contains(&Cell::new(Rank::Rank5, File::FileE)));
        assert!(destinations.contains(&Cell::new(Rank::Rank4, File::FileD)));

        assert!(destinations.contains(&Cell::new(Rank::Rank5, File::FileG)));
        assert!(destinations.contains(&Cell::new(Rank::Rank4, File::FileH)));
    }

    // #[test]
    // fn test_bishop_blocked_by_same_color() {
    //     let board = board_set!(
    //         (Color::White, bishop, Bishop) on [Rank::Rank6, File::FileF],
    //         (Color::White, pawn, Pawn) on [Rank::Rank7, File::FileG],
    //         (Color::White, pawn, Pawn) on [Rank::Rank5, File::FileE]
    //     );
    //
    //     let bishop = Bishop::new(Color::White);
    //     let start_cell = Cell::new(Rank::Rank6, File::FileF);
    //
    //     let moves = bishop.valid_moves(&board, start_cell);
    //     let destinations: Vec<Cell> = moves.iter().map(|m| m.move_to).collect();
    //
    //     // Check that same-color pieces block movement
    //     assert!(
    //         !destinations.contains(&Cell::new(Rank::Rank7, File::FileG)),
    //         "Bishop should not be able to move through same color piece"
    //     );
    //     assert!(
    //         !destinations.contains(&Cell::new(Rank::Rank8, File::FileH)),
    //         "Bishop should not be able to jump over same color piece"
    //     );
    //
    //     assert!(
    //         !destinations.contains(&Cell::new(Rank::Rank5, File::FileE)),
    //         "Bishop should not be able to capture same color piece"
    //     );
    //     assert!(
    //         !destinations.contains(&Cell::new(Rank::Rank4, File::FileD)),
    //         "Bishop should not be able to jump over same color piece"
    //     );
    //
    //     // All remaining moves should be NonCapture
    //     for m in &moves {
    //         assert!(
    //             matches!(m.move_type, MoveType::Rest(RestMoveType::NonCapture)),
    //             "Moves to empty squares should be NonCapture, found {:?}",
    //             m.move_type
    //         );
    //     }
    // }
    //
    // #[test]
    // fn test_bishop_captures_opposite_color() {
    //     let board = board_set!(
    //         (Color::White, bishop, Bishop) on [Rank::Rank6, File::FileF],
    //         (Color::Black, pawn, Pawn) on [Rank::Rank7, File::FileG],
    //         (Color::Black, pawn, Pawn) on [Rank::Rank5, File::FileE]
    //     );
    //
    //     let bishop = Bishop::new(Color::White);
    //     let start_cell = Cell::new(Rank::Rank6, File::FileF);
    //
    //     let moves = bishop.valid_moves(&board, start_cell);
    //
    //     // Check capture moves
    //     let capture_g7 = moves
    //         .iter()
    //         .find(|m| m.move_to == Cell::new(Rank::Rank7, File::FileG));
    //     assert!(
    //         capture_g7.is_some(),
    //         "Bishop should be able to capture on G7"
    //     );
    //     assert!(
    //         matches!(
    //             capture_g7.unwrap().move_type,
    //             MoveType::Rest(RestMoveType::Capture)
    //         ),
    //         "Move to G7 should be a Capture, found {:?}",
    //         capture_g7.unwrap().move_type
    //     );
    //
    //     let capture_e5 = moves
    //         .iter()
    //         .find(|m| m.move_to == Cell::new(Rank::Rank5, File::FileE));
    //     assert!(
    //         capture_e5.is_some(),
    //         "Bishop should be able to capture on E5"
    //     );
    //     assert!(
    //         matches!(
    //             capture_e5.unwrap().move_type,
    //             MoveType::Rest(RestMoveType::Capture)
    //         ),
    //         "Move to E5 should be a Capture, found {:?}",
    //         capture_e5.unwrap().move_type
    //     );
    //
    //     // Check blocked squares beyond captures
    //     let destinations: Vec<Cell> = moves.iter().map(|m| m.move_to).collect();
    //     assert!(
    //         !destinations.contains(&Cell::new(Rank::Rank8, File::FileH)),
    //         "Bishop should not be able to jump over opposite color piece"
    //     );
    //     assert!(
    //         !destinations.contains(&Cell::new(Rank::Rank4, File::FileD)),
    //         "Bishop should not be able to jump over opposite color piece"
    //     );
    //
    //     // Check that other moves are NonCapture
    //     for m in &moves {
    //         if m.move_to != Cell::new(Rank::Rank7, File::FileG)
    //             && m.move_to != Cell::new(Rank::Rank5, File::FileE)
    //         {
    //             assert!(
    //                 matches!(m.move_type, MoveType::Rest(RestMoveType::NonCapture)),
    //                 "Non-capture moves should be NonCapture, found {:?} for move to {:?}",
    //                 m.move_type,
    //                 m.move_to
    //             );
    //         }
    //     }
    // }
    //
    // #[test]
    // fn test_bishop_at_edge() {
    //     let board = board_set!();
    //     let bishop = Bishop::new(Color::White);
    //     let start_cell = Cell::new(Rank::Rank1, File::FileA);
    //
    //     let moves = bishop.valid_moves(&board, start_cell);
    //
    //     assert!(
    //         !moves.is_empty(),
    //         "Bishop should have some moves even at edge"
    //     );
    //
    //     let destinations: Vec<Cell> = moves.iter().map(|m| m.to).collect();
    //
    //     assert!(destinations.contains(&Cell::new(Rank::Rank2, File::FileB)));
    //     assert!(destinations.contains(&Cell::new(Rank::Rank3, File::FileC)));
    // }
    //
    // #[test]
    // fn test_bishop_diagonal_directions() {
    //     let board = board_set!();
    //     let bishop = Bishop::new(Color::Black);
    //     let start_cell = Cell::new(Rank::Rank6, File::FileF);
    //
    //     let moves = bishop.valid_moves(&board, start_cell);
    //     let destinations: Vec<Cell> = moves.iter().map(|m| m.to).collect();
    //
    //     let diagonal_directions = [
    //         (Rank::Rank7, File::FileG),
    //         (Rank::Rank5, File::FileE),
    //         (Rank::Rank5, File::FileG),
    //         (Rank::Rank7, File::FileE),
    //         (Rank::Rank8, File::FileF),
    //         (Rank::Rank4, File::FileF),
    //     ];
    //
    //     for (rank, file) in diagonal_directions {
    //         if let Ok(cell) = Cell::try_new(rank, file) {
    //             assert!(
    //                 destinations.contains(&cell),
    //                 "Bishop should be able to move to {:?}, {:?}",
    //                 rank,
    //                 file
    //             );
    //         }
    //     }
    // }
}
