use std::str::FromStr;

use crate::Color;
use crate::Piece;
use crate::Square;

/// Forsyth–Edwards Notation (fen)
///
/// The Forsyth–Edwards Notation (FEN) is a standard notation for describing a particular board.
/// This will parse the FEN string and return the board state, this can then be used to recreate a
/// board position.
///
/// See: https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation
///
/// ```
/// use common::Fen;
/// use common::Color;
///
/// let fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
/// let fen = Fen::from_str(fen_string).unwrap();
///
/// assert_eq!(fen.turn, Color::White);
/// ```
pub struct Fen {
    pub turn: Color,
    pub white_castling_kings_side: bool,
    pub white_castling_queen_side: bool,
    pub black_castling_kings_side: bool,
    pub black_castling_queen_side: bool,
    pub en_passant: Option<Square>,
    pub squares: Vec<Option<(Color, Piece)>>,
    pub half_move_clock: i32,
    pub full_move_number: i32,
}

impl Fen {
    pub fn new(fen_string: &String) -> Result<Self, String> {
        let mut parts = fen_string.split_whitespace();

        if parts.clone().count() != 6 {
            return Err("The fen must have 6 parts".to_string());
        }

        let mut index = -1;
        let mut squares: Vec<Option<(Color, Piece)>> = Vec::new();
        for c in parts.next().unwrap().chars() {
            if c == '/' {
                continue;
            }

            if let Ok(n) = c.to_string().parse::<i32>() {
                for _ in index..(n + index) {
                    index += 1;
                    squares.push(None);
                }

                continue;
            }

            index += 1;

            match c {
                'k' => squares.push(Some((Color::Black, Piece::King))),
                'q' => squares.push(Some((Color::Black, Piece::Queen))),
                'r' => squares.push(Some((Color::Black, Piece::Rook))),
                'n' => squares.push(Some((Color::Black, Piece::Knight))),
                'b' => squares.push(Some((Color::Black, Piece::Bishop))),
                'p' => squares.push(Some((Color::Black, Piece::Pawn))),

                'K' => squares.push(Some((Color::White, Piece::King))),
                'Q' => squares.push(Some((Color::White, Piece::Queen))),
                'R' => squares.push(Some((Color::White, Piece::Rook))),
                'N' => squares.push(Some((Color::White, Piece::Knight))),
                'B' => squares.push(Some((Color::White, Piece::Bishop))),
                'P' => squares.push(Some((Color::White, Piece::Pawn))),

                _ => return Err(format!("Unknown piece char '{c}' at position {index}")),
            }
        }

        // Check that we have 64 squares This will panic later when we try and fix each squares
        // into 64 bits.
        if squares.len() != 64 {
            return Err(format!(
                "Invalid fen there must be 64 squares found {}",
                squares.len()
            ));
        }

        let color = parts.next();
        let turn = match color {
            Some(c) => Color::from_str(c)?,
            None => return Err("Unable to parse color".to_string()),
        };

        let mut fen = Fen {
            squares,
            turn,
            en_passant: None,
            half_move_clock: 0,
            full_move_number: 0,
            white_castling_kings_side: false,
            white_castling_queen_side: false,
            black_castling_kings_side: false,
            black_castling_queen_side: false,
        };

        let castling = parts.next();
        if castling.is_some() {
            for c in castling.unwrap().chars() {
                match c {
                    'K' => fen.white_castling_kings_side = true,
                    'Q' => fen.white_castling_queen_side = true,
                    'k' => fen.black_castling_kings_side = true,
                    'q' => fen.black_castling_queen_side = true,
                    '-' => {}
                    _ => {
                        return Err(format!("Unexpected char '{c}' in castling part"));
                    }
                }
            }
        }

        fen.en_passant = match parts.next() {
            Some("-") => None,
            Some(square) => Some(Square::from_str(square)?),
            None => return Err("Missing en passant square".to_string()),
        };

        fen.half_move_clock = match parts.next() {
            Some(half_move_clock) => half_move_clock
                .parse()
                .map_err(|_| format!("Invalid half move clock {half_move_clock}").to_string())?,
            None => return Err("Missing half move clock".to_string()),
        };

        fen.full_move_number = match parts.next() {
            Some(half_move_clock) => half_move_clock
                .parse()
                .map_err(|_| format!("Invalid full move number {half_move_clock}").to_string())?,
            None => return Err("Missing full move number".to_string()),
        };

        Ok(fen)
    }

    pub fn from_str(fen: &str) -> Result<Self, String> {
        Self::new(&String::from(fen))
    }

    pub fn from_start_position() -> Result<Self, String> {
        let fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        Fen::from_str(fen_string)
    }

    pub fn to_string(&self) -> String {
        let squares = self
            .squares
            .chunks(8)
            .map(|file| "1".to_string())
            .collect::<Vec<String>>()
            .join("/");

        format!("{} {}", squares.to_string(), self.turn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dose_not_have_six_parts() {
        let message = Fen::from_str("a b").err().unwrap();
        assert_eq!(message, String::from("The fen must have 6 parts"));
    }

    #[test]
    fn invalid_piece() {
        let fen_string = "xnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let message = Fen::from_str(fen_string).err().unwrap();
        assert_eq!(
            message,
            String::from("Unknown piece char 'x' at position 0")
        );
    }

    #[test]
    fn invalid_number_of_squares() {
        let fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPP w KQkq - 0 1";
        let message = Fen::from_str(fen_string).err().unwrap();
        assert_eq!(
            message,
            String::from("Invalid fen there must be 64 squares found 54")
        );
    }

    #[test]
    fn invalid_color() {
        let fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR x KQkq - 0 1";
        let message = Fen::from_str(fen_string).err().unwrap();
        assert_eq!(message, String::from("Unable to parse 'x' into a color"));
    }

    macro_rules! assert_position {
        ($a:expr,$b:pat) => {
            assert_eq!(
                true,
                match $a {
                    $b => true,
                    _ => false,
                },
                "The position did not match: ({:?}))",
                $a,
            );
        };
    }

    #[test]
    fn parse_the_default_position() {
        let fen =
            Fen::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

        assert_position!(&fen.squares[0], Some((Color::Black, Piece::Rook)));
        assert_position!(&fen.squares[1], Some((Color::Black, Piece::Knight)));
        assert_position!(&fen.squares[2], Some((Color::Black, Piece::Bishop)));
        assert_position!(&fen.squares[3], Some((Color::Black, Piece::Queen)));
        assert_position!(&fen.squares[4], Some((Color::Black, Piece::King)));
        assert_position!(&fen.squares[5], Some((Color::Black, Piece::Bishop)));
        assert_position!(&fen.squares[6], Some((Color::Black, Piece::Knight)));
        assert_position!(&fen.squares[7], Some((Color::Black, Piece::Rook)));

        assert_eq!(true, matches!(fen.turn, Color::White));
    }

    macro_rules! assert_castling {
        ($pattern:literal, $white_king:literal, $white_queen:literal, $black_king:literal, $black_queen:literal) => {
            let fen_string = format!(
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w {} - 0 1",
                $pattern
            );

            let fen = Fen::from_str(&fen_string).unwrap();
            assert_eq!(fen.white_castling_kings_side, $white_king);
            assert_eq!(fen.white_castling_queen_side, $white_queen);
            assert_eq!(fen.black_castling_kings_side, $black_king);
            assert_eq!(fen.black_castling_queen_side, $black_queen);
        };
    }

    #[test]
    fn castling() {
        assert_castling!("KQkq", true, true, true, true);
        assert_castling!("KQk", true, true, true, false);
        assert_castling!("KQ", true, true, false, false);
        assert_castling!("K", true, false, false, false);
        assert_castling!("-", false, false, false, false);

        // Test in a different order
        assert_castling!("QK", true, true, false, false);
    }

    #[test]
    fn en_passant() {
        let fen_string = "rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 2";
        let fen = Fen::from_str(fen_string).unwrap();
        assert_eq!(fen.en_passant, Some(Square::D6));
    }

    #[test]
    fn clocks() {
        let fen_string = "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 5";
        let fen = Fen::from_str(fen_string).unwrap();

        assert_eq!(fen.half_move_clock, 4);
        assert_eq!(fen.full_move_number, 5);
    }

    #[test]
    fn to_string() {
        let fen_string =
            String::from("r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 5");
        let fen = Fen::new(&fen_string).unwrap();

        assert_eq!(fen.to_string(), fen_string);
    }
}
