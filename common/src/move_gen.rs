use crate::lookup::FILE_BITBOARDS;
use crate::lookup::RANK_BITBOARDS;
use crate::{
    bb, BitBoard, BitBoardIterator, BitBoardable, Board, Color, Piece, ResolvedMovement, Square,
};

const NORTH: i8 = 8;
const EAST: i8 = -1;
const SOUTH: i8 = -8;
const WEST: i8 = 1;
const NORTH_EAST: i8 = 9;
const NORTH_WEST: i8 = 7;
const SOUTH_EAST: i8 = -7;
const SOUTH_WEST: i8 = -9;

/// Scan a board in a direction until we hit a blocker or we hit the edge of the board. This will
/// calculate the ray attack for a sliding piece that will take blocker into account. This will
/// only scan in a positive direction for example "north", "east", "north east" and "north west".
/// You are unable to shift by a negative number. NOTE: This will also include any blockers as
/// valid moves so you can calculate any potential captures.
///
/// https://www.chessprogramming.org/On_an_empty_Board#PositiveRays
fn scan_bishop(occupancies: BitBoard, square_board: BitBoard, direction: i8) -> BitBoard {
    let mut ray_board = square_board;
    let mut i = square_board;

    loop {
        i = match direction {
            NORTH_EAST => (i << 7) & !FILE_BITBOARDS[0],
            NORTH_WEST => (i << 9) & !FILE_BITBOARDS[7],
            SOUTH_EAST => (i >> 9) & !FILE_BITBOARDS[0],
            SOUTH_WEST => (i >> 7) & !FILE_BITBOARDS[7],
            _ => break,
        };

        // Stop if the shift moves the piece off the board
        if i == 0 {
            break;
        }

        ray_board |= i;

        // Stop if we hit a blocker
        if occupancies != 0 && i & occupancies != 0 {
            break;
        }
    }

    ray_board
}

fn scan_rook(occupancies: BitBoard, square_board: BitBoard, direction: i8) -> BitBoard {
    let mut ray_board = square_board;
    let mut i = square_board;

    loop {
        // Perform the shift
        i = match direction {
            EAST => (i >> 1) & !FILE_BITBOARDS[0],
            WEST => (i << 1) & !FILE_BITBOARDS[7],
            SOUTH => (i >> 8) & !RANK_BITBOARDS[7],
            NORTH => (i << 8) & !RANK_BITBOARDS[0],
            _ => break,
        };

        // Stop if the shift goes out of bounds
        if i == 0 {
            break;
        }

        ray_board |= i; // Include the current square in the ray

        // Stop if we hit a blocker
        if occupancies != 0 && i & occupancies != 0 {
            break;
        }
    }

    ray_board
}

/// Calculates all of the available attacks that a rook can make given that its on a given square.
fn rook_attacks(square_board: BitBoard, occupancies: BitBoard) -> BitBoard {
    let line_attacks = scan_rook(occupancies, square_board, NORTH)
        | scan_rook(occupancies, square_board, SOUTH)
        | scan_rook(occupancies, square_board, EAST)
        | scan_rook(occupancies, square_board, WEST);

    line_attacks & !square_board
}

/// Calculates all of the available attacks that a bishop can make given that its on a given
/// square.
fn bishop_attacks(square_board: BitBoard, occupancies: BitBoard) -> BitBoard {
    let line_attacks = scan_bishop(occupancies, square_board, NORTH_EAST)
        | scan_bishop(occupancies, square_board, NORTH_WEST)
        | scan_bishop(occupancies, square_board, SOUTH_EAST)
        | scan_bishop(occupancies, square_board, SOUTH_WEST);

    line_attacks & !square_board
}

/// Calculate all of the attacks that a queen can make given that its on a given square. This one
/// is calculated by combining the rook and bishop attacks.
fn queen_attacks(square_board: BitBoard, occupancies: BitBoard) -> BitBoard {
    rook_attacks(square_board, occupancies) | bishop_attacks(square_board, occupancies)
}

/// Create the knight_moves from there move position. This may need to be moved into a
/// lookup of pre built bitboards like the rook and bishop attacks.
///
/// https://www.chessprogramming.org/Knight_Pattern
fn knight_attacks(bb: BitBoard) -> BitBoard {
    (bb >> 6 & !(FILE_BITBOARDS[7] | FILE_BITBOARDS[6]))
        | (bb >> 15 & !FILE_BITBOARDS[7])
        | (bb >> 17 & !FILE_BITBOARDS[0])
        | (bb >> 10 & !(FILE_BITBOARDS[0] | FILE_BITBOARDS[1]))
        | (bb << 6 & !(FILE_BITBOARDS[0] | FILE_BITBOARDS[1]))
        | (bb << 15 & !FILE_BITBOARDS[0])
        | (bb << 17 & !FILE_BITBOARDS[7])
        | (bb << 10 & !(FILE_BITBOARDS[7] | FILE_BITBOARDS[6]))
}

/// https://www.chessprogramming.org/King_Pattern
fn king_attacks(bb: BitBoard) -> BitBoard {
    (bb >> 7 & !FILE_BITBOARDS[7])
        | bb >> 8
        | (bb >> 9 & !FILE_BITBOARDS[0])
        | (bb >> 1 & !FILE_BITBOARDS[0])
        | (bb << 1 & !FILE_BITBOARDS[7])
        | (bb << 9 & !FILE_BITBOARDS[7])
        | bb << 8
        | (bb << 7 & !FILE_BITBOARDS[0])
}

fn pawn_moves(board: &Board, piece_board: BitBoard, occupancies: BitBoard) -> BitBoard {
    let one_square = match board.turn {
        Color::Black => piece_board >> 8 & !occupancies,
        Color::White => piece_board << 8 & !occupancies,
    };

    let two_square = match board.turn {
        Color::Black => one_square >> 8 & !occupancies & RANK_BITBOARDS[4],
        Color::White => one_square << 8 & !occupancies & RANK_BITBOARDS[3],
    };

    let left_attack = match board.turn {
        Color::Black => piece_board >> 9 & board.white_pieces() & !FILE_BITBOARDS[0],
        Color::White => piece_board << 9 & board.black_pieces() & !FILE_BITBOARDS[7],
    };

    let right_attack = match board.turn {
        Color::Black => piece_board >> 7 & board.white_pieces() & !FILE_BITBOARDS[7],
        Color::White => piece_board << 7 & board.black_pieces() & !FILE_BITBOARDS[0],
    };

    one_square | two_square | left_attack | right_attack
}

pub fn pseudo_moves(board: &Board) -> Vec<ResolvedMovement> {
    let mut output = Vec::new();

    let bitboards = match board.turn {
        Color::Black => board.black_boards(),
        Color::White => board.white_boards(),
    };

    let occupancies = board.black_pieces() | board.white_pieces();

    for (_, piece, bb) in bitboards {
        let mut itr = BitBoardIterator::new(bb);
        while let Some(index) = itr.next() {
            let move_board = match piece {
                Piece::Knight => knight_attacks(bb!(index)),
                Piece::King => king_attacks(bb!(index)),
                Piece::Bishop => bishop_attacks(bb!(index), occupancies),
                Piece::Queen => queen_attacks(bb!(index), occupancies),
                Piece::Rook => rook_attacks(bb!(index), occupancies),
                Piece::Pawn => pawn_moves(board, bb!(index), occupancies),
            };

            let mut move_itr = BitBoardIterator::new(move_board);
            while let Some(move_index) = move_itr.next() {
                output.push(ResolvedMovement {
                    piece,
                    from: Square::from_usize(index),
                    to: Square::from_usize(move_index),
                    promotion: None,
                    capture: match board.get_piece_at(&bb!(move_index)) {
                        Some((color, piece)) => {
                            if color == board.turn.opposite() {
                                Some(piece)
                            } else {
                                None
                            }
                        }
                        None => None,
                    },
                });
            }
        }
    }

    if board.turn == Color::White && board.white_castling_kings_side {
        output.push(ResolvedMovement {
            piece: Piece::King,
            from: Square::E1,
            to: Square::G1,
            capture: None,
            promotion: None,
        });
    }

    if board.turn == Color::White && board.white_castling_queen_side {
        output.push(ResolvedMovement {
            piece: Piece::King,
            from: Square::E1,
            to: Square::C1,
            capture: None,
            promotion: None,
        });
    }

    if board.turn == Color::Black && board.black_castling_kings_side {
        output.push(ResolvedMovement {
            piece: Piece::King,
            from: Square::E8,
            to: Square::G8,
            capture: None,
            promotion: None,
        });
    }

    if board.turn == Color::Black && board.black_castling_queen_side {
        output.push(ResolvedMovement {
            piece: Piece::King,
            from: Square::E8,
            to: Square::C8,
            capture: None,
            promotion: None,
        });
    }
    if let Some(square) = board.en_passant {
        let left_attack = match board.turn {
            Color::White => bb!(square) >> 7 & !FILE_BITBOARDS[7] & board.white_pieces(),
            Color::Black => bb!(square) << 9 & !FILE_BITBOARDS[7] & board.black_pieces(),
        };

        if left_attack != 0 {
            let (file, rank) = left_attack.file_and_rank();
            output.push(ResolvedMovement {
                piece: Piece::Pawn,
                from: Square::from_file_and_rank(file, rank),
                to: square,
                capture: Some(Piece::Pawn),
                promotion: None,
            });
        }

        let right_attack = match board.turn {
            Color::White => bb!(square) >> 9 & !FILE_BITBOARDS[0] & board.white_pieces(),
            Color::Black => bb!(square) << 7 & !FILE_BITBOARDS[0] & board.black_pieces(),
        };

        if right_attack != 0 {
            let (file, rank) = right_attack.file_and_rank();
            output.push(ResolvedMovement {
                piece: Piece::Pawn,
                from: Square::from_file_and_rank(file, rank),
                to: square,
                capture: Some(Piece::Pawn),
                promotion: None,
            });
        }
    }

    output
}

pub fn is_move_to_check(board: &Board, movement: ResolvedMovement) -> bool {
    let mut new_board = board.clone();
    new_board.move_piece(movement);
    new_board.turn = new_board.turn.opposite();
    is_in_check(&new_board)
}

pub fn is_in_check(board: &Board) -> bool {
    let mut new_board = board.clone();
    let king_board = match new_board.turn {
        Color::White => new_board.white_king_board,
        Color::Black => new_board.black_king_board,
    };

    let (file, rank) = king_board.file_and_rank();
    let king_square = Square::from_file_and_rank(file, rank);

    new_board.turn = new_board.turn.opposite();

    for m in pseudo_moves(&new_board) {
        if m.to == king_square && m.capture == Some(Piece::King) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod check_tests {
    use super::*;

    #[test]
    fn is_in_check_test() {
        // Simple in check
        assert!(is_in_check(
            &Board::from_fen_str("3k4/3q4/8/8/8/8/8/3K4 w - - 0 1").unwrap()
        ));

        // Is in check because it white to move
        assert!(is_in_check(
            &Board::from_fen_str("2k4r/1ppr3p/p3p3/8/5Q2/5n2/Pq3PPP/2R1R1K1 w - - 0 24").unwrap()
        ));
    }

    #[test]
    fn simple_move_to_check() {
        let board = Board::from_fen_str("4K3/3Q4/8/8/8/8/8/4k3 w - - 0 1").unwrap();
        let movement = ResolvedMovement {
            piece: Piece::Queen,
            from: Square::D7,
            to: Square::E7,
            capture: None,
            promotion: None,
        };

        assert!(!is_move_to_check(&board, movement));
    }

    #[test]
    fn pick_the_correct_rook_to_move() {
        let board =
            Board::from_fen_str("2kr4/1pp4p/p3pQ2/2R5/q6P/8/5PP1/3rR1K1 w - - 3 28").unwrap();

        assert!(is_move_to_check(
            &board,
            ResolvedMovement {
                piece: Piece::Rook,
                from: Square::E1,
                to: Square::E5,
                capture: None,
                promotion: None,
            }
        ));

        assert!(!is_move_to_check(
            &board,
            ResolvedMovement {
                piece: Piece::Rook,
                from: Square::C5,
                to: Square::E5,
                capture: None,
                promotion: None,
            }
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Creates a bitboard from a string. The string should be 8x8 and contain only `x` and `.`.
    /// Any square with an `x` will be set to 1, and any square with a `.` will be set to 0. All
    /// white space will be ignored so it can be in any format.
    ///
    /// ```
    /// board(concat!(
    ///     " . . x . . . . . ",
    ///     " . . x . . . . . ",
    ///     " x x . x x x x x ",
    ///     " . . x . . . . . ",
    ///     " . . x . . . . . ",
    ///     " . . x . . . . . ",
    ///     " . . x . . . . . ",
    ///     " . . x . . . . . ",
    /// ));
    /// ```
    fn board(str: &str) -> BitBoard {
        let mut board = 0;

        let mut squares = str.to_string().to_ascii_lowercase();
        squares.retain(|c| ".x".contains(c));

        for (i, c) in squares.chars().rev().enumerate() {
            if c == 'x' {
                board |= bb!(i);
            }
        }

        board
    }

    #[test]
    fn scans_south() {
        assert_eq!(
            scan_rook(bb!(0), bb!(Square::D3), SOUTH),
            board(concat!(
                " . . . . . . . . ",
                " . . . . . . . . ",
                " . . . . . . . . ",
                " . . . . . . . . ",
                " . . . . . . . . ",
                " . . . x . . . . ",
                " . . . x . . . . ",
                " . . . x . . . . ",
            ))
        )
    }

    macro_rules! assert_rook_attacks {
        ($piece_board:expr, $blockers:expr, $expected:expr) => {
            assert_eq!(
                rook_attacks($piece_board, board($blockers)),
                board($expected)
            );
        };
    }

    #[test]
    fn will_calculate_rook_moves_with_no_blockers() {
        assert_rook_attacks!(
            bb!(Square::C6),
            "",
            concat!(
                " . . x . . . . . ",
                " . . x . . . . . ",
                " x x . x x x x x ",
                " . . x . . . . . ",
                " . . x . . . . . ",
                " . . x . . . . . ",
                " . . x . . . . . ",
                " . . x . . . . . ",
            )
        );
    }

    #[test]
    fn rook_on_another_edge() {
        assert_rook_attacks!(
            bb!(Square::H5),
            "",
            concat!(
                " . . . . . . . x ",
                " . . . . . . . x ",
                " . . . . . . . x ",
                " x x x x x x x . ",
                " . . . . . . . x ",
                " . . . . . . . x ",
                " . . . . . . . x ",
                " . . . . . . . x ",
            )
        );
    }

    #[test]
    fn rook_on_yet_another_edge() {
        assert_rook_attacks!(
            bb!(Square::E1),
            concat!(
                " . . . . . . . . ",
                " . . . . . . . . ",
                " . . . . . . . . ",
                " . . . . . . . . ",
                " . . . . . . . . ",
                " . . . . . . . . ",
                " . . . . . . . . ",
                " . . . x . . . . ",
            ),
            concat!(
                " . . . . x . . . ",
                " . . . . x . . . ",
                " . . . . x . . . ",
                " . . . . x . . . ",
                " . . . . x . . . ",
                " . . . . x . . . ",
                " . . . . x . . . ",
                " . . . x . x x x ",
            )
        );
    }

    #[test]
    fn rook_on_the_edge() {
        assert_rook_attacks!(
            bb!(Square::H1),
            "",
            concat!(
                " . . . . . . . x ",
                " . . . . . . . x ",
                " . . . . . . . x ",
                " . . . . . . . x ",
                " . . . . . . . x ",
                " . . . . . . . x ",
                " . . . . . . . x ",
                " x x x x x x x . ",
            )
        );
    }

    #[test]
    fn calculates_rook_moves_with_blockers_no_each_side() {
        assert_rook_attacks!(
            bb!(Square::C6),
            concat!(
                " . . . . . . . . ",
                " . . x . . . . . ",
                " . x . . x . . . ",
                " . . . . . . . . ",
                " . . x . . . . . ",
                " . . . . . . . . ",
                " . . . . . . . . ",
                " . . . . . . . . ",
            ),
            concat!(
                " . . . . . . . . ",
                " . . x . . . . . ",
                " . x . x x . . . ",
                " . . x . . . . . ",
                " . . x . . . . . ",
                " . . . . . . . . ",
                " . . . . . . . . ",
                " . . . . . . . . ",
            )
        );
    }

    macro_rules! assert_bishop_attacks {
        ($piece_board:expr, $blockers:expr, $expected:expr) => {
            assert_eq!(
                bishop_attacks($piece_board, board($blockers)),
                board($expected)
            );
        };
    }

    #[test]
    fn calculates_bishop_attacks_with_no_blockers() {
        assert_bishop_attacks!(
            bb!(Square::C6),
            "",
            concat!(
                " x . . . x . . . ",
                " . x . x . . . . ",
                " . . . . . . . . ",
                " . x . x . . . . ",
                " x . . . x . . . ",
                " . . . . . x . . ",
                " . . . . . . x . ",
                " . . . . . . . x ",
            )
        );
    }

    #[test]
    fn bishop_attacks_on_the_edge() {
        assert_bishop_attacks!(
            bb!(Square::A4),
            "",
            concat!(
                " . . . . x . . . ",
                " . . . x . . . . ",
                " . . x . . . . . ",
                " . x . . . . . . ",
                " . . . . . . . . ",
                " . x . . . . . . ",
                " . . x . . . . . ",
                " . . . x . . . . ",
            )
        );
    }

    #[test]
    fn calculates_bishop_attacks_with_blockers_on_each_side() {
        assert_bishop_attacks!(
            bb!(Square::C6),
            concat!(
                " . . . . . . . . ",
                " . x . x . . . . ",
                " . . . . . . . . ",
                " . x . . . . . . ",
                " . . . . . . . . ",
                " . . . . . x . . ",
                " . . . . . . . . ",
                " . . . . . . . . ",
            ),
            concat!(
                " . . . . . . . . ",
                " . x . x . . . . ",
                " . . . . . . . . ",
                " . x . x . . . . ",
                " . . . . x . . . ",
                " . . . . . x . . ",
                " . . . . . . . . ",
                " . . . . . . . . ",
            )
        );
    }

    #[test]
    fn quick_queen_attack_test() {
        assert_eq!(
            queen_attacks(
                bb!(Square::D6),
                board(concat!(
                    " . x . . . x . . ",
                    " . . . . . . . . ",
                    " . x . . . x . . ",
                    " . . . . . . . . ",
                    " . x . x . x . . ",
                    " . . . . . . . . ",
                    " . . . . . . . . ",
                    " . . . . . . . . ",
                ),)
            ),
            board(concat!(
                " . x . x . x . . ",
                " . . x x x . . . ",
                " . x x . x x . . ",
                " . . x x x . . . ",
                " . x . x . x . . ",
                " . . . . . . . . ",
                " . . . . . . . . ",
                " . . . . . . . . ",
            ))
        );
    }

    #[test]
    fn calculates_knight_attacks_for_a_middle_square() {
        assert_eq!(
            knight_attacks(bb!(Square::D4)),
            board(concat!(
                " . . . . . . . . ",
                " . . . . . . . . ",
                " . . x . x . . . ",
                " . x . . . x . . ",
                " . . . . . . . . ",
                " . x . . . x . . ",
                " . . x . x . . . ",
                " . . . . . . . . ",
            ))
        );
    }

    #[test]
    fn calculates_knight_attacks_on_the_edge() {
        assert_eq!(
            knight_attacks(bb!(Square::B2)),
            board(concat!(
                " . . . . . . . . ",
                " . . . . . . . . ",
                " . . . . . . . . ",
                " . . . . . . . . ",
                " x . x . . . . . ",
                " . . . x . . . . ",
                " . . . . . . . . ",
                " . . . x . . . . ",
            ))
        );
    }

    #[test]
    fn calculates_king_attacks_in_the_middle_of_the_board() {
        assert_eq!(
            king_attacks(bb!(Square::E5)),
            board(concat!(
                " . . . . . . . . ",
                " . . . . . . . . ",
                " . . . x x x . . ",
                " . . . x . x . . ",
                " . . . x x x . . ",
                " . . . . . . . . ",
                " . . . . . . . . ",
                " . . . . . . . . ",
            ))
        );
    }

    #[test]
    fn calculates_king_attacks_on_the_edge_of_the_board() {
        assert_eq!(
            king_attacks(bb!(Square::A5)),
            board(concat!(
                " . . . . . . . . ",
                " . . . . . . . . ",
                " x x . . . . . . ",
                " . x . . . . . . ",
                " x x . . . . . . ",
                " . . . . . . . . ",
                " . . . . . . . . ",
                " . . . . . . . . ",
            ))
        );
    }

    macro_rules! get_moves {
        ($board:expr) => {
            pseudo_moves(&Board::from_fen_str($board).unwrap())
        };
    }

    macro_rules! assert_has_move {
        ($moves:expr, $move:expr) => {
            assert!(
                $moves.clone().into_iter().any(|m| m.uci() == $move),
                "Move {} not found in moves: {:#?}",
                $move,
                $moves
            );
        };
    }

    #[test]
    fn calculates_en_passant() {
        let moves = get_moves!("8/8/8/8/6Pp/8/8/8 b - g3 0 16");
        assert_has_move!(moves, "h4g3");
    }

    #[test]
    fn calculates_en_passant_for_black() {
        let moves = get_moves!("5b1r/1p1kp1p1/1rp1Rp2/3P4/pPPN2P1/6BP/P1N2PK1/8 b - b3 0 28");
        assert_has_move!(moves, "a4b3");
    }

    #[test]
    fn get_rook_moves() {
        let moves = get_moves!("r1P5/8/P7/8/8/8/8/8 b - - 0 1");

        // Captures the pawn
        assert_has_move!(moves, "a8a6");

        // Moves to the left
        assert_has_move!(moves, "a8b8");
    }

    #[test]
    fn white_pawn_captures() {
        let moves = get_moves!("8/8/8/p7/1P6/8/8/8 w - - 0 1");
        assert_has_move!(moves, "b4a5");
        assert_has_move!(moves, "b4b5");
        assert_eq!(moves.len(), 2);
    }

    #[test]
    fn black_pawn_captures() {
        let moves = get_moves!("8/8/8/1p6/P7/8/8/8 b - - 0 1");
        assert_has_move!(moves, "b5a4");
        assert_has_move!(moves, "b5b4");
        assert_eq!(moves.len(), 2);
    }

    #[test]
    fn creates_the_correct_bishop_moves() {
        let moves = get_moves!("8/8/8/8/b7/8/8/8 b - - 0 1");

        let mut target_board = 0;
        for m in moves {
            target_board |= bb!(m.to);
        }

        assert_eq!(
            target_board,
            board(concat!(
                " . . . . x . . . ",
                " . . . x . . . . ",
                " . . x . . . . . ",
                " . x . . . . . . ",
                " . . . . . . . . ",
                " . x . . . . . . ",
                " . . x . . . . . ",
                " . . . x . . . . ",
            ))
        );
    }
}
