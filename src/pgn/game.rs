use std::collections::HashMap;

use super::game_result::GameResult;

#[derive(Clone, Debug)]
pub struct PgnMove {
    pub ply: i32,
    pub san: String,
}

pub struct PgnGame {
    pub meta: HashMap<String, String>,
    pub moves: Vec<PgnMove>,
    pub result: GameResult,
}

impl PgnGame {
    pub fn new() -> Self {
        Self {
            meta: HashMap::new(),
            moves: Vec::new(),
            result: GameResult::InProgress,
        }
    }
}
