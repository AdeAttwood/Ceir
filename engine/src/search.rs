use crate::transposition_table::{Bound, TTEntry, TranspositionTable};
use crate::{evaluation, move_sort, uci::UciWriter};
use common::{attacked_squares, castle_moves, pseudo_moves, Board, Color, ResolvedMovement};

const MAX_POSITIVE: i32 = 500000;
const MAX_NEGATIVE: i32 = -500000;
const MATE_SCORE: i32 = 400000;

pub struct Search<'a, T: UciWriter + ?Sized> {
    pub writer: &'a mut T,
    start_pos: Board,
    max_depth: usize,
    nodes: i32,
    transposition_table: &'a mut TranspositionTable,
}

impl<'a, T: UciWriter + ?Sized> Search<'a, T> {
    pub fn new(
        writer: &'a mut T,
        transposition_table: &'a mut TranspositionTable,
        start_pos: Board,
        max_depth: usize,
    ) -> Search<'a, T> {
        Self {
            writer,
            start_pos,
            max_depth,
            nodes: 0,
            transposition_table,
        }
    }

    pub fn search(&mut self) {
        let mut board = self.start_pos.clone();
        self.nega_max(
            &mut board,
            &Vec::new(),
            self.max_depth,
            MAX_NEGATIVE,
            MAX_POSITIVE,
        );

        self.writer.writeln(
            &self
                .transposition_table
                .uci_info(&board, self.nodes)
                .unwrap(),
        );

        self.writer
            .writeln(&self.transposition_table.uci_best_move(&board).unwrap());
    }

    fn nega_max(
        &mut self,
        board: &mut Board,
        line: &Vec<ResolvedMovement>,
        depth: usize,
        mut alpha: i32,
        mut beta: i32,
    ) -> i32 {
        self.nodes += 1;

        if let Some(entry) = self.transposition_table.retrieve(board.hash().unwrap()) {
            if entry.depth >= depth {
                match entry.bound {
                    Bound::Exact => return entry.value,
                    Bound::LowerBound => alpha = alpha.max(entry.value),
                    Bound::UpperBound => beta = beta.min(entry.value),
                }

                if alpha >= beta {
                    return entry.value;
                }
            }
        }

        if depth == 0 {
            return self.quiesce(board, alpha, beta);
        }

        let mut moved = false;
        let mut moves = vec![
            pseudo_moves(&board),
            castle_moves(board, &attacked_squares(board, &board.turn)),
        ]
        .concat();

        let mut best_value = i32::MIN;
        let mut best_move = None;

        moves.sort_by_key(|m| move_sort::sort_key(&m));

        for movement in &moves {
            let mut new_board = board.clone();
            new_board.move_piece(*movement);

            let attackers = attacked_squares(&new_board, &new_board.turn);
            let king = match new_board.turn {
                Color::White => new_board.black_king_board,
                Color::Black => new_board.white_king_board,
            };

            // We are in check and its not a valid move
            if attackers & king != 0 {
                continue;
            }

            moved = true;

            let mut new_line = line.clone();
            new_line.push(*movement);

            let score = -self.nega_max(&mut new_board, &new_line, depth - 1, -beta, -alpha);

            if score > best_value {
                best_value = score;
                best_move = Some(*movement);
            }

            alpha = alpha.max(score);

            if score >= beta {
                return beta;
            }
        }

        let mated_value = -MATE_SCORE + (depth as i32);

        let entry = TTEntry {
            seen: 1,
            depth,
            value: if !moved { mated_value } else { best_value },
            movement: best_move,
            bound: if best_value <= alpha {
                Bound::UpperBound
            } else if best_value >= beta {
                Bound::LowerBound
            } else {
                Bound::Exact
            },
        };

        self.transposition_table.store(board.hash().unwrap(), entry);

        if !moved {
            return mated_value;
        }

        alpha
    }

    fn quiesce(&mut self, board: &Board, mut alpha: i32, beta: i32) -> i32 {
        let score = evaluation::evaluate(board);

        if score >= beta {
            return beta;
        }

        if alpha < score {
            alpha = score;
        }

        let moves = vec![
            pseudo_moves(&board),
            castle_moves(board, &attacked_squares(board, &board.turn)),
        ]
        .concat();

        for movement in &moves {
            if movement.capture.is_none() {
                return alpha;
            }

            let mut new_board = board.clone();
            new_board.move_piece(*movement);

            let score = -self.quiesce(&new_board, -beta, -alpha);

            if score >= beta {
                return beta;
            }

            if score > alpha {
                alpha = score;
            }
        }

        alpha
    }
}
