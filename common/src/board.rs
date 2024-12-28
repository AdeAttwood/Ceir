use crate::bb;
use crate::hasher;
use crate::BitBoard;
use crate::BitBoardable;
use crate::Color;
use crate::Fen;
use crate::Piece;
use crate::ResolvedMovement;
use crate::Square;

#[derive(Copy, Clone, Debug, Default)]
pub struct Board {
    /// The color that is turn currently to move
    pub turn: Color,

    /// The square that can be attacked by en passant
    pub en_passant: Option<Square>,

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

        self.white_castling_kings_side = fen.white_castling_kings_side;
        self.white_castling_queen_side = fen.white_castling_queen_side;
        self.black_castling_kings_side = fen.black_castling_kings_side;
        self.black_castling_queen_side = fen.black_castling_queen_side;

        self.en_passant = fen.en_passant;

        self.turn = fen.turn;
    }

    pub fn from_fen_str(fen: &str) -> Result<Self, String> {
        let mut board = Self::default();
        board.load_fen(&Fen::from_str(&fen)?);

        Ok(board)
    }

    pub fn from_start_position() -> Result<Self, String> {
        let mut board = Self::default();
        board.load_fen(&Fen::from_start_position()?);

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
        let bb = vec![self.black_boards(), self.white_boards()].concat();
        for (color, piece, target_board) in bb {
            if source_board & target_board != 0 {
                return Some((color, piece));
            }
        }

        None
    }

    fn color_board(&mut self, piece: &Piece, color: Color) -> &mut BitBoard {
        match piece {
            Piece::Pawn if color == Color::White => &mut self.white_pawn_board,
            Piece::Pawn if color == Color::Black => &mut self.black_pawn_board,
            Piece::Rook if color == Color::White => &mut self.white_rook_board,
            Piece::Rook if color == Color::Black => &mut self.black_rook_board,
            Piece::Bishop if color == Color::White => &mut self.white_bishop_board,
            Piece::Bishop if color == Color::Black => &mut self.black_bishop_board,
            Piece::Knight if color == Color::White => &mut self.white_knight_board,
            Piece::Knight if color == Color::Black => &mut self.black_knight_board,
            Piece::Queen if color == Color::White => &mut self.white_queen_board,
            Piece::Queen if color == Color::Black => &mut self.black_queen_board,
            Piece::King if color == Color::White => &mut self.white_king_board,
            Piece::King if color == Color::Black => &mut self.black_king_board,
            _ => unreachable!("There are only black and white pieces, what did I miss?"),
        }
    }

    pub fn move_piece(&mut self, movement: ResolvedMovement) {
        let board = self.color_board(&movement.piece, self.turn);
        *board &= !bb!(movement.from);
        *board |= bb!(movement.to);

        if movement.piece == Piece::Pawn && Some(movement.to) == self.en_passant {
            if let Some(en_passant) = self.en_passant {
                let capture_square = match self.turn {
                    Color::White => bb!(en_passant) >> 8,
                    Color::Black => bb!(en_passant) << 8,
                };

                let capture_board = self.color_board(&Piece::Pawn, self.turn.opposite());
                *capture_board &= !capture_square;
            }
        }

        if let Some(capture) = movement.capture {
            let capture_board = self.color_board(&capture, self.turn.opposite());
            *capture_board &= !bb!(movement.to);
        }

        if movement.is_white_king_castle() {
            self.white_rook_board &= !bb!(Square::H1);
            self.white_rook_board |= bb!(Square::F1);
        }

        if movement.is_white_queen_castle() {
            self.white_rook_board &= !bb!(Square::A1);
            self.white_rook_board |= bb!(Square::D1);
        }

        if movement.is_black_king_castle() {
            self.black_rook_board &= !bb!(Square::H8);
            self.black_rook_board |= bb!(Square::F8);
        }

        if movement.is_black_queen_castle() {
            self.black_rook_board &= !bb!(Square::A8);
            self.black_rook_board |= bb!(Square::D8);
        }

        if movement.piece == Piece::King {
            match self.turn {
                Color::White => {
                    self.white_castling_kings_side = false;
                    self.white_castling_queen_side = false;
                }
                Color::Black => {
                    self.black_castling_kings_side = false;
                    self.black_castling_queen_side = false;
                }
            }
        }

        if movement.from == Square::A1 && movement.piece == Piece::Rook {
            self.white_castling_queen_side = false;
        }

        if movement.from == Square::H1 && movement.piece == Piece::Rook {
            self.white_castling_kings_side = false;
        }

        if movement.from == Square::A8 && movement.piece == Piece::Rook {
            self.black_castling_queen_side = false;
        }

        if movement.from == Square::H8 && movement.piece == Piece::Rook {
            self.black_castling_kings_side = false;
        }

        if let Some(promotion) = movement.promotion {
            let pawn_board = self.color_board(&Piece::Pawn, self.turn);
            *pawn_board &= !bb!(movement.to);

            let new_piece_board = self.color_board(&promotion, self.turn);
            *new_piece_board |= bb!(movement.to);
        }

        if movement.piece == Piece::Pawn
            && self.turn == Color::White
            && movement.from.rank_char() == '2'
            && movement.to.rank_char() == '4'
        {
            let (file, rank) = (bb!(movement.to) >> 8).file_and_rank();
            self.en_passant = Some(Square::from_file_and_rank(file, rank));
        } else if movement.piece == Piece::Pawn
            && self.turn == Color::Black
            && movement.from.rank_char() == '7'
            && movement.to.rank_char() == '5'
        {
            let (file, rank) = (bb!(movement.to) << 8).file_and_rank();
            self.en_passant = Some(Square::from_file_and_rank(file, rank));
        } else {
            self.en_passant = None;
        }

        self.turn = self.turn.opposite();
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

    pub fn hash(&self) -> Result<u64, String> {
        hasher::hash_board(self)
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

    #[test]
    fn moves_a_piece() {
        let mut board = Board::from_start_position().unwrap();

        #[rustfmt::skip]
        assert_eq!(board.get_piece_at(&bb!(Square::E2)), Some((Color::White, Piece::Pawn)));
        assert_eq!(board.get_piece_at(&bb!(Square::E4)), None);

        board.move_piece(ResolvedMovement {
            piece: Piece::Pawn,
            from: Square::E2,
            to: Square::E4,
            capture: None,
            promotion: None,
        });

        #[rustfmt::skip]
        assert_eq!(board.get_piece_at(&bb!(Square::E4)), Some((Color::White, Piece::Pawn)));
        assert_eq!(board.get_piece_at(&bb!(Square::E2)), None);
    }

    #[test]
    fn remove_the_pawn_when_making_an_en_passant_move() {
        let fen = "2kr1b1r/pppb2pp/4p2q/4Pp2/1P2B3/2P3P1/P3QP1P/RN3RK1 w - f6 0 15";
        let mut board = Board::from_fen_str(fen).unwrap();

        assert_eq!(board.get_piece_at(&bb!(Square::F6)), None);

        let moves = crate::move_gen::pseudo_moves(&board);
        let m = moves
            .iter()
            .find(|m| m.capture == Some(Piece::Pawn) && m.to == Square::F6)
            .unwrap();

        board.move_piece(*m);

        assert_eq!(board.get_piece_at(&bb!(Square::F5)), None);
        assert_eq!(
            board.get_piece_at(&bb!(Square::F6)),
            Some((Color::White, Piece::Pawn))
        );
    }
}
