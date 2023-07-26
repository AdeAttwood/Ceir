use super::game::{PgnGame, PgnMove};
use super::game_result::GameResult;
use super::reader::PgnReader;

pub fn parse(reader: &mut PgnReader) -> Vec<PgnGame> {
    let mut games: Vec<PgnGame> = Vec::new();
    let mut game = PgnGame::new();

    while let Some(c) = reader.next() {
        match c {
            '[' => {
                let key = match reader.read_to(' ') {
                    Some(key) => key,
                    None => panic!("Unable to parse key value"),
                };

                // Take the first quote from the string, then read all the way to the next one
                reader.next();
                let value = match reader.read_to('"') {
                    Some(value) => value,
                    None => panic!("Unable to parse value value"),
                };

                game.meta.insert(key, value);
                reader.read_to_whitespace();
            }
            '0'..='9' => {
                if reader.peek().unwrap() == '-' || reader.peek().unwrap() == '/' {
                    match reader.read_to_whitespace().unwrap().as_str() {
                        "-1" => game.result = GameResult::WinBlack,
                        "-0" => game.result = GameResult::WinWhite,
                        "/2-1/2" => game.result = GameResult::Draw,
                        result => panic!("Unable to parse result {result}"),
                    };

                    games.push(game);
                    game = PgnGame::new();
                    continue;
                }

                let number = match reader.read_to('.') {
                    Some(n) => n,
                    None => panic!("Unable to read start of the move"),
                };

                reader.skip_whitespace();

                let mut index_str = String::from(c);
                index_str.push_str(number.as_str());

                let index = index_str
                    .parse::<i32>()
                    .expect(&format!("Unable to convert {index_str} to a number"));

                let white_move = reader.read_to_whitespace().expect("Expected from");
                let comment = match reader.peek().unwrap() {
                    '{' => {
                        reader.next();
                        let result = reader.read_to('}');
                        reader.skip_whitespace();

                        result
                    }
                    _ => None,
                };

                game.moves.push(PgnMove {
                    ply: index,
                    san: white_move,
                    comment,
                });

                let black_move_or_result = reader.read_to_whitespace().expect("Expected to");
                let result = match black_move_or_result.as_str() {
                    "0-1" => Some(GameResult::WinBlack),
                    "1-0" => Some(GameResult::WinBlack),
                    "1/2-1/2" => Some(GameResult::Draw),
                    "*" => Some(GameResult::InProgress),
                    _ => None,
                };

                if let Some(result) = result {
                    game.result = result;
                    games.push(game);
                    game = PgnGame::new();

                    continue;
                }

                let comment = match reader.peek().unwrap() {
                    '{' => {
                        reader.next();
                        let result = reader.read_to('}');
                        reader.skip_whitespace();

                        result
                    }
                    _ => None,
                };

                game.moves.push(PgnMove {
                    ply: index,
                    san: black_move_or_result,
                    comment,
                });
            }
            '*' => {
                game.result = GameResult::InProgress;
                games.push(game);
                game = PgnGame::new();
            }
            _ => {}
        }
    }

    games
}
