use crate::bitboards::*;
use crate::fen::Fen;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Color::Black => write!(f, "black"),
            Color::White => write!(f, "white"),
        }
    }
}

#[derive(Copy, Clone)]
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

    // An extra bit board for printing markers on the board. This is useful for debugging.
    markers: BitBoard,
}

impl Board {
    /// Create a new empty board with no pieces in it
    pub fn empty() -> Self {
        Self {
            turn: Color::White,

            black_rook_board: 0,
            black_bishop_board: 0,
            black_knight_board: 0,
            black_pawn_board: 0,
            black_queen_board: 0,
            black_king_board: 0,
            white_bishop_board: 0,
            white_king_board: 0,
            white_knight_board: 0,
            white_pawn_board: 0,
            white_queen_board: 0,
            white_rook_board: 0,

            markers: 0,
        }
    }

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
        let mut board = Self::empty();
        board.load_fen(&Fen::from_str(&fen)?);

        Ok(board)
    }

    // This the first function to be created that loaded a board. Now we have the fen loader its
    // much more convenient and this will probably be removed later.
    #[allow(dead_code)]
    fn load_array(&mut self, array: [&str; 64]) {
        for i in 0..64 {
            // Because the square A1 is the right most bit we need to reverse the number of bits we
            // are shifting to come from the right
            let index = 63 - i;
            match array[i as usize] {
                "R" => self.black_rook_board = self.black_rook_board | (1 << index),
                "B" => self.black_bishop_board = self.black_bishop_board | (1 << index),
                "N" => self.black_knight_board = self.black_knight_board | (1 << index),
                "P" => self.black_pawn_board = self.black_pawn_board | (1 << index),
                "K" => self.black_king_board = self.black_king_board | (1 << index),
                "Q" => self.black_queen_board = self.black_queen_board | (1 << index),
                "r" => self.white_rook_board = self.white_rook_board | (1 << index),
                "b" => self.white_bishop_board = self.white_bishop_board | (1 << index),
                "n" => self.white_knight_board = self.white_knight_board | (1 << index),
                "p" => self.white_pawn_board = self.white_pawn_board | (1 << index),
                "k" => self.white_king_board = self.white_king_board | (1 << index),
                "q" => self.white_queen_board = self.white_queen_board | (1 << index),
                " " => (),
                &_ => todo!(),
            }
        }
    }

    fn get_hash(&self) -> String {
        format!(
            "{:b}{:b}{:b}{:b}{:b}{:b}{:b}{:b}{:b}{:b}{:b}{:b}{}",
            self.black_rook_board,
            self.black_knight_board,
            self.black_king_board,
            self.black_queen_board,
            self.black_bishop_board,
            self.black_pawn_board,
            self.white_rook_board,
            self.white_knight_board,
            self.white_king_board,
            self.white_queen_board,
            self.white_bishop_board,
            self.white_pawn_board,
            match self.turn {
                Color::White => 1,
                Color::Black => 0,
            }
        )
    }

    pub fn print(&self) {
        println!("     a  b  c  d  e  f  g  h");
        println!("    ────────────────────────");
        print!(" 8 │");
        for i in (0..64).rev() {
            if self.markers & (1 << i) != 0 {
                print!(" x ")
            } else if self.black_rook_board & (1 << i) != 0 {
                print!(" r ")
            } else if self.black_king_board & (1 << i) != 0 {
                print!(" k ")
            } else if self.black_knight_board & (1 << i) != 0 {
                print!(" n ")
            } else if self.black_bishop_board & (1 << i) != 0 {
                print!(" b ")
            } else if self.black_queen_board & (1 << i) != 0 {
                print!(" q ")
            } else if self.black_pawn_board & (1 << i) != 0 {
                print!(" p ")
            } else if self.white_rook_board & (1 << i) != 0 {
                print!(" R ")
            } else if self.white_king_board & (1 << i) != 0 {
                print!(" K ")
            } else if self.white_knight_board & (1 << i) != 0 {
                print!(" N ")
            } else if self.white_bishop_board & (1 << i) != 0 {
                print!(" B ")
            } else if self.white_queen_board & (1 << i) != 0 {
                print!(" Q ")
            } else if self.white_pawn_board & (1 << i) != 0 {
                print!(" P ")
            } else {
                print!(" . ")
            }

            if i % 8 == 0 {
                print!("│ {}\n", i / 8 + 1);
                if i != 0 {
                    print!(" {} │", i / 8)
                }
            }
        }

        println!("    ────────────────────────");
        println!("     a  b  c  d  e  f  g  h");
        println!();
        println!("Its {} to move", self.turn);
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

    pub fn occupied(&self) -> BitBoard {
        self.black_pieces() | self.white_pieces()
    }

    pub fn do_move(&mut self, from: Square, to: Square) {
        let from_board: BitBoard = 1 << (from as usize);
        let to_board: BitBoard = 1 << (to as usize);

        let white_boards = vec![
            &mut self.white_rook_board,
            &mut self.white_bishop_board,
            &mut self.white_knight_board,
            &mut self.white_pawn_board,
            &mut self.white_queen_board,
            &mut self.white_king_board,
        ];

        let black_boards = vec![
            &mut self.black_rook_board,
            &mut self.black_bishop_board,
            &mut self.black_knight_board,
            &mut self.black_pawn_board,
            &mut self.black_queen_board,
            &mut self.black_king_board,
        ];

        let (my_board, other_board) = match self.turn {
            Color::Black => (black_boards, white_boards),
            Color::White => (white_boards, black_boards),
        };

        for board in my_board {
            if *board & from_board != 0 {
                *board &= !from_board;
                *board |= to_board;
            }
        }

        for board in other_board {
            if *board & to_board != 0 {
                *board &= !to_board;
            }
        }

        self.turn = self.turn.opposite();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn will_do_it() {
        let board = Board::from_fen_str("3kq3/8/8/8/8/8/8/3K4 b - - 0 1").unwrap();
        assert_eq!(
            board.get_hash(),
            "0010000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000100000000"
        );
    }
}