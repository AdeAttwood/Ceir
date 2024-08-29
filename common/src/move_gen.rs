use crate::lookup::FILE_BITBOARDS;
use crate::lookup::RANK_BITBOARDS;
use crate::{bb, BitBoard, BitBoardIterator, Board, Color, Piece, ResolvedMovement, Square};

const NORTH: i8 = 8;
const EAST: i8 = 1;
const SOUTH: i8 = -8;
const WEST: i8 = -1;
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
        // Perform the shift
        i = if direction > 0 {
            i << direction
        } else {
            i >> -direction
        };

        // Stop if the shift goes out of bounds
        if i == 0 {
            break;
        }

        ray_board |= i;

        // Check if we are on an invalid edge, preventing wrap around
        if (direction == NORTH_EAST && i & FILE_BITBOARDS[0] != 0)
            || (direction == NORTH_WEST && i & FILE_BITBOARDS[7] != 0)
            || (direction == SOUTH_WEST && i & FILE_BITBOARDS[7] != 0)
            || (direction == SOUTH_EAST && i & FILE_BITBOARDS[0] != 0)
        {
            break;
        }

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
        i = if direction > 0 {
            i << direction // Positive shift (east/north)
        } else {
            i >> -direction // Negative shift (west/south)
        };

        // Stop if the shift goes out of bounds
        if i == 0 {
            break;
        }

        ray_board |= i; // Include the current square in the ray

        // Stop if we hit the edge of the board
        if (direction == EAST && i & FILE_BITBOARDS[0] != 0)
            || (direction == WEST && i & FILE_BITBOARDS[7] != 0)
            || (direction == NORTH && i & RANK_BITBOARDS[7] != 0)
            || (direction == SOUTH && i & RANK_BITBOARDS[0] != 0)
        {
            break;
        }

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
        Color::Black => piece_board >> 9 & board.white_pieces() & !FILE_BITBOARDS[7],
        Color::White => piece_board << 9 & board.black_pieces() & !FILE_BITBOARDS[0],
    };

    let right_attack = match board.turn {
        Color::Black => piece_board >> 7 & board.white_pieces() & !FILE_BITBOARDS[0],
        Color::White => piece_board << 7 & board.black_pieces() & !FILE_BITBOARDS[7],
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
                    capture: None,
                    promotion: None,
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

    output
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
}
