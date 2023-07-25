use pgn::parser;
use pgn::reader::PgnReader;
use std::fs;

mod book;
mod pgn;

// https://sites.google.com/site/computerschess/download

fn main() {
    let paths = fs::read_dir("./pgn").unwrap();

    let mut game_count = 0;
    let mut book = book::Node::empty();

    for path in paths {
        let path = path.unwrap().path();
        let extension = path.extension();
        if !(extension.is_some() && extension.unwrap() == "pgn") {
            continue;
        }

        let mut reader = PgnReader::new(&path);
        let games = parser::parse(&mut reader);
        game_count += games.len();

        println!(
            "Adding {} to the book with {} games",
            &path.to_str().unwrap(),
            games.len()
        );
        for game in &games {
            book.add_line(&mut game.moves.clone(), &game.result);
        }
    }

    let encoded = bincode::serialize(&book).unwrap();
    fs::write("book.cbk", encoded).unwrap();

    println!("");
    println!("Book created with {game_count} games");
}
