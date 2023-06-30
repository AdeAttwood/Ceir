mod bitboards;
mod board;
mod fen;
mod move_gen;
mod moves;
mod search;
mod uci;
mod uci_command;

fn main() {
    let mut writer = uci::UciOutputWriter::new();
    let mut uci = uci::Uci::new();

    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        uci.handle(&line, &mut writer);
    }
}
