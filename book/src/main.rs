use common::{AmbiguousMovement, Board, Color, Game, GameResult, Piece, ResolvedMovement, Square};
use pgn::parse;

use cli::{deindent, ArgBuilder};

use std::collections::HashMap;
use std::io::{BufReader, Read};

#[derive(Debug, Clone)]
pub struct PolyglotEntry {
    pub mv: u16,
    pub weight: u16,
    pub learn: u32,
}

impl PolyglotEntry {
    fn from_bytes(bytes: &[u8]) -> Self {
        let mv = u16::from_be_bytes(bytes[0..2].try_into().unwrap());
        let weight = u16::from_be_bytes(bytes[2..4].try_into().unwrap());
        let learn = u32::from_be_bytes(bytes[4..8].try_into().unwrap());

        PolyglotEntry { mv, weight, learn }
    }

    fn from_move(movement: &ResolvedMovement) -> Self {
        let promotion = match movement.promotion {
            Some(Piece::Queen) => 4,
            Some(Piece::Rook) => 3,
            Some(Piece::Bishop) => 2,
            Some(Piece::Knight) => 1,
            _ => 0,
        };

        let from = movement.from as usize;
        let to = movement.to as usize;
        let mv = (promotion << 12 | from << 6 | to) as u16;

        PolyglotEntry {
            mv,
            weight: 0,
            learn: 0,
        }
    }

    pub fn ambiguous_move(&self) -> AmbiguousMovement {
        let from = Square::from_usize((self.mv >> 6 & common::random::SQ_MASK) as usize);
        let to = Square::from_usize((self.mv & common::random::SQ_MASK) as usize);
        let promotion = match self.mv >> 12 {
            0 => None,
            1 => Some(Piece::Knight),
            2 => Some(Piece::Bishop),
            3 => Some(Piece::Rook),
            4 => Some(Piece::Queen),
            _ => panic!("Invalid promotion_piece"),
        };

        AmbiguousMovement {
            file: None,
            piece: None,
            from: Some(from),
            to,
            capture: None,
            promotion,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PolyglotBook {
    pub entries: HashMap<u64, Vec<PolyglotEntry>>,
}

impl PolyglotBook {
    pub fn insert(&mut self, key: u64, entry: PolyglotEntry) {
        match self.entries.get_mut(&key) {
            Some(entries) => {
                for e in entries.iter_mut() {
                    if e.mv == entry.mv {
                        e.weight += entry.weight;
                        e.learn = entry.learn;
                        return;
                    }
                }

                entries.push(entry);
            }
            None => {
                self.entries.insert(key, vec![entry]);
            }
        }
    }

    pub fn load_bin_file(&mut self, path: &str) -> Result<(), String> {
        let file = std::fs::File::open(path).map_err(|e| e.to_string())?;

        let mut reader = BufReader::new(file);
        let mut buffer = [0u8; 16];

        while reader.read_exact(&mut buffer).is_ok() {
            let key = u64::from_be_bytes(buffer[0..8].try_into().unwrap());
            let entry = PolyglotEntry::from_bytes(&buffer[8..]);

            self.insert(key, entry);
        }

        Ok(())
    }

    pub fn load_png_file(&mut self, path: &str) -> Result<(), String> {
        let mut file = std::fs::File::open(path).map_err(|e| e.to_string())?;
        let mut buf = vec![];
        file.read_to_end(&mut buf).map_err(|e| e.to_string())?;

        let content = String::from_utf8_lossy(&buf).into_owned();
        let games = parse(&content).map_err(|e| e.to_string())?;

        for game in games {
            let mut board = Board::from_start_position().unwrap();
            for item in game.history {
                let key = board.hash().unwrap();
                let mut entry = PolyglotEntry::from_move(&item);

                match game.result {
                    GameResult::WhiteWin => {
                        if board.turn == Color::White {
                            entry.weight = 2
                        }
                    }
                    GameResult::BlackWin => {
                        if board.turn == Color::Black {
                            entry.weight = 2
                        }
                    }
                    _ => entry.weight = 0,
                }

                self.insert(key, entry);
                board.move_piece(item);
            }
        }

        Ok(())
    }
}

struct Args {
    pub files: Vec<String>,
}

impl Args {
    fn new() -> Result<Self, String> {
        let args = ArgBuilder::new(std::env::args().collect());
        if args.bool("-h", "--help") {
            return Err(deindent(
                r#"
                    Usage: book [OPTIONS]

                        --help, -h         Display this help message
                        --file, -f <file>  Specify the path to the polyglot book
                "#,
            ));
        }

        Ok(Args {
            files: args.string_list("-f", "--file")?,
        })
    }
}

fn main() {
    let args = match Args::new() {
        Ok(args) => args,
        Err(e) => {
            println!("{e}");
            return;
        }
    };

    let mut book = PolyglotBook::default();
    for file in args.files.iter() {
        let load_result = match std::path::Path::new(file).extension() {
            Some(ext) if ext == "bin" => book.load_bin_file(file),
            Some(ext) if ext == "pgn" => book.load_png_file(file),
            Some(ext) => Err(format!("Invalid file extension {}", ext.to_string_lossy())),
            None => Err(format!("Unable to determine the file type of {file}")),
        };

        match load_result {
            Ok(_) => {}
            Err(e) => {
                println!("Error loading {file}\n    {e}");
                return;
            }
        }
    }

    let mut game = Game::default();
    game.board = Board::from_start_position().unwrap();

    let hash = game.board.hash().unwrap();
    if let Some(entries) = book.entries.get(&hash) {
        for entry in entries {
            let movement = entry.ambiguous_move();
            println!("{}: {:?}", entry.weight, movement);
        }
    }
}
