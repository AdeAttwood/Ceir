#[derive(Debug, PartialEq)]
pub struct PositionOptions {
    pub position: String,
    pub moves: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct GoOptions {
    /// search x plies only.
    pub depth: i32,
    /// white has x msec left on the clock
    pub wtime: i32,
    /// black has x msec left on the clock
    pub btime: i32,
    /// white increment per move in mseconds if x > 0
    pub winc: i32,
    /// black increment per move in mseconds if x > 0
    pub binc: i32,
}

#[derive(Debug, PartialEq)]
pub enum UciCommand {
    Uci,
    NewGame,
    IsReady,
    Print,
    Stop,
    Position(PositionOptions),
    Go(GoOptions),
}

impl TryFrom<&String> for UciCommand {
    type Error = String;
    fn try_from(input: &String) -> Result<Self, Self::Error> {
        let mut tokens = input.trim().split_whitespace();
        let command = tokens.next();

        match command {
            Some("uci") => Ok(UciCommand::Uci),
            Some("ucinewgame") => Ok(UciCommand::NewGame),
            Some("isready") => Ok(UciCommand::IsReady),
            Some("stop") => Ok(UciCommand::Stop),
            Some("quit") => Ok(UciCommand::Stop),
            Some("d") => Ok(UciCommand::Print),
            Some("position") => {
                let mut options = PositionOptions {
                    position: String::from(""),
                    moves: Vec::new(),
                };

                while let Some(token) = tokens.next() {
                    match token {
                        "startpos" => options.position = String::from(token),
                        "fen" => {
                            let mut parts: Vec<String> = Vec::new();
                            for _ in 0..6 {
                                if let Some(fen_part) = tokens.next() {
                                    parts.push(String::from(fen_part));
                                } else {
                                    return Err(format!("Incomplete fen string"));
                                }
                            }

                            options.position = parts.join(" ")
                        }
                        "moves" => {
                            while let Some(m) = tokens.next() {
                                options.moves.push(String::from(m))
                            }
                        }
                        _ => return Err(format!("Unexpected token '{token}'")),
                    }
                }

                Ok(UciCommand::Position(options))
            }
            Some("go") => {
                let mut options = GoOptions {
                    depth: 4,
                    wtime: 0,
                    btime: 0,
                    binc: 0,
                    winc: 0,
                };

                while let Some(token) = tokens.next() {
                    match token {
                        "depth" => match tokens.next() {
                            Some(depth) => match depth.to_string().parse::<i32>() {
                                Ok(number) => options.depth = number,
                                Err(message) => return Err(message.to_string()),
                            },
                            None => return Err(format!("Missing depth")),
                        },
                        "wtime" => match tokens.next() {
                            Some(depth) => match depth.to_string().parse::<i32>() {
                                Ok(number) => options.wtime = number,
                                Err(message) => return Err(message.to_string()),
                            },
                            None => return Err(format!("Missing wtime value")),
                        },
                        "btime" => match tokens.next() {
                            Some(depth) => match depth.to_string().parse::<i32>() {
                                Ok(number) => options.btime = number,
                                Err(message) => return Err(message.to_string()),
                            },
                            None => return Err(format!("Missing btime value")),
                        },
                        "winc" => match tokens.next() {
                            Some(depth) => match depth.to_string().parse::<i32>() {
                                Ok(number) => options.winc = number,
                                Err(message) => return Err(message.to_string()),
                            },
                            None => return Err(format!("Missing winc value")),
                        },
                        "binc" => match tokens.next() {
                            Some(depth) => match depth.to_string().parse::<i32>() {
                                Ok(number) => options.binc = number,
                                Err(message) => return Err(message.to_string()),
                            },
                            None => return Err(format!("Missing binc value")),
                        },
                        _ => return Err(format!("Unexpected token {token}")),
                    }
                }

                Ok(UciCommand::Go(options))
            }
            Some(command) => Err(format!("Invalid command {command}")),
            None => Err("Missing command".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::uci_command::*;

    fn parse_command(command: &String) -> UciCommand {
        match command.try_into() {
            Ok(c) => c,
            Err(message) => panic!("{message}"),
        }
    }

    fn parse_command_error(command: &String) -> String {
        match UciCommand::try_from(command) {
            Ok(_) => panic!("Command should return an error"),
            Err(message) => message,
        }
    }

    #[test]
    fn will_parse_uci_command() {
        let command = parse_command(&String::from("uci"));
        assert_eq!(command, UciCommand::Uci);
    }

    #[test]
    fn will_parse_isready_command() {
        let command = parse_command(&String::from("isready"));
        assert_eq!(command, UciCommand::IsReady);
    }

    #[test]
    fn will_parse_start_pos() {
        let command = parse_command(&String::from("position startpos"));
        let position_options = match command {
            UciCommand::Position(options) => options,
            _ => panic!("Unable to get the options from the position"),
        };

        assert_eq!(position_options.position, "startpos");
    }

    #[test]
    fn will_load_a_fen() {
        let command = parse_command(&String::from(
            "position fen 2kr3r/ppp2pQp/4p3/8/Pb2b3/8/2P2PPP/R1B2RK1 b - - 0 16",
        ));
        let position_options = match command {
            UciCommand::Position(options) => options,
            _ => panic!("Unable to get the options from the position"),
        };

        assert_eq!(
            position_options.position,
            "2kr3r/ppp2pQp/4p3/8/Pb2b3/8/2P2PPP/R1B2RK1 b - - 0 16"
        );
    }

    #[test]
    fn will_parse_start_pos_with_missing_fen() {
        let error = parse_command_error(&String::from("position fen"));
        assert_eq!(error, "Incomplete fen string");
    }

    #[test]
    fn invalid_token_mov() {
        let error = parse_command_error(&String::from(
            "position fen 2kr3r/ppp2pQp/4p3/8/Pb2b3/8/2P2PPP/R1B2RK1 b - - 0 16 move",
        ));
        assert_eq!(error, "Unexpected token 'move'");
    }

    #[test]
    fn will_parse_start_pos_as_fen() {
        let command = parse_command(&String::from("position fen 1 2 3 4 5 6"));
        let position_options = match command {
            UciCommand::Position(options) => options,
            _ => panic!("Unable to get the options from the position"),
        };

        assert_eq!(position_options.position, "1 2 3 4 5 6");
    }

    #[test]
    fn will_parse_start_pos_as_fen_and_moves() {
        let command = parse_command(&String::from("position fen 1 2 3 4 5 6 moves e2e4"));
        let position_options = match command {
            UciCommand::Position(options) => options,
            _ => panic!("Unable to get the options from the position"),
        };

        assert_eq!(position_options.moves.len(), 1);
        assert_eq!(position_options.moves[0], "e2e4");
    }

    #[test]
    fn will_parse_go() {
        let command = parse_command(&String::from("go"));
        let go_options = match command {
            UciCommand::Go(options) => options,
            _ => panic!("Unable to get the options from the position"),
        };

        assert_eq!(go_options.depth, 4);
    }

    #[test]
    fn will_parse_go_with_depth() {
        let command = parse_command(&String::from("go depth 20"));
        let go_options = match command {
            UciCommand::Go(options) => options,
            _ => panic!("Unable to get the options from the position"),
        };

        assert_eq!(go_options.depth, 20);
    }

    #[test]
    fn will_parse_go_with_times() {
        let command = parse_command(&String::from("go wtime 300000 btime 300000 winc 0 binc 0"));
        let go_options = match command {
            UciCommand::Go(options) => options,
            _ => panic!("Unable to get the options from the position"),
        };

        assert_eq!(go_options.wtime, 300000);
    }

    #[test]
    fn will_parse_new_game_command() {
        let command = parse_command(&String::from("ucinewgame"));
        assert_eq!(command, UciCommand::NewGame);
    }
}
