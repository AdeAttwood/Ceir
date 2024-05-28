use crate::Piece;
use crate::Square;
// use crate::;

#[derive(Debug, Copy, Default, Clone)]
pub struct Movement {
    pub piece: Piece,
    pub from: Option<Square>,
    pub to: Square,
    pub capture: Option<Piece>,
    pub promotion: Option<Piece>,
}

impl Movement {
    // pub fn format_move(&self) -> String {
    //     format!(
    //         "{}{}",
    //         SQUARE_NAME_LOOKUP[self.from as usize], SQUARE_NAME_LOOKUP[self.to as usize]
    //     )
    // }
}
