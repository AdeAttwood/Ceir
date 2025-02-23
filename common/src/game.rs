use std::collections::HashMap;

use crate::{Board, Color, ResolvedMovement};

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
    pub half_move_clock: u32,
    pub full_move_number: u32,
    pub metadata: HashMap<String, String>,
    pub history: Vec<ResolvedMovement>,
    pub board: Board,
    pub result: GameResult,
}

impl Game {
    pub fn move_piece(&mut self, movement: ResolvedMovement) {
        if self.board.turn == Color::Black {
            self.full_move_number += 1;
        }

        if movement.capture.is_some() || movement.piece == crate::Piece::Pawn {
            self.half_move_clock = 0;
        } else {
            self.half_move_clock += 1;
        }

        self.history.push(movement);
        self.board.move_piece(movement);
    }

    pub fn play_to(&mut self, index: usize) -> Game {
        let mut game = Game {
            board: Board::from_start_position().unwrap(),
            ..Game::default()
        };

        for i in 0..index {
            game.move_piece(self.history[i]);
        }

        game
    }
}
