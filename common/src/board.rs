use crate::BitBoard;
use crate::Color;
use crate::Fen;
use crate::Piece;

#[derive(Copy, Clone, Debug, Default)]
pub struct Board {
    /// The color that is turn currently to move
    pub turn: Color,

    /// A bit board for each of the white pieces
    pub white_bishop_board: BitBoard,
    pub white_king_board: BitBoard,
    pub white_knight_board: BitBoard,
    pub white_pawn_board: BitBoard,
    pub white_queen_board: BitBoard,
    pub white_rook_board: BitBoard,

    /// A bit board for each of the black pieces
    pub black_bishop_board: BitBoard,
    pub black_king_board: BitBoard,
    pub black_knight_board: BitBoard,
    pub black_pawn_board: BitBoard,
    pub black_queen_board: BitBoard,
    pub black_rook_board: BitBoard,

    pub white_castling_kings_side: bool,
    pub white_castling_queen_side: bool,
    pub black_castling_kings_side: bool,
    pub black_castling_queen_side: bool,

    // An extra bit board for printing markers on the board. This is useful for debugging.
    #[allow(dead_code)]
    markers: BitBoard,
}

impl Board {
    pub fn load_fen(&mut self, fen: &Fen) {
        self.black_rook_board = 0;
        self.black_bishop_board = 0;
        self.black_knight_board = 0;
        self.black_pawn_board = 0;
        self.black_queen_board = 0;
        self.black_king_board = 0;
        self.white_bishop_board = 0;
        self.white_king_board = 0;
        self.white_knight_board = 0;
        self.white_pawn_board = 0;
        self.white_queen_board = 0;
        self.white_rook_board = 0;

        for (index, square) in fen.squares.iter().rev().enumerate() {
            match square {
                Some((Color::White, Piece::King)) => self.white_king_board |= 1 << index,
                Some((Color::White, Piece::Queen)) => self.white_queen_board |= 1 << index,
                Some((Color::White, Piece::Rook)) => self.white_rook_board |= 1 << index,
                Some((Color::White, Piece::Bishop)) => self.white_bishop_board |= 1 << index,
                Some((Color::White, Piece::Knight)) => self.white_knight_board |= 1 << index,
                Some((Color::White, Piece::Pawn)) => self.white_pawn_board |= 1 << index,

                Some((Color::Black, Piece::King)) => self.black_king_board |= 1 << index,
                Some((Color::Black, Piece::Queen)) => self.black_queen_board |= 1 << index,
                Some((Color::Black, Piece::Rook)) => self.black_rook_board |= 1 << index,
                Some((Color::Black, Piece::Bishop)) => self.black_bishop_board |= 1 << index,
                Some((Color::Black, Piece::Knight)) => self.black_knight_board |= 1 << index,
                Some((Color::Black, Piece::Pawn)) => self.black_pawn_board |= 1 << index,

                // Skip over empty squares
                None => (),
            }
        }

        self.turn = fen.turn;
    }

    pub fn from_fen_str(fen: &str) -> Result<Self, String> {
        let mut board = Self::default();
        board.load_fen(&Fen::from_str(&fen)?);

        Ok(board)
    }

    pub fn white_pieces(&self) -> BitBoard {
        self.white_bishop_board
            | self.white_king_board
            | self.white_knight_board
            | self.white_pawn_board
            | self.white_queen_board
            | self.white_rook_board
    }

    pub fn black_pieces(&self) -> BitBoard {
        self.black_bishop_board
            | self.black_king_board
            | self.black_knight_board
            | self.black_pawn_board
            | self.black_queen_board
            | self.black_rook_board
    }

    pub fn black_boards(&self) -> Vec<(Color, Piece, BitBoard)> {
        vec![
            (Color::Black, Piece::Queen, self.black_queen_board),
            (Color::Black, Piece::Rook, self.black_rook_board),
            (Color::Black, Piece::Bishop, self.black_bishop_board),
            (Color::Black, Piece::Knight, self.black_knight_board),
            (Color::Black, Piece::Pawn, self.black_pawn_board),
            (Color::Black, Piece::King, self.black_king_board),
        ]
    }

    pub fn white_boards(&self) -> Vec<(Color, Piece, BitBoard)> {
        vec![
            (Color::White, Piece::Queen, self.white_queen_board),
            (Color::White, Piece::Rook, self.white_rook_board),
            (Color::White, Piece::Bishop, self.white_bishop_board),
            (Color::White, Piece::Knight, self.white_knight_board),
            (Color::White, Piece::Pawn, self.white_pawn_board),
            (Color::White, Piece::King, self.white_king_board),
        ]
    }

    pub fn get_piece_at(&self, source_board: &BitBoard) -> Option<(Color, Piece)> {
        let bitboards = vec![self.black_boards(), self.white_boards()].concat();
        for (color, piece, target_board) in bitboards {
            if source_board & target_board != 0 {
                return Some((color, piece));
            }
        }

        None
    }

    pub fn print(&self) {
        const LAST_BIT: u64 = 63;

        println!("     a  b  c  d  e  f  g  h");
        println!("    ────────────────────────");

        for rank in 0..8 {
            print!("{} │", 8 - rank);
            for file in 0..8 {
                let mask = 1u64 << (LAST_BIT - (rank * 8) - file);
                if self.markers & mask != 0 {
                    print!(" x ")
                } else if self.black_rook_board & mask != 0 {
                    print!(" r ")
                } else if self.black_king_board & mask != 0 {
                    print!(" k ")
                } else if self.black_knight_board & mask != 0 {
                    print!(" n ")
                } else if self.black_bishop_board & mask != 0 {
                    print!(" b ")
                } else if self.black_queen_board & mask != 0 {
                    print!(" q ")
                } else if self.black_pawn_board & mask != 0 {
                    print!(" p ")
                } else if self.white_rook_board & mask != 0 {
                    print!(" R ")
                } else if self.white_king_board & mask != 0 {
                    print!(" K ")
                } else if self.white_knight_board & mask != 0 {
                    print!(" N ")
                } else if self.white_bishop_board & mask != 0 {
                    print!(" B ")
                } else if self.white_queen_board & mask != 0 {
                    print!(" Q ")
                } else if self.white_pawn_board & mask != 0 {
                    print!(" P ")
                } else {
                    print!(" . ")
                }
            }

            println!(" │ {}", 8 - rank);
        }

        println!("    ────────────────────────");
        println!("     a  b  c  d  e  f  g  h");

        println!();
        println!("Its {} to move", self.turn);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bb;
    use crate::Square;

    #[test]
    fn creates_a_default_empty_board() {
        let board = Board::default();
        assert_eq!(board.turn, Color::White);
    }

    #[test]
    fn loads_in_a_fen() {
        let board = Board::from_fen_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .unwrap();

        assert_eq!(board.turn, Color::White);
        assert_eq!(board.white_queen_board, bb!(Square::D1));
    }

    #[test]
    fn loads_in_the_correct_order() {
        let board = Board::from_fen_str("8/1p2k3/8/8/1R2K3/8/1p6/8 w - - 0 1").unwrap();

        assert_eq!(board.turn, Color::White);

        assert_eq!(board.white_rook_board, bb!(Square::B4));
        assert_eq!(board.white_king_board, bb!(Square::E4));

        assert_eq!(board.black_king_board, bb!(Square::E7));
        assert_eq!(board.black_pawn_board, bb!(Square::B2) | bb!(Square::B7));
    }
}
