use common::AmbiguousMovement;
use common::Board;
use common::Color;
use common::Game;
use common::GameResult;
use common::Movement;
use common::Piece;
use common::Square;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "pgn.pest"]
struct PGNParser;

fn parse_san_move(pair: Pair<Rule>) -> Result<AmbiguousMovement, String> {
    if pair.as_rule() != Rule::san_move {
        return Err("Unable to parse rule, its not a san move".to_string());
    }

    for item in pair.into_inner() {
        match item.as_rule() {
            Rule::pawn_move => {
                let mut item_itr = item.into_inner();
                let file = item_itr.next().unwrap().as_str();
                let rank = item_itr.next().unwrap().as_str();

                return Ok(AmbiguousMovement {
                    file: None,
                    piece: Some(Piece::Pawn),
                    from: None,
                    to: Square::from_file_and_rank_str(file, rank)?,
                    capture: None,
                    promotion: None,
                });
            }
            Rule::pice_move => {
                let mut item_itr = item.into_inner();
                let piece = item_itr.next().unwrap().as_str();
                let file = item_itr.next().unwrap().as_str();
                let rank = item_itr.next().unwrap().as_str();

                return Ok(AmbiguousMovement {
                    file: None,
                    piece: match Piece::from_str(piece) {
                        Some(p) => Some(p),
                        None => return Err(format!("Invalid piece '{}'", piece)),
                    },
                    from: None,
                    to: Square::from_file_and_rank_str(file, rank)?,
                    capture: None,
                    promotion: None,
                });
            }
            Rule::disambiguous_move => {
                // disambiguous_move = { pice ~ (file | rank) ~ file ~ rank }
                let mut item_itr = item.into_inner();
                let piece_str = item_itr.next().unwrap().as_str();
                let file_or_rank = item_itr.next().unwrap();
                let file = item_itr.next().unwrap().as_str();
                let rank = item_itr.next().unwrap().as_str();

                return Ok(AmbiguousMovement {
                    file: Some(file_or_rank.as_str().chars().next().unwrap()),
                    piece: match Piece::from_str(piece_str) {
                        Some(p) => Some(p),
                        None => return Err(format!("Invalid piece '{}'", piece_str)),
                    },
                    from: None,
                    to: Square::from_file_and_rank_str(file, rank)?,
                    capture: None,
                    promotion: None,
                });
            }
            _ => unreachable!("Parse error, invalid san move item {:?}", item.as_rule()),
        }
    }

    unreachable!("Parse error, unable to parse san move");
}

fn parse_san(board: &Board, pair: Pair<Rule>) -> Result<AmbiguousMovement, String> {
    if pair.as_rule() != Rule::san {
        return Err("Unable to parse rule, its not a san".to_string());
    }

    let mut movement = AmbiguousMovement::default();

    for item in pair.into_inner() {
        match item.as_rule() {
            Rule::san_move => {
                movement = parse_san_move(item)?;
            }
            Rule::kings_side_castle => {
                if board.turn == Color::White {
                    movement = AmbiguousMovement {
                        file: None,
                        piece: Some(Piece::King),
                        from: Some(Square::E1),
                        to: Square::G1,
                        capture: None,
                        promotion: None,
                    }
                } else {
                    movement = AmbiguousMovement {
                        file: None,
                        piece: Some(Piece::King),
                        from: Some(Square::E8),
                        to: Square::G8,
                        capture: None,
                        promotion: None,
                    }
                }
            }
            Rule::capture => {
                let mut item_itr = item.into_inner();
                let file_or_piece = item_itr.next().unwrap();
                let _unused_capture_marker = item_itr.next();
                let file = item_itr.next().unwrap().as_str();
                let rank = item_itr.next().unwrap().as_str();

                let to_square = Square::from_file_and_rank_str(file, rank)?;

                movement = match file_or_piece.as_rule() {
                    Rule::pice => AmbiguousMovement {
                        file: None,
                        piece: Some(Piece::from_str(file_or_piece.as_str()).unwrap()),
                        from: None,
                        to: to_square,
                        capture: None,
                        promotion: None,
                    },
                    // TODO(AdeAttwood): Find a way to resolve the piece that is on this file.
                    Rule::file => AmbiguousMovement {
                        file: Some(file_or_piece.as_str().chars().next().unwrap()),
                        piece: Some(Piece::Pawn),
                        from: None,
                        to: to_square,
                        capture: None,
                        promotion: None,
                    },
                    _ => unreachable!("Unexpected capture expected a file or a piece"),
                };
            }
            Rule::promotion => {
                let piece_str = item.into_inner().next().unwrap().as_str();
                movement.promotion = Piece::from_str(piece_str);
            }
            Rule::suffix => {
                // { ("+" | "#" | "?" | "!")+ }
            }
            _ => unreachable!("Parse error, invalid san item {:?}", item.as_rule()),
        }
    }

    Ok(movement)
}

fn parse_move(board: &Board, pair: Pair<Rule>) -> Result<(i32, Movement), String> {
    if pair.as_rule() != Rule::pgn_move {
        return Err("Unable to parse rule, its not a move".to_string());
    }

    let mut number: i32 = 0;
    for item in pair.into_inner() {
        match item.as_rule() {
            Rule::move_number => {
                number = match item.as_str().parse::<i32>() {
                    Ok(number) => number,
                    Err(e) => panic!("Invalid move number: {}", e),
                };
            }
            Rule::single_san => {
                let movement = parse_san(board, item.into_inner().next().unwrap())?;
                return Ok((number, Movement::Ambiguous(movement)));
            }
            _ => unreachable!("Parse error, invalid move item {:?}", item.as_rule()),
        }
    }

    unreachable!("Parse error, expected a move");
}

fn parse_result(pair: Pair<Rule>) -> Result<GameResult, String> {
    if pair.as_rule() != Rule::result {
        return Err("Unable to parse rule, its not a result".to_string());
    }

    let result_rule = pair.into_inner().next().unwrap().as_rule();
    match result_rule {
        Rule::white_win => Ok(GameResult::WhiteWin),
        Rule::black_win => Ok(GameResult::BlackWin),
        Rule::draw => Ok(GameResult::Draw),
        _ => unreachable!("Parse error, invalid game result {:?}", result_rule),
    }
}

fn parse_game(pair: Pair<Rule>) -> Result<Game, String> {
    if pair.as_rule() != Rule::game {
        return Err("Unable to parse rule, its not a game".to_string());
    }

    let mut game = Game::default();

    // TODO(AdeAttwood): Find a way to get the start pos from the metadata
    game.board = Board::from_start_position()?;

    for item in pair.into_inner() {
        match item.as_rule() {
            Rule::meta => {
                let mut meta_itr = item.into_inner();
                let key = meta_itr.next().unwrap().as_str();
                let value = meta_itr
                    .next()
                    .into_iter()
                    .next()
                    .unwrap()
                    .into_inner()
                    .as_str();

                game.metadata.insert(key.to_string(), value.to_string());
            }
            Rule::pgn_move => {
                let (_ /* index */, movement) = parse_move(&game.board, item)?;
                let resolved_move = match movement {
                    Movement::Ambiguous(ambiguous) => ambiguous.resolve(&game.board)?,
                    Movement::Resolved(resolved) => resolved,
                };

                game.move_piece(resolved_move);
            }
            Rule::result => game.result = parse_result(item)?,
            _ => unreachable!("Unexpected game item: {:?}", item.as_rule()),
        }
    }

    Ok(game)
}

pub fn parse(string: &String) -> Result<Vec<Game>, String> {
    let parsed = match PGNParser::parse(Rule::game, string) {
        Ok(parsed) => parsed,
        Err(e) => return Err(format!("{}", e)),
    };

    let mut games: Vec<Game> = Vec::new();

    for pair in parsed {
        match pair.as_rule() {
            Rule::game => match parse_game(pair) {
                Ok(game) => games.push(game),
                Err(err) => return Err(err),
            },
            _ => unreachable!("Unexpected rule: {:?}", pair.as_rule()),
        }
    }

    Ok(games)
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn it_works() {
        let input =
            std::fs::read_to_string(format!("{}/data/first.pgn", env!("CARGO_MANIFEST_DIR")))
                .unwrap();

        let games = parse(&input).unwrap();
        assert_eq!(games.len(), 1);
    }
}
