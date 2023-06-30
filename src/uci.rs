use crate::bitboards::Square;
use crate::board::Board;
use crate::fen::Fen;
use crate::search::Search;
use crate::uci_command::{GoOptions, PositionOptions, UciCommand};

pub trait UciWriter {
    fn writeln(&mut self, output: &str);
}

pub struct UciOutputWriter {}
impl UciOutputWriter {
    pub fn new() -> Self {
        Self {}
    }
}

impl UciWriter for UciOutputWriter {
    fn writeln(&mut self, output: &str) {
        println!("{output}");
    }
}

pub struct Uci {
    board: Board,
}

impl Uci {
    pub fn new() -> Self {
        Self {
            board: Board::empty(),
        }
    }

    pub fn handle(&mut self, input: &String, writer: &mut dyn UciWriter) {
        let command: UciCommand = match input.try_into() {
            Ok(c) => c,
            Err(message) => {
                writer.writeln(message.as_str());
                return;
            }
        };

        match command {
            UciCommand::Uci => self.uci(writer),
            UciCommand::NewGame => self.board = Board::empty(),
            UciCommand::IsReady => writer.writeln("readyok"),
            UciCommand::Print => self.board.print(),
            UciCommand::Stop => std::process::exit(0),
            UciCommand::Position(options) => self.position(writer, &options),
            UciCommand::Go(options) => self.go(writer, &options),
        }
    }

    fn position(&mut self, writer: &mut dyn UciWriter, options: &PositionOptions) {
        let fen_string = if &options.position == "startpos" {
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()
        } else {
            options.position.clone()
        };

        let fen = match Fen::from_str(&fen_string) {
            Ok(fen) => fen,
            Err(message) => {
                writer.writeln(&message);
                return;
            }
        };

        self.board.load_fen(&fen);

        for m in &options.moves {
            let s = m.as_str();
            if s.len() != 4 {
                writer.writeln(&format!("Invalid move '{s}'"));
                return;
            }

            let from = match Square::try_from(&s[0..2]) {
                Ok(s) => s,
                Err(m) => {
                    writer.writeln(&m);
                    return;
                }
            };

            let to = match Square::try_from(&s[2..4]) {
                Ok(s) => s,
                Err(m) => {
                    writer.writeln(&m);
                    return;
                }
            };

            self.board.do_move(from, to);
        }
    }

    fn go(&self, writer: &mut dyn UciWriter, options: &GoOptions) {
        let mut search = Search::new(options.depth);

        let mut board = self.board.clone();
        search.search(&mut board, writer);
    }

    fn uci(&self, writer: &mut dyn UciWriter) {
        writer.writeln("id name ChessRs");
        writer.writeln("id author Ade Attwood");
        writer.writeln("uciok");
    }
}

#[cfg(test)]
mod tests {
    use crate::uci::*;

    pub struct UciTestWriter {
        pub lines: Vec<String>,
    }

    impl UciTestWriter {
        pub fn new() -> Self {
            Self { lines: Vec::new() }
        }
    }

    impl UciWriter for UciTestWriter {
        fn writeln(&mut self, output: &str) {
            self.lines.push(String::from(output));
        }
    }

    #[test]
    fn will_handle_uci_command() {
        let mut writer = UciTestWriter::new();
        let mut uci = Uci::new();

        uci.handle(&String::from("uci"), &mut writer);
        assert_eq!(writer.lines.len(), 3);
        assert!(writer.lines.join("\n").contains("uciok"))
    }

    #[test]
    fn will_handle_ready_ok_command() {
        let mut writer = UciTestWriter::new();
        let mut uci = Uci::new();

        uci.handle(&String::from("isready"), &mut writer);
        assert_eq!(writer.lines.len(), 1);
        assert_eq!(writer.lines[0], "readyok");
    }
}
