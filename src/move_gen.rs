use crate::bitboards::{
    BitBoard, BitBoardIterator, ANTI_DIAGONAL_BIT_BOARDS, DIAGONAL_BIT_BOARDS, FILE_BIT_BOARDS,
    RANK_BIT_BOARDS, SQUARE_LOOKUP,
};
use crate::board::{Board, Color, Piece};
use crate::moves::Move;

#[rustfmt::skip]
pub const MVV_LVA: [[i32; 6]; 6] = [
    [0, 0, 0, 0, 0, 0],       // victim K, attacker K, Q, R, B, N, P
    [50, 51, 52, 53, 54, 55], // victim Q, attacker K, Q, R, B, N, P
    [40, 41, 42, 43, 44, 45], // victim R, attacker K, Q, R, B, N, P
    [30, 31, 32, 33, 34, 35], // victim B, attacker K, Q, R, B, N, P
    [20, 21, 22, 23, 24, 25], // victim N, attacker K, Q, R, B, N, P
    [10, 11, 12, 13, 14, 15], // victim P, attacker K, Q, R, B, N, P
];

pub struct MoveGen<'a> {
    pub board: &'a Board,
    pub pseudo_moves: Vec<Move>,
    pub is_check: bool,
    pub checking: bool,
}

impl MoveGen<'_> {
    pub fn new(board: &Board) -> MoveGen {
        let mut move_gen = MoveGen {
            board,
            pseudo_moves: Vec::new(),
            is_check: false,
            checking: false,
        };
        move_gen.init();

        move_gen
    }

    /// Gets the line attack for a given position mask, the mask is a bitboard representing a rank
    /// or file on the board.
    ///
    /// positiveRayAttacks = o  ^ (o - 2s);
    /// negativeRayAttacks = reverse (o' ^ (o' - 2s'));
    ///
    /// lineAttacks = (o-2s) ^ reverse( o'-2s')
    ///
    /// See:https://www.chessprogramming.org/Efficient_Generation_of_Sliding_Piece_Attacks#Sliding_Attacks_by_Calculation
    fn line_attack(&self, square_board: BitBoard, mask: BitBoard) -> BitBoard {
        let occupied = self.board.occupied() & mask;

        let positive_rays = occupied - (2 * square_board);
        let negative_rays =
            (occupied.reverse_bits() - (2 * square_board.reverse_bits())).reverse_bits();

        (positive_rays ^ negative_rays) & mask
    }

    fn rook_moves(&self, index: usize) -> BitBoard {
        let from_file = index / 8;
        let from_rank = index % 8;
        let file_bit_board = FILE_BIT_BOARDS[from_rank];
        let rank_bit_board = RANK_BIT_BOARDS[from_file];

        let square_board = 1 << index;

        self.line_attack(square_board, file_bit_board)
            | self.line_attack(square_board, rank_bit_board)
    }

    fn bishop_moves(&self, index: usize) -> BitBoard {
        let file = index / 8;
        let rank = index % 8;

        // Because we have our bitboards reversed we need to calculate the diagonal and anti
        // diagonal index in reverse. Usually to get the anti diagonal you would do `rank +
        // file` however for us its `7 + rank - file`
        //
        // See: https://www.chessprogramming.org/Diagonals
        // See: https://www.chessprogramming.org/Anti-Diagonals
        let diagonal_mask = DIAGONAL_BIT_BOARDS[rank + file];
        let anti_diagonal_mask = ANTI_DIAGONAL_BIT_BOARDS[7 + rank - file];

        let square_board = 1 << index;
        self.line_attack(square_board, anti_diagonal_mask)
            | self.line_attack(square_board, diagonal_mask)
    }

    fn queen_moves(&self, index: usize) -> BitBoard {
        self.rook_moves(index) | self.bishop_moves(index)
    }

    fn knight_moves(&self, index: usize) -> BitBoard {
        let bb = 1 << index;
        // Create the knight_moves from there move position. This may need to be moved into a
        // lookup of pre built bitboards like the rook and bishop attacks.
        //
        // https://www.chessprogramming.org/Knight_Pattern
        (bb << 6 & !(FILE_BIT_BOARDS[7] | FILE_BIT_BOARDS[6]))
            | (bb << 15 & !FILE_BIT_BOARDS[7])
            | (bb << 17 & !FILE_BIT_BOARDS[0])
            | (bb << 10 & !(FILE_BIT_BOARDS[0] | FILE_BIT_BOARDS[1]))
            | (bb >> 6 & !(FILE_BIT_BOARDS[0] | FILE_BIT_BOARDS[1]))
            | (bb >> 15 & !FILE_BIT_BOARDS[0])
            | (bb >> 17 & !FILE_BIT_BOARDS[7])
            | (bb >> 10 & !(FILE_BIT_BOARDS[7] | FILE_BIT_BOARDS[6]))
    }

    fn pawn_moves(&self, turn: Color, index: usize) -> BitBoard {
        let piece_board = 1 << index;

        let one_square = match turn {
            Color::Black => piece_board >> 8 & !self.board.occupied(),
            Color::White => piece_board << 8 & !self.board.occupied(),
        };

        let two_square = match turn {
            Color::Black => one_square >> 8 & !self.board.occupied() & RANK_BIT_BOARDS[4],
            Color::White => one_square << 8 & !self.board.occupied() & RANK_BIT_BOARDS[3],
        };

        let left_attack = match turn {
            Color::Black => piece_board >> 9 & self.board.white_pieces() & !FILE_BIT_BOARDS[7],
            Color::White => piece_board << 9 & self.board.black_pieces() & !FILE_BIT_BOARDS[0],
        };

        let right_attack = match turn {
            Color::Black => piece_board >> 7 & self.board.white_pieces() & !FILE_BIT_BOARDS[0],
            Color::White => piece_board << 7 & self.board.black_pieces() & !FILE_BIT_BOARDS[7],
        };

        one_square | two_square | left_attack | right_attack
    }

    fn king_moves(&self, index: usize) -> BitBoard {
        let bb = 1 << index;
        (bb << 7 & !FILE_BIT_BOARDS[7])
            | bb << 8
            | (bb << 9 & !FILE_BIT_BOARDS[0])
            | (bb << 1 & !FILE_BIT_BOARDS[0])
            | (bb >> 1 & !FILE_BIT_BOARDS[7])
            | (bb >> 9 & !FILE_BIT_BOARDS[7])
            | bb >> 8
            | (bb >> 7 & !FILE_BIT_BOARDS[0])
    }

    fn black_boards(&self) -> Vec<(Color, Piece, BitBoard)> {
        vec![
            (Color::Black, Piece::Rook, self.board.black_rook_board),
            (Color::Black, Piece::Bishop, self.board.black_bishop_board),
            (Color::Black, Piece::Knight, self.board.black_knight_board),
            (Color::Black, Piece::Pawn, self.board.black_pawn_board),
            (Color::Black, Piece::Queen, self.board.black_queen_board),
            (Color::Black, Piece::King, self.board.black_king_board),
        ]
    }

    fn white_boards(&self) -> Vec<(Color, Piece, BitBoard)> {
        vec![
            (Color::White, Piece::Rook, self.board.white_rook_board),
            (Color::White, Piece::Bishop, self.board.white_bishop_board),
            (Color::White, Piece::Knight, self.board.white_knight_board),
            (Color::White, Piece::Pawn, self.board.white_pawn_board),
            (Color::White, Piece::Queen, self.board.white_queen_board),
            (Color::White, Piece::King, self.board.white_king_board),
        ]
    }

    fn get_capture(&self, source_board: BitBoard) -> Option<Piece> {
        let bitboards = match self.board.turn.opposite() {
            Color::Black => self.black_boards(),
            Color::White => self.white_boards(),
        };

        for (_, piece, target_board) in bitboards {
            if source_board & target_board != 0 {
                return Some(piece);
            }
        }

        None
    }

    fn init(&mut self) {
        let bitboards = match self.board.turn {
            Color::Black => self.black_boards(),
            Color::White => self.white_boards(),
        };

        let available_squares = !match self.board.turn {
            Color::Black => self.board.black_pieces(),
            Color::White => self.board.white_pieces(),
        };

        for (_, piece, board) in bitboards {
            let mut itr = BitBoardIterator::new(board);
            while let Some(index) = itr.next() {
                let move_board = match piece {
                    Piece::Knight => self.knight_moves(index),
                    Piece::King => self.king_moves(index),
                    Piece::Bishop => self.bishop_moves(index),
                    Piece::Queen => self.queen_moves(index),
                    Piece::Rook => self.rook_moves(index),
                    Piece::Pawn => self.pawn_moves(self.board.turn, index),
                };

                let mut move_itr = BitBoardIterator::new(move_board & available_squares);
                while let Some(move_index) = move_itr.next() {
                    let capture = self.get_capture(1 << move_index);
                    if let Some(piece) = capture {
                        if piece == Piece::King {
                            self.checking = true;
                        }
                    }

                    self.pseudo_moves.push(Move {
                        piece,
                        from: SQUARE_LOOKUP[index],
                        to: SQUARE_LOOKUP[move_index],
                        capture,
                    });
                }
            }
        }

        self.pseudo_moves.sort_by_key(|m| match m.capture {
            None => 0,
            Some(victim) => -MVV_LVA[victim as usize][m.piece as usize],
        });

        let king = match self.board.turn {
            Color::Black => self.board.black_king_board,
            Color::White => self.board.white_king_board,
        };

        let attackers = match self.board.turn {
            Color::Black => self.white_boards(),
            Color::White => self.black_boards(),
        };

        for (_, piece, board) in attackers {
            let mut itr = BitBoardIterator::new(board);
            while let Some(index) = itr.next() {
                let move_board = match piece {
                    Piece::Knight => self.knight_moves(index),
                    Piece::King => self.king_moves(index),
                    Piece::Bishop => self.bishop_moves(index),
                    Piece::Queen => self.queen_moves(index),
                    Piece::Rook => self.rook_moves(index),
                    Piece::Pawn => self.pawn_moves(self.board.turn, index),
                };

                if king & move_board != 0 {
                    self.is_check = true;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fen::Fen;

    fn get_moves(fen: &str) -> Vec<Move> {
        let mut board = Board::empty();
        let fen = Fen::from_str(&fen).unwrap();
        board.load_fen(&fen);

        let move_gen = MoveGen::new(&board);
        move_gen.pseudo_moves
    }

    #[test]
    fn rook_will_not_panic_and_stop() {
        let moves = get_moves("r1P5/8/P7/8/8/8/8/8 b - - 0 1");
        assert_eq!(moves.len(), 4);
    }

    #[test]
    fn knight_wont_go_off_the_board() {
        let moves = get_moves("8/8/8/8/8/N7/8/8 w - - 0 1");
        assert_eq!(moves.len(), 4);
    }

    #[test]
    fn king_wont_go_off_the_board() {
        let moves = get_moves("8/8/8/8/K7/8/8/8 w - - 0 1");
        assert_eq!(moves.len(), 5);
    }

    #[test]
    fn king_still_wont_go_off_the_board() {
        let moves = get_moves("8/8/8/8/8/8/K7/8 w - - 0 1");
        assert_eq!(moves.len(), 5);
    }

    #[test]
    fn is_in_check() {
        let board = Board::from_fen_str("3k4/3q4/8/8/8/8/8/3K4 w - - 0 1").unwrap();
        let move_gen = MoveGen::new(&board);

        assert!(move_gen.is_check);
    }

    #[test]
    fn this_is_not_check() {
        let board =
            Board::from_fen_str("rnbqkbnr/pppppppp/8/8/8/P7/1PPPPPPP/RNBQKBNR b KQkq - 0 1")
                .unwrap();

        let move_gen = MoveGen::new(&board);

        assert!(!move_gen.is_check);
    }
}
