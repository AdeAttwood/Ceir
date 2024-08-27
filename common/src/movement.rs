use crate::bb;
use crate::pseudo_moves;
use crate::Board;
use crate::Piece;
use crate::Square;

#[derive(Debug, Copy, Default, Clone)]
pub struct AmbiguousMovement {
    pub file: Option<char>,
    pub piece: Option<Piece>,
    pub from: Option<Square>,
    pub to: Square,
    pub capture: Option<Piece>,
    pub promotion: Option<Piece>,
}

impl AmbiguousMovement {
    fn resolve_capture(&self, board: &Board) -> Option<Piece> {
        if let Some(capture) = self.capture {
            return Some(capture);
        }

        if let Some((_, capture)) = board.get_piece_at(&bb!(self.to)) {
            return Some(capture);
        }

        None
    }

    pub fn resolve(&self, board: &Board) -> Result<ResolvedMovement, String> {
        let moves = pseudo_moves(&board);

        for m in &moves {
            if let Some(file) = self.file {
                if self.to == m.to && (self.piece == Some(m.piece) || self.piece.is_none()) {
                    let c = m.from.file_char();
                    if c == file {
                        let mut resolved = *m;
                        resolved.capture = self.resolve_capture(board);
                        resolved.promotion = self.promotion;
                        return Ok(resolved);
                    }
                }

                continue;
            }

            if self.to == m.to && self.piece == Some(m.piece) {
                let mut resolved = *m;
                resolved.capture = self.resolve_capture(board);
                resolved.promotion = self.promotion;
                return Ok(resolved);
            }
        }

        Err("Unable to resolve ambiguous movement".to_string())
    }
}

#[derive(Debug, Copy, Default, Clone)]
pub struct ResolvedMovement {
    pub piece: Piece,
    pub from: Square,
    pub to: Square,
    pub capture: Option<Piece>,
    pub promotion: Option<Piece>,
}

impl ResolvedMovement {
    pub fn is_white_king_castle(&self) -> bool {
        self.piece == Piece::King && self.from == Square::E1 && self.to == Square::G1
    }

    pub fn is_white_queen_castle(&self) -> bool {
        self.piece == Piece::King && self.from == Square::E1 && self.to == Square::C1
    }

    pub fn is_black_king_castle(&self) -> bool {
        self.piece == Piece::King && self.from == Square::E8 && self.to == Square::G8
    }

    pub fn is_black_queen_castle(&self) -> bool {
        self.piece == Piece::King && self.from == Square::E8 && self.to == Square::C8
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Movement {
    Ambiguous(AmbiguousMovement),
    Resolved(ResolvedMovement),
}
