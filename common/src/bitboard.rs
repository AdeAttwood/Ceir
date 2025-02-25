/// The base bitboard type that wraps a 64 bit int so we can represent a chess board in small
/// amount of space and preform bit opperations on them fast.
///
/// See: https://www.chessprogramming.org/Bitboards
pub type BitBoard = u64;

/// Helper functions that will add functions to a u64 that will help keep things readable.
pub trait BitBoardable {
    fn print(&self);
    fn file_and_rank(&self) -> (usize, usize);
}

impl BitBoardable for u64 {
    /// Calcualtes the file and rank of the bit in the bitboard. I will return a 0 baised index of
    /// the file and rank btween 0 and 7. It will only work on a bitboard that has one peice
    /// marked. If the board has more than that it will panic.
    ///
    /// ```
    /// use common::bb;
    /// use common::Square;
    /// use common::BitBoardable;
    /// use common::BitBoard;
    ///
    /// let board: BitBoard = bb!(Square::A2);
    /// let (file, rank) = board.file_and_rank();
    ///
    /// Square::from_file_and_rank(file, rank);
    /// ```
    fn file_and_rank(&self) -> (usize, usize) {
        let trailing_zeros = self.trailing_zeros();
        (
            (7 - (trailing_zeros % 8)) as usize,
            (trailing_zeros / 8) as usize,
        )
    }

    fn print(&self) {
        const LAST_BIT: u64 = 63;

        println!("     a  b  c  d  e  f  g  h");
        println!("    ────────────────────────");

        for rank in 0..8 {
            print!("{} │", 8 - rank);
            for file in 0..8 {
                let mask = 1u64 << (LAST_BIT - (rank * 8) - file);
                let char = if self & mask != 0 { " x " } else { " . " };
                print!("{char}");
            }

            println!(" │ {}", 8 - rank);
        }

        println!("    ────────────────────────");
        println!("     a  b  c  d  e  f  g  h");
    }
}

/// ```
/// use common::bb;
/// use common::Square;
///
/// bb!(Square::A1);
/// ```
#[macro_export]
macro_rules! bb {
    ($square:expr) => {
        1 << $square as usize
    };
}

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
            self.board &= !(1 << trailing);
            Some(trailing)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Square;

    use super::*;

    macro_rules! bb_str {
        ($square:expr) => {
            format!("{:064b}", $square)
        };
    }

    #[test]
    fn flips_the_correct_bits() {
        assert_eq!(
            bb_str!(bb!(Square::A1)),
            "0000000000000000000000000000000000000000000000000000000010000000",
        );

        assert_eq!(
            bb_str!(bb!(Square::from_usize(27))),
            "0000000000000000000000000000000000001000000000000000000000000000",
        );
    }

    #[test]
    fn this_will() {
        let (file, rank) = bb!(Square::C7).file_and_rank();
        assert_eq!(file, 2);
        assert_eq!(rank, 6);
    }

    #[test]
    fn will_do_it() {
        let mut itr = BitBoardIterator::new(bb!(Square::A1));
        assert!(itr.next().is_some());
        assert!(itr.next().is_none());
    }

    #[test]
    fn will_do_it_with_zero() {
        let mut itr = BitBoardIterator::new(0);
        assert!(itr.next().is_none());
    }

    #[test]
    fn we_can_iterate_over_the_bits_in_the_correct_order() {
        let board = bb!(Square::A1) | bb!(Square::B1) | bb!(Square::C1);
        let mut itr = BitBoardIterator::new(board);

        assert_eq!(Square::from_usize(itr.next().unwrap()), Square::C1);
        assert_eq!(Square::from_usize(itr.next().unwrap()), Square::B1);
        assert_eq!(Square::from_usize(itr.next().unwrap()), Square::A1);
    }
}
