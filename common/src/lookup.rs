use crate::bb;
use crate::square::Square::*;
use crate::BitBoard;

#[rustfmt::skip]
pub const FILE_BITBOARDS: [BitBoard; 8] = [
    bb!(A1) | bb!(A2) | bb!(A3) | bb!(A4) | bb!(A5) | bb!(A6) | bb!(A7) | bb!(A8),
    bb!(B1) | bb!(B2) | bb!(B3) | bb!(B4) | bb!(B5) | bb!(B6) | bb!(B7) | bb!(B8),
    bb!(C1) | bb!(C2) | bb!(C3) | bb!(C4) | bb!(C5) | bb!(C6) | bb!(C7) | bb!(C8),
    bb!(D1) | bb!(D2) | bb!(D3) | bb!(D4) | bb!(D5) | bb!(D6) | bb!(D7) | bb!(D8),
    bb!(E1) | bb!(E2) | bb!(E3) | bb!(E4) | bb!(E5) | bb!(E6) | bb!(E7) | bb!(E8),
    bb!(F1) | bb!(F2) | bb!(F3) | bb!(F4) | bb!(F5) | bb!(F6) | bb!(F7) | bb!(F8),
    bb!(G1) | bb!(G2) | bb!(G3) | bb!(G4) | bb!(G5) | bb!(G6) | bb!(G7) | bb!(G8),
    bb!(H1) | bb!(H2) | bb!(H3) | bb!(H4) | bb!(H5) | bb!(H6) | bb!(H7) | bb!(H8)
];

#[rustfmt::skip]
pub const RANK_BITBOARDS: [BitBoard; 8] = [
    bb!(A1) | bb!(B1) | bb!(C1) | bb!(D1) | bb!(E1) | bb!(F1) | bb!(G1) | bb!(H1),
    bb!(A2) | bb!(B2) | bb!(C2) | bb!(D2) | bb!(E2) | bb!(F2) | bb!(G2) | bb!(H2),
    bb!(A3) | bb!(B3) | bb!(C3) | bb!(D3) | bb!(E3) | bb!(F3) | bb!(G3) | bb!(H3),
    bb!(A4) | bb!(B4) | bb!(C4) | bb!(D4) | bb!(E4) | bb!(F4) | bb!(G4) | bb!(H4),
    bb!(A5) | bb!(B5) | bb!(C5) | bb!(D5) | bb!(E5) | bb!(F5) | bb!(G5) | bb!(H5),
    bb!(A6) | bb!(B6) | bb!(C6) | bb!(D6) | bb!(E6) | bb!(F6) | bb!(G6) | bb!(H6),
    bb!(A7) | bb!(B7) | bb!(C7) | bb!(D7) | bb!(E7) | bb!(F7) | bb!(G7) | bb!(H7),
    bb!(A8) | bb!(B8) | bb!(C8) | bb!(D8) | bb!(E8) | bb!(F8) | bb!(G8) | bb!(H8),
];

pub const ALL_SQUARES: BitBoard = RANK_BITBOARDS[0]
    | RANK_BITBOARDS[1]
    | RANK_BITBOARDS[2]
    | RANK_BITBOARDS[3]
    | RANK_BITBOARDS[4]
    | RANK_BITBOARDS[5]
    | RANK_BITBOARDS[6]
    | RANK_BITBOARDS[7];

#[rustfmt::skip]
pub const ROOK_ATTACKS: [BitBoard; 64] = [
    RANK_BITBOARDS[0] | FILE_BITBOARDS[0], RANK_BITBOARDS[0] | FILE_BITBOARDS[1], RANK_BITBOARDS[0] | FILE_BITBOARDS[2], RANK_BITBOARDS[0] | FILE_BITBOARDS[3], RANK_BITBOARDS[0] | FILE_BITBOARDS[4], RANK_BITBOARDS[0] | FILE_BITBOARDS[5], RANK_BITBOARDS[0] | FILE_BITBOARDS[6], RANK_BITBOARDS[0] | FILE_BITBOARDS[7],
    RANK_BITBOARDS[1] | FILE_BITBOARDS[0], RANK_BITBOARDS[1] | FILE_BITBOARDS[1], RANK_BITBOARDS[1] | FILE_BITBOARDS[2], RANK_BITBOARDS[1] | FILE_BITBOARDS[3], RANK_BITBOARDS[1] | FILE_BITBOARDS[4], RANK_BITBOARDS[1] | FILE_BITBOARDS[5], RANK_BITBOARDS[1] | FILE_BITBOARDS[6], RANK_BITBOARDS[1] | FILE_BITBOARDS[7],
    RANK_BITBOARDS[2] | FILE_BITBOARDS[0], RANK_BITBOARDS[2] | FILE_BITBOARDS[1], RANK_BITBOARDS[2] | FILE_BITBOARDS[2], RANK_BITBOARDS[2] | FILE_BITBOARDS[3], RANK_BITBOARDS[2] | FILE_BITBOARDS[4], RANK_BITBOARDS[2] | FILE_BITBOARDS[5], RANK_BITBOARDS[2] | FILE_BITBOARDS[6], RANK_BITBOARDS[2] | FILE_BITBOARDS[7],
    RANK_BITBOARDS[3] | FILE_BITBOARDS[0], RANK_BITBOARDS[3] | FILE_BITBOARDS[1], RANK_BITBOARDS[3] | FILE_BITBOARDS[2], RANK_BITBOARDS[3] | FILE_BITBOARDS[3], RANK_BITBOARDS[3] | FILE_BITBOARDS[4], RANK_BITBOARDS[3] | FILE_BITBOARDS[5], RANK_BITBOARDS[3] | FILE_BITBOARDS[6], RANK_BITBOARDS[3] | FILE_BITBOARDS[7],
    RANK_BITBOARDS[4] | FILE_BITBOARDS[0], RANK_BITBOARDS[4] | FILE_BITBOARDS[1], RANK_BITBOARDS[4] | FILE_BITBOARDS[2], RANK_BITBOARDS[4] | FILE_BITBOARDS[3], RANK_BITBOARDS[4] | FILE_BITBOARDS[4], RANK_BITBOARDS[4] | FILE_BITBOARDS[5], RANK_BITBOARDS[4] | FILE_BITBOARDS[6], RANK_BITBOARDS[4] | FILE_BITBOARDS[7],
    RANK_BITBOARDS[5] | FILE_BITBOARDS[0], RANK_BITBOARDS[5] | FILE_BITBOARDS[1], RANK_BITBOARDS[5] | FILE_BITBOARDS[2], RANK_BITBOARDS[5] | FILE_BITBOARDS[3], RANK_BITBOARDS[5] | FILE_BITBOARDS[4], RANK_BITBOARDS[5] | FILE_BITBOARDS[5], RANK_BITBOARDS[5] | FILE_BITBOARDS[6], RANK_BITBOARDS[5] | FILE_BITBOARDS[7],
    RANK_BITBOARDS[6] | FILE_BITBOARDS[0], RANK_BITBOARDS[6] | FILE_BITBOARDS[1], RANK_BITBOARDS[6] | FILE_BITBOARDS[2], RANK_BITBOARDS[6] | FILE_BITBOARDS[3], RANK_BITBOARDS[6] | FILE_BITBOARDS[4], RANK_BITBOARDS[6] | FILE_BITBOARDS[5], RANK_BITBOARDS[6] | FILE_BITBOARDS[6], RANK_BITBOARDS[6] | FILE_BITBOARDS[7],
    RANK_BITBOARDS[7] | FILE_BITBOARDS[0], RANK_BITBOARDS[7] | FILE_BITBOARDS[1], RANK_BITBOARDS[7] | FILE_BITBOARDS[2], RANK_BITBOARDS[7] | FILE_BITBOARDS[3], RANK_BITBOARDS[7] | FILE_BITBOARDS[4], RANK_BITBOARDS[7] | FILE_BITBOARDS[5], RANK_BITBOARDS[7] | FILE_BITBOARDS[6], RANK_BITBOARDS[7] | FILE_BITBOARDS[7],
];

#[rustfmt::skip]
pub const BISHOP_ATTACKS: [BitBoard; 64] = [
    DIAGONAL_BIT_BOARDS[7] | ANTI_DIAGONAL_BIT_BOARDS[0], DIAGONAL_BIT_BOARDS[7 - 1] | ANTI_DIAGONAL_BIT_BOARDS[1], DIAGONAL_BIT_BOARDS[7 - 2] | ANTI_DIAGONAL_BIT_BOARDS[2], DIAGONAL_BIT_BOARDS[7 - 3] | ANTI_DIAGONAL_BIT_BOARDS[3], DIAGONAL_BIT_BOARDS[7 - 4] | ANTI_DIAGONAL_BIT_BOARDS[4], DIAGONAL_BIT_BOARDS[7 - 5] | ANTI_DIAGONAL_BIT_BOARDS[5], DIAGONAL_BIT_BOARDS[7 - 6] | ANTI_DIAGONAL_BIT_BOARDS[6], DIAGONAL_BIT_BOARDS[7 - 7] | ANTI_DIAGONAL_BIT_BOARDS[7],
    DIAGONAL_BIT_BOARDS[7 + 1] | ANTI_DIAGONAL_BIT_BOARDS[1], DIAGONAL_BIT_BOARDS[7 + 1 - 1] | ANTI_DIAGONAL_BIT_BOARDS[1 + 1], DIAGONAL_BIT_BOARDS[7 + 1 - 2] | ANTI_DIAGONAL_BIT_BOARDS[1 + 2], DIAGONAL_BIT_BOARDS[7 + 1 - 3] | ANTI_DIAGONAL_BIT_BOARDS[1 + 3], DIAGONAL_BIT_BOARDS[7 + 1 - 4] | ANTI_DIAGONAL_BIT_BOARDS[1 + 4], DIAGONAL_BIT_BOARDS[7 + 1 - 5] | ANTI_DIAGONAL_BIT_BOARDS[1 + 5], DIAGONAL_BIT_BOARDS[7 + 1 - 6] | ANTI_DIAGONAL_BIT_BOARDS[1 + 6], DIAGONAL_BIT_BOARDS[7 + 1 - 7] | ANTI_DIAGONAL_BIT_BOARDS[1 + 7],
    DIAGONAL_BIT_BOARDS[7 + 2] | ANTI_DIAGONAL_BIT_BOARDS[2], DIAGONAL_BIT_BOARDS[7 + 2 - 1] | ANTI_DIAGONAL_BIT_BOARDS[2 + 1], DIAGONAL_BIT_BOARDS[7 + 2 - 2] | ANTI_DIAGONAL_BIT_BOARDS[2 + 2], DIAGONAL_BIT_BOARDS[7 + 2 - 3] | ANTI_DIAGONAL_BIT_BOARDS[2 + 3], DIAGONAL_BIT_BOARDS[7 + 2 - 4] | ANTI_DIAGONAL_BIT_BOARDS[2 + 4], DIAGONAL_BIT_BOARDS[7 + 2 - 5] | ANTI_DIAGONAL_BIT_BOARDS[2 + 5], DIAGONAL_BIT_BOARDS[7 + 2 - 6] | ANTI_DIAGONAL_BIT_BOARDS[2 + 6], DIAGONAL_BIT_BOARDS[7 + 2 - 7] | ANTI_DIAGONAL_BIT_BOARDS[2 + 7],
    DIAGONAL_BIT_BOARDS[7 + 3] | ANTI_DIAGONAL_BIT_BOARDS[3], DIAGONAL_BIT_BOARDS[7 + 3 - 1] | ANTI_DIAGONAL_BIT_BOARDS[3 + 1], DIAGONAL_BIT_BOARDS[7 + 3 - 2] | ANTI_DIAGONAL_BIT_BOARDS[3 + 2], DIAGONAL_BIT_BOARDS[7 + 3 - 3] | ANTI_DIAGONAL_BIT_BOARDS[3 + 3], DIAGONAL_BIT_BOARDS[7 + 3 - 4] | ANTI_DIAGONAL_BIT_BOARDS[3 + 4], DIAGONAL_BIT_BOARDS[7 + 3 - 5] | ANTI_DIAGONAL_BIT_BOARDS[3 + 5], DIAGONAL_BIT_BOARDS[7 + 3 - 6] | ANTI_DIAGONAL_BIT_BOARDS[3 + 6], DIAGONAL_BIT_BOARDS[7 + 3 - 7] | ANTI_DIAGONAL_BIT_BOARDS[3 + 7],
    DIAGONAL_BIT_BOARDS[7 + 4] | ANTI_DIAGONAL_BIT_BOARDS[4], DIAGONAL_BIT_BOARDS[7 + 4 - 1] | ANTI_DIAGONAL_BIT_BOARDS[4 + 1], DIAGONAL_BIT_BOARDS[7 + 4 - 2] | ANTI_DIAGONAL_BIT_BOARDS[4 + 2], DIAGONAL_BIT_BOARDS[7 + 4 - 3] | ANTI_DIAGONAL_BIT_BOARDS[4 + 3], DIAGONAL_BIT_BOARDS[7 + 4 - 4] | ANTI_DIAGONAL_BIT_BOARDS[4 + 4], DIAGONAL_BIT_BOARDS[7 + 4 - 5] | ANTI_DIAGONAL_BIT_BOARDS[4 + 5], DIAGONAL_BIT_BOARDS[7 + 4 - 6] | ANTI_DIAGONAL_BIT_BOARDS[4 + 6], DIAGONAL_BIT_BOARDS[7 + 4 - 7] | ANTI_DIAGONAL_BIT_BOARDS[4 + 7],
    DIAGONAL_BIT_BOARDS[7 + 5] | ANTI_DIAGONAL_BIT_BOARDS[5], DIAGONAL_BIT_BOARDS[7 + 5 - 1] | ANTI_DIAGONAL_BIT_BOARDS[5 + 1], DIAGONAL_BIT_BOARDS[7 + 5 - 2] | ANTI_DIAGONAL_BIT_BOARDS[5 + 2], DIAGONAL_BIT_BOARDS[7 + 5 - 3] | ANTI_DIAGONAL_BIT_BOARDS[5 + 3], DIAGONAL_BIT_BOARDS[7 + 5 - 4] | ANTI_DIAGONAL_BIT_BOARDS[5 + 4], DIAGONAL_BIT_BOARDS[7 + 5 - 5] | ANTI_DIAGONAL_BIT_BOARDS[5 + 5], DIAGONAL_BIT_BOARDS[7 + 5 - 6] | ANTI_DIAGONAL_BIT_BOARDS[5 + 6], DIAGONAL_BIT_BOARDS[7 + 5 - 7] | ANTI_DIAGONAL_BIT_BOARDS[5 + 7],
    DIAGONAL_BIT_BOARDS[7 + 6] | ANTI_DIAGONAL_BIT_BOARDS[6], DIAGONAL_BIT_BOARDS[7 + 6 - 1] | ANTI_DIAGONAL_BIT_BOARDS[6 + 1], DIAGONAL_BIT_BOARDS[7 + 6 - 2] | ANTI_DIAGONAL_BIT_BOARDS[6 + 2], DIAGONAL_BIT_BOARDS[7 + 6 - 3] | ANTI_DIAGONAL_BIT_BOARDS[6 + 3], DIAGONAL_BIT_BOARDS[7 + 6 - 4] | ANTI_DIAGONAL_BIT_BOARDS[6 + 4], DIAGONAL_BIT_BOARDS[7 + 6 - 5] | ANTI_DIAGONAL_BIT_BOARDS[6 + 5], DIAGONAL_BIT_BOARDS[7 + 6 - 6] | ANTI_DIAGONAL_BIT_BOARDS[6 + 6], DIAGONAL_BIT_BOARDS[7 + 6 - 7] | ANTI_DIAGONAL_BIT_BOARDS[6 + 7],
    DIAGONAL_BIT_BOARDS[7 + 7] | ANTI_DIAGONAL_BIT_BOARDS[7], DIAGONAL_BIT_BOARDS[7 + 7 - 1] | ANTI_DIAGONAL_BIT_BOARDS[7 + 1], DIAGONAL_BIT_BOARDS[7 + 7 - 2] | ANTI_DIAGONAL_BIT_BOARDS[7 + 2], DIAGONAL_BIT_BOARDS[7 + 7 - 3] | ANTI_DIAGONAL_BIT_BOARDS[7 + 3], DIAGONAL_BIT_BOARDS[7 + 7 - 4] | ANTI_DIAGONAL_BIT_BOARDS[7 + 4], DIAGONAL_BIT_BOARDS[7 + 7 - 5] | ANTI_DIAGONAL_BIT_BOARDS[7 + 5], DIAGONAL_BIT_BOARDS[7 + 7 - 6] | ANTI_DIAGONAL_BIT_BOARDS[7 + 6], DIAGONAL_BIT_BOARDS[7 + 7 - 7] | ANTI_DIAGONAL_BIT_BOARDS[7 + 7],
];

#[rustfmt::skip]
pub const ANTI_DIAGONAL_BIT_BOARDS: [BitBoard; 15] = [
    bb!(A1),
    bb!(B1) | bb!(A2),
    bb!(C1) | bb!(B2) | bb!(A3),
    bb!(D1) | bb!(C2) | bb!(B3) | bb!(A4),
    bb!(E1) | bb!(D2) | bb!(C3) | bb!(B4) | bb!(A5),
    bb!(F1) | bb!(E2) | bb!(D3) | bb!(C4) | bb!(B5) | bb!(A6),
    bb!(G1) | bb!(F2) | bb!(E3) | bb!(D4) | bb!(C5) | bb!(B6) | bb!(A7),
    bb!(H1) | bb!(G2) | bb!(F3) | bb!(E4) | bb!(D5) | bb!(C6) | bb!(B7) | bb!(A8),
    bb!(H2) | bb!(G3) | bb!(F4) | bb!(E5) | bb!(D6) | bb!(C7) | bb!(B8),
    bb!(H3) | bb!(G4) | bb!(F5) | bb!(E6) | bb!(D7) | bb!(C8),
    bb!(H4) | bb!(G5) | bb!(F6) | bb!(E7) | bb!(D8),
    bb!(H5) | bb!(G6) | bb!(F7) | bb!(E8),
    bb!(H6) | bb!(G7) | bb!(F8),
    bb!(H7) | bb!(G8),
    bb!(H8),
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

pub const EDGES: BitBoard =
    RANK_BITBOARDS[0] | RANK_BITBOARDS[7] | FILE_BITBOARDS[0] | FILE_BITBOARDS[7];

pub const CASTLE_WHITE_KING_SIDE: BitBoard = bb!(G1) | bb!(F1);
pub const CASTLE_WHITE_QUEEN_SIDE: BitBoard = bb!(D1) | bb!(C1);
pub const CASTLE_BLACK_KING_SIDE: BitBoard = bb!(G8) | bb!(F8);
pub const CASTLE_BLACK_QUEEN_SIDE: BitBoard = bb!(D8) | bb!(C8);
