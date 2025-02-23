mod uci_client;

use common::{Board, Game, Movement, ResolvedMovement};
use uci_client::{GoCommandResult, Score};
use uci_client::{UciEngine, UciError, UciResult};

const MAX_DEPTH: u32 = 20;

#[derive(Debug)]
pub enum MoveAnalysis {
    GoodMove,
    Blunder,
}

/// Builds a move line in uci format from teh start of the game to the move at the given index.
/// Move indexes are forced to be valid to the game history. If an index is given that is greater
/// than the history, we will return the line to the end of the history. If the index is equal to
/// less than 0 then an empty line will be returned.
pub fn build_line(game: &Game, move_index: usize) -> Vec<String> {
    // TODO(AdeAttwood): We can't have a value less than 0 with a usize
    if move_index <= 0 {
        return vec![];
    }

    if move_index >= game.history.len() {
        return game
            .history
            .iter()
            .map(|m| m.uci())
            .collect::<Vec<String>>();
    }

    game.history[0..move_index]
        .iter()
        .map(|m| m.uci())
        .collect::<Vec<String>>()
}

// pub fn analyze_move(
//     engine: &mut UciEngine,
//     board: &Board,
//     movement: ResolvedMovement,
// ) -> UciResult<MoveAnalysis> {
//     let prev_line = build_line(game, game.history.len());
//
//     println!("PrevLine: {:?}", prev_line);
//     let mut new_game = game.clone();
//     new_game.move_piece(movement);
//
//     let curr_line = build_line(new_game, new_game.history.len());
//     println!("CurrLine: {:?}", prev_line);
//
//     Ok(MoveAnalysis::GoodMove)
// }

fn main() -> UciResult<()> {
    let pgn_data = include_str!("../../pgn_data/game.pgn");
    let games = pgn::parse(&pgn_data.to_string()).unwrap();
    let game = &games[0];

    let mut engine = UciEngine::new("stockfish")?;
    let _ = engine.uci()?;

    let mut current_game = Game {
        board: Board::from_start_position().unwrap(),
        ..Game::default()
    };

    for (i, movement) in game.history.iter().enumerate() {
        engine.position(
            "startpos",
            &build_line(&current_game, current_game.history.len()),
        );
        let prev_analysis = engine.go_depth(MAX_DEPTH)?;

        current_game.move_piece(movement.clone());
        engine.position(
            "startpos",
            &build_line(&current_game, current_game.history.len()),
        );
        let curr_analysis = engine.go_depth(MAX_DEPTH)?;

        println!(
            "{:?} {:?}",
            prev_analysis.best_info()?.score,
            curr_analysis.best_info()?.score
        );
    }

    Ok(())
}
