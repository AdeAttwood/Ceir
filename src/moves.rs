use crate::bitboards::{Square, SQUARE_NAME_LOOKUP};
use crate::board::Piece;

#[derive(Debug, Copy, Clone)]
pub struct Move {
    pub piece: Piece,
    pub from: Square,
    pub to: Square,
    pub capture: Option<Piece>,
}

impl Move {
    pub fn format_move(&self) -> String {
        format!(
            "{}{}",
            SQUARE_NAME_LOOKUP[self.from as usize], SQUARE_NAME_LOOKUP[self.to as usize]
        )
    }
}
