use std::collections::HashMap;

use crate::{Board, ResolvedMovement};

#[derive(Clone, Debug, Default)]
pub enum GameResult {
    #[default]
    InProgress,
    WhiteWin,
    BlackWin,
    Draw,
}

#[derive(Debug, Default)]
pub struct Game {
    pub metadata: HashMap<String, String>,
    pub history: Vec<ResolvedMovement>,
    pub board: Board,
    pub result: GameResult,
}

impl Game {
    pub fn move_piece(&mut self, movement: ResolvedMovement) {
        self.history.push(movement);
        self.board.move_piece(movement);
    }
}
