use crate::uci::UciWriter;
use common::{
    attacked_squares, is_in_check, is_move_to_check, pseudo_moves, Board, ResolvedMovement,
};
use common::{castle_moves, BitBoardable};
use std::cmp::{max, min};

extern "C" {
    fn rand() -> usize;
    fn srand(seed: usize);
    fn time(pointer: usize) -> usize;
}

fn is_valid_move(board: &Board, movement: &ResolvedMovement) -> bool {
    let mut new_board = board.clone();
    new_board.move_piece(*movement);
    let attackers = attacked_squares(&new_board, &board.turn.opposite());

    // Check the board is not in check after the move
    if is_in_check(board, &attackers) {
        return false;
    };

    true
}

pub fn search(board: &Board, writer: &mut dyn UciWriter) -> Option<ResolvedMovement> {
    let mut moves = vec![
        pseudo_moves(&board),
        castle_moves(board, &attacked_squares(board, &board.turn)),
    ]
    .concat();

    unsafe {
        srand(time(0));
    }

    while !moves.is_empty() {
        let seed = unsafe { rand() };
        let mut index = seed % (moves.len() + 1 - 0) + 0;
        if index == moves.len() {
            index = index - 1;
        }

        let movement = moves.remove(index);

        if !is_valid_move(board, &movement) {
            continue;
        }

        writer.writeln(&format!("bestmove {}", movement.uci()));
        return Some(movement);
    }

    None
}
