use std::collections::HashMap;

use crate::Board;

#[derive(Clone, Debug, Default)]
pub enum GameResult {
    #[default]
    InProgress,
    WhiteWin,
    BlackWin,
    Draw,
}

#[derive(Clone, Debug, Default)]
pub struct Game {
    pub metadata: HashMap<String, String>,
    pub history: Vec<Board>,
    pub board: Board,
    pub result: GameResult,
}
