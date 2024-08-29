/// The full deffinistion all all of the squares on a board. They are in the order of the what bits
/// they index in a Bitboard so we can use then to easlly create bitboards or test posative bits at
/// a squares while makeing things readable.
///
/// See: https://www.chessprogramming.org/Squares
///
/// To create a bitboard from a square you can shift it by the index of the enum
///
/// ```
/// use common::Square;
///
/// 1 << Square::A1 as usize;
/// ```
///
/// For convitions you can use the `bb` macro to hide some of the bit shifting and clean stuff upp.
///
/// ```
/// use common::bb;
/// use common::Square;
///
/// bb!(Square::A1);
/// ```
#[derive(Debug, PartialEq, Clone, Default, Copy)]
#[rustfmt::skip]
pub enum Square {
    #[default]
    H1, G1, F1, E1, D1, C1, B1, A1,
    H2, G2, F2, E2, D2, C2, B2, A2,
    H3, G3, F3, E3, D3, C3, B3, A3,
    H4, G4, F4, E4, D4, C4, B4, A4,
    H5, G5, F5, E5, D5, C5, B5, A5,
    H6, G6, F6, E6, D6, C6, B6, A6,
    H7, G7, F7, E7, D7, C7, B7, A7,
    H8, G8, F8, E8, D8, C8, B8, A8,
}

impl Square {
    /// Create a square from a string like "a1"
    ///
    /// ```
    /// let s = common::Square::from_str("a1").unwrap();
    /// assert_eq!(s, common::Square::A1);
    /// ```
    pub fn from_str(s: &str) -> Result<Self, String> {
        let mut chars = s.chars();
        let file = chars.next().ok_or("Invalid square")?;
        let rank = chars.next().ok_or("Invalid square")?;

        Self::from_file_and_rank_str(&file.to_string(), &rank.to_string())
    }

    pub fn from_file_and_rank_str(file: &str, rank: &str) -> Result<Self, String> {
        let file_usize = match file {
            "a" => 0,
            "b" => 1,
            "c" => 2,
            "d" => 3,
            "e" => 4,
            "f" => 5,
            "g" => 6,
            "h" => 7,
            _ => return Err(format!("Invalid file '{}'", file)),
        };

        let rank_usize = match rank {
            "1" => 0,
            "2" => 1,
            "3" => 2,
            "4" => 3,
            "5" => 4,
            "6" => 5,
            "7" => 6,
            "8" => 7,
            _ => return Err(format!("Invalid rank '{}'", rank)),
        };

        Ok(Self::from_file_and_rank(file_usize, rank_usize))
    }

    pub fn from_file_and_rank(file: usize, rank: usize) -> Self {
        Self::from_usize(rank << 3 ^ (7 - file))
    }

    pub fn file_char(&self) -> char {
        (7 - (*self as usize & 7) as u8 + b'a') as char
    }

    #[rustfmt::skip]
    pub fn from_usize(index: usize) -> Self {
        match index {
             0 => Square::H1,  1 => Square::G1,  2 => Square::F1,  3 => Square::E1,  4 => Square::D1,  5 => Square::C1,  6 => Square::B1,  7 => Square::A1,
             8 => Square::H2,  9 => Square::G2, 10 => Square::F2, 11 => Square::E2, 12 => Square::D2, 13 => Square::C2, 14 => Square::B2, 15 => Square::A2,
            16 => Square::H3, 17 => Square::G3, 18 => Square::F3, 19 => Square::E3, 20 => Square::D3, 21 => Square::C3, 22 => Square::B3, 23 => Square::A3,
            24 => Square::H4, 25 => Square::G4, 26 => Square::F4, 27 => Square::E4, 28 => Square::D4, 29 => Square::C4, 30 => Square::B4, 31 => Square::A4,
            32 => Square::H5, 33 => Square::G5, 34 => Square::F5, 35 => Square::E5, 36 => Square::D5, 37 => Square::C5, 38 => Square::B5, 39 => Square::A5,
            40 => Square::H6, 41 => Square::G6, 42 => Square::F6, 43 => Square::E6, 44 => Square::D6, 45 => Square::C6, 46 => Square::B6, 47 => Square::A6,
            48 => Square::H7, 49 => Square::G7, 50 => Square::F7, 51 => Square::E7, 52 => Square::D7, 53 => Square::C7, 54 => Square::B7, 55 => Square::A7,
            56 => Square::H8, 57 => Square::G8, 58 => Square::F8, 59 => Square::E8, 60 => Square::D8, 61 => Square::C8, 62 => Square::B8, 63 => Square::A8,
            _ => panic!("'{index}' must be between 0 and 63.")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_char() {
        assert_eq!(Square::A1.file_char(), 'a');
        assert_eq!(Square::A5.file_char(), 'a');

        assert_eq!(Square::G5.file_char(), 'g');
    }

    #[test]
    fn we_can_create_a_square_from_a_file_and_rank() {
        let square = Square::from_file_and_rank(7, 7);
        assert_eq!(square, Square::H8);
    }

    #[test]
    fn we_can_get_a_square_in_the_middle_of_the_board() {
        let square = Square::from_file_and_rank(2, 6);
        assert_eq!(square, Square::C7);
    }

    #[test]
    fn from_file_and_rank() {
        let squares = vec![(0, 0, Square::A1), (1, 3, Square::B4)];

        for (file, rank, expected) in squares {
            let square = Square::from_file_and_rank(file, rank);
            assert_eq!(square, expected);
        }
    }

    #[test]
    fn converts_from_a_file_and_rank_string() {
        let square = Square::from_file_and_rank_str("h", "8").unwrap();
        assert_eq!(square, Square::H8);
    }

    #[test]
    fn converts_from_a_str() {
        let square = Square::from_str("h8").unwrap();
        assert_eq!(square, Square::H8);
    }
}
