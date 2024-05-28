#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    #[default]
    Pawn,
}

impl Piece {
    pub fn from_str(c: &str) -> Option<Self> {
        match c {
            "k" => Some(Self::King),
            "q" => Some(Self::Queen),
            "r" => Some(Self::Rook),
            "b" => Some(Self::Bishop),
            "n" => Some(Self::Knight),
            "p" => Some(Self::Pawn),
            "K" => Some(Self::King),
            "Q" => Some(Self::Queen),
            "R" => Some(Self::Rook),
            "B" => Some(Self::Bishop),
            "N" => Some(Self::Knight),
            "P" => Some(Self::Pawn),
            _ => None,
        }
    }
}
