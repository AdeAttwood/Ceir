use crate::bb;
use crate::Board;
use crate::Color;
use crate::Piece;

use crate::BitBoardable;

/// Zobrist hashing function
pub fn hash_board(board: &Board) -> Result<u64, String> {
    let mut hash: u64 = 0;

    for square in 0..64 {
        let square_board = bb!(square);
        if let Some(board_square) = board.get_piece_at(&square_board) {
            let piece = match board_square {
                (Color::Black, Piece::Pawn) => 0,
                (Color::White, Piece::Pawn) => 1,
                (Color::Black, Piece::Knight) => 2,
                (Color::White, Piece::Knight) => 3,
                (Color::Black, Piece::Bishop) => 4,
                (Color::White, Piece::Bishop) => 5,
                (Color::Black, Piece::Rook) => 6,
                (Color::White, Piece::Rook) => 7,
                (Color::Black, Piece::Queen) => 8,
                (Color::White, Piece::Queen) => 9,
                (Color::Black, Piece::King) => 10,
                (Color::White, Piece::King) => 11,
            };

            let (file, row) = square_board.file_and_rank();
            hash ^= crate::random::RANDOM_PIECE[64 * piece + 8 * row + file];
        }
    }

    if board.white_castling_kings_side {
        hash ^= crate::random::RANDOM_CASTLE[0];
    }

    if board.white_castling_queen_side {
        hash ^= crate::random::RANDOM_CASTLE[1];
    }

    if board.black_castling_kings_side {
        hash ^= crate::random::RANDOM_CASTLE[2];
    }

    if board.black_castling_queen_side {
        hash ^= crate::random::RANDOM_CASTLE[3];
    }

    if let Some(en_passant) = board.en_passant {
        let (file, _) = bb!(en_passant).file_and_rank();
        hash ^= crate::random::RANDOM_ENPASSANT[file];
    }

    if board.turn == Color::White {
        hash ^= crate::random::RANDOM_TURN;
    }

    Ok(hash)
}

#[cfg(test)]
mod tests {
    use crate::Board;

    #[test]
    fn hashes_a_board() {
        let positions: Vec<(&str, u64)> = vec![
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                0x463b96181691fc9c,
            ),
            (
                "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1",
                0x823c9b50fd114196,
            ),
            (
                "rnbqkbnr/1pppp1pp/p7/4Pp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6 0 3",
                0x753958bdf5b34982,
            ),
        ];

        for (fen, hash) in positions {
            let board = Board::from_fen_str(fen).unwrap();
            assert_eq!(hash, super::hash_board(&board).unwrap());
        }
    }
}
