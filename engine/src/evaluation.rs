use common::{BitBoardIterator, Board, Color, Piece};

#[rustfmt::skip]
const PAWN_SCORE: [i32; 64] = [
    0,  0,  0,   0,   0,   0,   0,  0,
    50, 50, 50,  50,  50,  50,  50, 50,
    10, 10, 20,  30,  30,  20,  10, 10,
    5,  5,  10,  25,  25,  10,  5,  5,
    0,  0,  0,   20,  20,  0,   0,  0,
    5,  -5, -10, 0,   0,   -10, -5, 5,
    5,  10, 10,  -20, -20, 10,  10, 5,
    0,  0,  0,   0,   0,   0,   0,  0
];

#[rustfmt::skip]
const KNIGHT_SCORE: [i32; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

#[rustfmt::skip]
const BISHOP_SCORE: [i32; 64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20,
];

#[rustfmt::skip]
const ROOK_SCORE: [i32; 64] = [
     0,  0,  0,  0,  0,  0,  0,  0,
     5, 10, 10, 10, 10, 10, 10,  5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
     0,  0,  0,  5,  5,  0,  0,  0
];

#[rustfmt::skip]
const QUEEN_SCORE: [i32; 64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
     -5,  0,  5,  5,  5,  5,  0, -5,
      0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20
];

#[rustfmt::skip]
const KING_SCORE: [i32; 64] = [
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
     20, 20,  0,  0,  0,  0, 20, 20,
     20, 30, 10,  0,  0, 10, 30, 20
];

fn material_score(board: &Board) -> i32 {
    let white_pieces = (board.white_pawn_board.count_ones() as i32 * 100)
        + (board.white_queen_board.count_ones() as i32 * 9 * 100)
        + (board.white_rook_board.count_ones() as i32 * 5 * 100)
        + (board.white_bishop_board.count_ones() as i32 * 3 * 100)
        + (board.white_knight_board.count_ones() as i32 * 3 * 100);

    let black_pieces = (board.black_pawn_board.count_ones() as i32 * 100)
        + (board.black_queen_board.count_ones() as i32 * 9 * 100)
        + (board.black_rook_board.count_ones() as i32 * 5 * 100)
        + (board.black_bishop_board.count_ones() as i32 * 3 * 100)
        + (board.black_knight_board.count_ones() as i32 * 3 * 100);

    let offset = match board.turn {
        Color::White => 1,
        Color::Black => -1,
    };

    (white_pieces - black_pieces) * offset
}

pub fn evaluate(board: &Board) -> i32 {
    let mut score = material_score(board);

    for (_, piece, bitboard) in board.white_boards() {
        let mut it = BitBoardIterator::new(bitboard.reverse_bits());
        while let Some(index) = it.next() {
            score += match piece {
                Piece::Pawn => PAWN_SCORE[index],
                Piece::Rook => ROOK_SCORE[index],
                Piece::Queen => QUEEN_SCORE[index],
                Piece::Bishop => BISHOP_SCORE[index],
                Piece::Knight => KNIGHT_SCORE[index],
                Piece::King => KING_SCORE[index],
            }
        }
    }

    for (_, piece, bitboard) in board.black_boards() {
        let mut it = BitBoardIterator::new(bitboard);
        while let Some(index) = it.next() {
            score -= match piece {
                Piece::Pawn => PAWN_SCORE[index],
                Piece::Rook => ROOK_SCORE[index],
                Piece::Queen => QUEEN_SCORE[index],
                Piece::Bishop => BISHOP_SCORE[index],
                Piece::Knight => KNIGHT_SCORE[index],
                Piece::King => KING_SCORE[index],
            }
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use common::{bb, BitBoard, Square};

    use super::*;

    macro_rules! bb_index {
        ($square:expr) => {
            BitBoardIterator::new((bb!($square) as BitBoard).reverse_bits())
                .next()
                .unwrap()
        };
    }

    #[test]
    fn it_indexes_the_array_correctly() {
        let index = bb_index!(Square::C4);

        assert_eq!(0, PAWN_SCORE[bb_index!(Square::C4)]);
        assert_eq!(20, PAWN_SCORE[bb_index!(Square::D4)]);
        assert_eq!(20, PAWN_SCORE[bb_index!(Square::E4)]);
        assert_eq!(0, PAWN_SCORE[bb_index!(Square::F4)]);
    }

    #[test]
    fn will_eval_the_position() {
        assert_eq!(
            40,
            evaluate(&Board::from_fen_str("8/3p4/8/8/3P4/8/8/8 w - - 0 1").unwrap())
        );
        assert_eq!(
            0,
            evaluate(&Board::from_fen_str("8/3p4/8/8/8/8/3P4/8 w - - 0 1").unwrap())
        );
    }
}
