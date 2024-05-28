use crate::lookup::EDGES;
use crate::{bb, BitBoard, BitBoardIterator, Board, Color, Movement, Piece, Square};

/// Scan a board in a direction until we hit a blocker or we hit the edge of the board. This will
/// calculate the ray attack for a sliding piece that will take blocker into account. This will
/// only scan in a positive direction for example "north", "east", "north east" and "north west".
/// You are unable to shift by a negative number. NOTE: This will also include any blockers as
/// valid moves so you can calculate any potential captures.
///
/// https://www.chessprogramming.org/On_an_empty_Board#PositiveRays
fn scan_positive(occupancies: BitBoard, square_board: BitBoard, direction: i8) -> BitBoard {
    let mut ray_board = square_board;
    let mut i = square_board >> direction;

    loop {
        ray_board |= i;

        if i & occupancies != 0 || i & EDGES != 0 {
            break;
        }

        i = i >> direction;
    }

    ray_board
}

/// Scan negative arrays. Rather than duplicate the code this reverses the bits then scans
/// positive. When we are done are reverse the bits again to give us the negative ray. Again this
/// will stop at any blockers.
fn scan_negative(occupancies: BitBoard, square_board: BitBoard, direction: i8) -> BitBoard {
    scan_positive(
        occupancies.reverse_bits(),
        square_board.reverse_bits(),
        direction,
    )
    .reverse_bits()
}

/// Calculates all of the available attacks that a rook can make given that its on a given square.
pub fn rook_attacks(square_board: BitBoard, occupancies: BitBoard) -> BitBoard {
    let line_attacks = scan_positive(occupancies, square_board, 1)
        | scan_positive(occupancies, square_board, 8)
        | scan_negative(occupancies, square_board, 1)
        | scan_negative(occupancies, square_board, 8);

    line_attacks & !square_board
}

/// Calculates all of the available attacks that a bishop can make given that its on a given
/// square.
pub fn bishop_attacks(square_board: BitBoard, occupancies: BitBoard) -> BitBoard {
    let line_attacks = scan_positive(occupancies, square_board, 7)
        | scan_positive(occupancies, square_board, 9)
        | scan_negative(occupancies, square_board, 7)
        | scan_negative(occupancies, square_board, 9);

    line_attacks & !square_board
}

pub fn pseudo_moves(board: &Board) -> Vec<Movement> {
    let mut output = Vec::new();

    let bitboards = match board.turn {
        Color::Black => board.black_boards(),
        Color::White => board.white_boards(),
    };

    let available_squares = !match board.turn {
        Color::Black => board.black_pieces(),
        Color::White => board.white_pieces(),
    };

    let occupancies = board.black_pieces() | board.white_pieces();

    for (_, piece, board) in bitboards {
        let mut itr = BitBoardIterator::new(board);
        while let Some(index) = itr.next() {
            let move_board = match piece {
                // Piece::Knight => self.knight_moves(index),
                // Piece::King => self.king_moves(index),
                Piece::Bishop => bishop_attacks(bb!(index), occupancies),
                // Piece::Queen => self.queen_moves(index),
                Piece::Rook => rook_attacks(bb!(index), occupancies),
                // Piece::Pawn => self.pawn_moves(self.board.turn, index),
                _ => 0,
            };

            let mut move_itr = BitBoardIterator::new(move_board & available_squares);
            while let Some(move_index) = move_itr.next() {
                output.push(Movement {
                    piece,
                    from: Some(Square::from_usize(index)),
                    to: Square::from_usize(move_index),
                    capture: None,
                    promotion: None,
                });
            }
        }
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
}
