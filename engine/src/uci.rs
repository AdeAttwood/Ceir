use crate::evaluation::evaluate;
use crate::search::Search;
use crate::transposition_table::TranspositionTable;
use crate::uci_command::{GoOptions, PositionOptions, UciCommand};
use common::bb;
use common::Board;
use common::Fen;
use common::Piece;
use common::ResolvedMovement;
use common::Square;

pub trait UciWriter {
    fn writeln(&mut self, output: &str);
}

pub struct UciOutputWriter {}
impl Default for UciOutputWriter {
    fn default() -> Self {
        Self::new()
    }
}

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
    transposition_table: TranspositionTable,
}

impl Default for Uci {
    fn default() -> Self {
        Self::new()
    }
}

impl Uci {
    pub fn new() -> Self {
        Self {
            board: Board::from_start_position().unwrap(),
            transposition_table: TranspositionTable::new(),
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
            UciCommand::NewGame => self.board = Board::from_start_position().unwrap(),
            UciCommand::IsReady => writer.writeln("readyok"),
            UciCommand::Print => self.print(writer),
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
            if s.len() != 4 && s.len() != 5 {
                writer.writeln(&format!("Invalid move '{s}'"));
                return;
            }

            let from = match Square::from_str(&s[0..2]) {
                Ok(s) => s,
                Err(m) => {
                    writer.writeln(&m);
                    return;
                }
            };

            let (color, piece) = match self.board.get_piece_at(&bb!(from)) {
                Some(p) => p,
                None => {
                    writer.writeln(&format!("There is no piece on the source square of {s}"));
                    return;
                }
            };

            if color != self.board.turn {
                writer.writeln(&format!("It is not {color}'s turn to move"));
                return;
            }

            let to = match Square::from_str(&s[2..4]) {
                Ok(s) => s,
                Err(m) => {
                    writer.writeln(&m);
                    return;
                }
            };

            let mut capture: Option<Piece> = None;
            if let Some((color, c)) = self.board.get_piece_at(&bb!(to)) {
                capture = Some(c);
                if color == self.board.turn {
                    writer.writeln("You can not capture your own piece");
                    return;
                }
            }

            let mut promotion: Option<Piece> = None;
            if s.len() == 5 {
                promotion = Piece::from_str(&s[4..5]);
            }

            self.board.move_piece(ResolvedMovement {
                piece,
                from,
                to,
                capture,
                promotion,
            });
        }
    }

    fn go(&mut self, writer: &mut dyn UciWriter, options: &GoOptions) {
        let mut search = Search::new(
            writer,
            &mut self.transposition_table,
            self.board,
            options.depth as usize,
        );
        search.search();

        self.transposition_table.clean();
    }

    fn uci(&self, writer: &mut dyn UciWriter) {
        writer.writeln("id name Ceir Development");
        writer.writeln("id author Ade Attwood");
        writer.writeln("uciok");
    }

    fn print(&self, writer: &mut dyn UciWriter) {
        self.board.print();
        let eval = evaluate(&self.board);
        writer.writeln(&format!("Eval: {eval}"))
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

    // position startpos moves e2e4
}
