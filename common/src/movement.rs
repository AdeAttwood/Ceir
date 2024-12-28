use crate::attacked_squares;
use crate::bb;
use crate::castle_moves;
use crate::is_move_to_check;
use crate::pseudo_moves;
use crate::Board;
use crate::Piece;
use crate::Square;

#[derive(Debug, Copy, Clone)]
pub enum Movement {
    Ambiguous(AmbiguousMovement),
    Resolved(ResolvedMovement),
}

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
        let moves = [
            pseudo_moves(board),
            castle_moves(board, &attacked_squares(board, &board.turn.opposite())),
        ]
        .concat();

        for m in &moves {
            if let Some(file) = self.file {
                if self.to == m.to
                    && (self.piece == Some(m.piece) || self.piece.is_none())
                    && (file == m.from.file_char() || file == m.from.rank_char())
                    && !is_move_to_check(board, *m)
                {
                    let mut resolved = *m;
                    resolved.capture = self.resolve_capture(board);
                    resolved.promotion = self.promotion;
                    return Ok(resolved);
                }

                continue;
            }

            if self.to == m.to && self.piece == Some(m.piece) && !is_move_to_check(board, *m) {
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

    pub fn uci(&self) -> String {
        if let Some(promotion) = self.promotion {
            format!(
                "{}{}{}",
                self.from.uci(),
                self.to.uci(),
                promotion.to_lower()
            )
        } else {
            format!("{}{}", self.from.uci(), self.to.uci())
        }
    }
}
