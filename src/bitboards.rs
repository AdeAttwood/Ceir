use Square::*;

pub type BitBoard = i64;

#[allow(dead_code)]
pub fn print_bitboard(bitboard: &BitBoard) {
    println!("────────────────────────");
    for i in (0..64).rev() {
        if bitboard & (1 << i) != 0 {
            print!(" x ");
        } else {
            print!(" . ");
        }

        if i % 8 == 0 {
            print!("\n");
        }
    }
    println!("────────────────────────");
}

macro_rules! bb {
    ($square:expr) => {
        1 << $square as usize
    };
}

pub const RANK_BIT_BOARDS: [BitBoard; 8] = [
    255,
    65280,
    16711680,
    4278190080,
    1095216660480,
    280375465082880,
    71776119061217280,
    -72057594037927936,
];

// index 0 = file H
pub const FILE_BIT_BOARDS: [BitBoard; 8] = [
    72340172838076673,
    144680345676153346,
    289360691352306692,
    578721382704613384,
    1157442765409226768,
    2314885530818453536,
    4629771061636907072,
    -9187201950435737472,
];

#[derive(Debug, PartialEq, Clone, Copy)]
#[rustfmt::skip]
pub enum Square {
    H1, G1, F1, E1, D1, C1, B1, A1,
    H2, G2, F2, E2, D2, C2, B2, A2,
    H3, G3, F3, E3, D3, C3, B3, A3,
    H4, G4, F4, E4, D4, C4, B4, A4,
    H5, G5, F5, E5, D5, C5, B5, A5,
    H6, G6, F6, E6, D6, C6, B6, A6,
    H7, G7, F7, E7, D7, C7, B7, A7,
    H8, G8, F8, E8, D8, C8, B8, A8,
}

impl TryFrom<&str> for Square {
    type Error = String;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let found = SQUARE_NAME_LOOKUP
            .iter()
            .position(|square| square == &input);
        match found {
            Some(s) => Ok(SQUARE_LOOKUP[s]),
            None => Err(format!("Invalid square {input}")),
        }
    }
}

#[rustfmt::skip]
pub const SQUARE_LOOKUP: [Square; 64] = [
    H1, G1, F1, E1, D1, C1, B1, A1,
    H2, G2, F2, E2, D2, C2, B2, A2,
    H3, G3, F3, E3, D3, C3, B3, A3,
    H4, G4, F4, E4, D4, C4, B4, A4,
    H5, G5, F5, E5, D5, C5, B5, A5,
    H6, G6, F6, E6, D6, C6, B6, A6,
    H7, G7, F7, E7, D7, C7, B7, A7,
    H8, G8, F8, E8, D8, C8, B8, A8,
];

pub const SQUARE_NAME_LOOKUP: [&str; 64] = [
    "h1", "g1", "f1", "e1", "d1", "c1", "b1", "a1", "h2", "g2", "f2", "e2", "d2", "c2", "b2", "a2",
    "h3", "g3", "f3", "e3", "d3", "c3", "b3", "a3", "h4", "g4", "f4", "e4", "d4", "c4", "b4", "a4",
    "h5", "g5", "f5", "e5", "d5", "c5", "b5", "a5", "h6", "g6", "f6", "e6", "d6", "c6", "b6", "a6",
    "h7", "g7", "f7", "e7", "d7", "c7", "b7", "a7", "h8", "g8", "f8", "e8", "d8", "c8", "b8", "a8",
];

#[rustfmt::skip]
pub const ANTI_DIAGONAL_BIT_BOARDS: [BitBoard; 15] = [
    bb!(H8),
    bb!(H7) | bb!(G8),
    bb!(H6) | bb!(G7) | bb!(F8),
    bb!(H5) | bb!(G6) | bb!(F7) | bb!(E8),
    bb!(H4) | bb!(G5) | bb!(F6) | bb!(E7) | bb!(D8),
    bb!(H3) | bb!(G4) | bb!(F5) | bb!(E6) | bb!(D7) | bb!(C8),
    bb!(H2) | bb!(G3) | bb!(F4) | bb!(E5) | bb!(D6) | bb!(C7) | bb!(B8),
    bb!(H1) | bb!(G2) | bb!(F3) | bb!(E4) | bb!(D5) | bb!(C6) | bb!(B7) | bb!(A8),
    bb!(G1) | bb!(F2) | bb!(E3) | bb!(D4) | bb!(C5) | bb!(B6) | bb!(A7),
    bb!(F1) | bb!(E2) | bb!(D3) | bb!(C4) | bb!(B5) | bb!(A6),
    bb!(E1) | bb!(D2) | bb!(C3) | bb!(B4) | bb!(A5),
    bb!(D1) | bb!(C2) | bb!(B3) | bb!(A4),
    bb!(C1) | bb!(B2) | bb!(A3),
    bb!(B1) | bb!(A2),
    bb!(A1),
];

#[rustfmt::skip]
pub const DIAGONAL_BIT_BOARDS: [BitBoard; 15] = [
    bb!(H1),
    bb!(G1) | bb!(H2),
    bb!(F1) | bb!(G2) | bb!(H3),
    bb!(E1) | bb!(F2) | bb!(G3) | bb!(H4),
    bb!(D1) | bb!(E2) | bb!(F3) | bb!(G4) | bb!(H5),
    bb!(C1) | bb!(D2) | bb!(E3) | bb!(F4) | bb!(G5) | bb!(H6),
    bb!(B1) | bb!(C2) | bb!(D3) | bb!(E4) | bb!(F5) | bb!(G6) | bb!(H7),
    bb!(A1) | bb!(B2) | bb!(C3) | bb!(D4) | bb!(E5) | bb!(F6) | bb!(G7) | bb!(H8),
    bb!(A2) | bb!(B3) | bb!(C4) | bb!(D5) | bb!(E6) | bb!(F7) | bb!(G8),
    bb!(A3) | bb!(B4) | bb!(C5) | bb!(D6) | bb!(E7) | bb!(F8),
    bb!(A4) | bb!(B5) | bb!(C6) | bb!(D7) | bb!(E8),
    bb!(A5) | bb!(B6) | bb!(C7) | bb!(D8),
    bb!(A6) | bb!(B7) | bb!(C8),
    bb!(A7) | bb!(B8),
    bb!(A8),
];

pub struct BitBoardIterator {
    board: BitBoard,
}

impl BitBoardIterator {
    pub fn new(board: BitBoard) -> Self {
        Self { board }
    }

    // Advance the iterator by removing the least significant bit and returning the index at that
    // bit. Once there are no significant bits left None is returned.
    //
    // https://www.chessprogramming.org/General_Setwise_Operations#TheLeastSignificantOneBitLS1B
    pub fn next(&mut self) -> Option<usize> {
        let trailing = self.board.trailing_zeros() as usize;
        if trailing == 64 {
            None
        } else {
            self.board = self.board & !(1 << trailing);
            Some(trailing)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn will_do_it() {
        let mut itr = BitBoardIterator::new(4);
        assert!(itr.next().is_some());
        assert!(itr.next().is_none());
    }

    #[test]
    fn will_do_it_more() {
        let mut itr = BitBoardIterator::new(-9223372036854775808);
        assert_eq!(itr.next().unwrap(), 63);
        assert!(itr.next().is_none());
    }

    #[test]
    fn will_do_it_with_zero() {
        let mut itr = BitBoardIterator::new(0);
        assert!(itr.next().is_none());
    }

    #[test]
    fn will_get_a_move_from_a_string() {
        let square: Square = "a2".try_into().unwrap();
        assert_eq!(square, Square::A2);
    }

    #[test]
    fn will_fail_on_an_invalid_square() {
        let error = Square::try_from("a9");
        assert!(error.is_err());
    }
}
