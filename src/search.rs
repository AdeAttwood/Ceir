use std::collections::HashMap;

use crate::bitboards::BitBoardIterator;
use crate::board::{Board, Color};
use crate::move_gen::MoveGen;
use crate::uci::UciWriter;

#[rustfmt::skip]
const PAWN_SCORE: [i32; 64] = [
    0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0, -10, -10,   0,   0,   0,
    0,   0,   0,   5,   5,   0,   0,   0,
    5,   5,   10,  20,  25,   5,   5,   5,
    10,  10,  10,  20,  20,  10,  10,  10,
    20,  20,  20,  30,  30,  30,  20,  20,
    30,  30,  30,  40,  40,  30,  30,  30,
    90,  90,  90,  90,  90,  90,  90,  90
];

#[derive(Debug, Clone)]
pub struct SearchNode {
    pub depth: i32,
    pub score: i32,
    pub nodes: i32,
    pub mate: i32,
    pub pv: String,
}

struct PvList {}

impl PvList {
    pub fn to_list(map: &HashMap<usize, SearchNode>) -> Vec<SearchNode> {
        let mut list = map.clone().into_values().collect::<Vec<SearchNode>>();
        list.sort_by_key(|node| node.depth);

        list
    }

    pub fn uci_bestmove(map: &HashMap<usize, SearchNode>) -> Result<String, String> {
        let list = PvList::to_list(map);

        let node = match list.first() {
            Some(node) => node,
            None => {
                return Err("Empty PV list".to_string());
            }
        };

        Ok(format!("bestmove {}", node.pv))
    }

    pub fn uci_info(map: &HashMap<usize, SearchNode>) -> Result<String, String> {
        let list = PvList::to_list(map);

        let last = match list.last() {
            Some(node) => node,
            None => {
                return Err("Empty PV list".to_string());
            }
        };

        let first = match list.first() {
            Some(node) => node,
            None => {
                return Err("Empty PV list".to_string());
            }
        };

        let line = list
            .iter()
            .map(|node| node.pv.clone())
            .collect::<Vec<String>>()
            .join(" ");

        Ok(format!(
            "info depth {} nodes {} score cp {} pv {}",
            last.depth, last.nodes, first.score, line
        ))
    }
}

pub struct Search {
    pub depth: i32,
    pub node_count: i32,
    pub pv_list: HashMap<usize, SearchNode>,
}

impl Search {
    pub fn new(depth: i32) -> Self {
        Self {
            depth,
            node_count: 0,
            pv_list: HashMap::new(),
        }
    }

    fn material_score(&self, current_board: &Board) -> i32 {
        let white_pieces = (current_board.white_pawn_board.count_ones() as i32 * 100)
            + (current_board.white_queen_board.count_ones() as i32 * 9 * 100)
            + (current_board.white_rook_board.count_ones() as i32 * 5 * 100)
            + (current_board.white_bishop_board.count_ones() as i32 * 3 * 100)
            + (current_board.white_knight_board.count_ones() as i32 * 3 * 100);

        let black_pieces = (current_board.black_pawn_board.count_ones() as i32 * 100)
            + (current_board.black_queen_board.count_ones() as i32 * 9 * 100)
            + (current_board.black_rook_board.count_ones() as i32 * 5 * 100)
            + (current_board.black_bishop_board.count_ones() as i32 * 3 * 100)
            + (current_board.black_knight_board.count_ones() as i32 * 3 * 100);

        white_pieces - black_pieces
    }

    pub fn evaluate(&mut self, board: &Board) -> i32 {
        let mut score = self.material_score(&board);

        let pawns = match board.turn {
            Color::Black => board.black_pawn_board.reverse_bits(),
            Color::White => board.white_pawn_board,
        };

        let mut it = BitBoardIterator::new(pawns);
        while let Some(index) = it.next() {
            score += PAWN_SCORE[index];
        }

        let offset = match board.turn {
            Color::White => 1,
            Color::Black => -1,
        };

        score * offset
    }

    #[allow(dead_code)]
    fn quiesce(&mut self, board: &Board, mut alpha: i32, beta: i32) -> i32 {
        let score = self.evaluate(board);
        if score >= beta {
            return beta;
        }

        if alpha < score {
            alpha = score;
        }

        let move_gen = MoveGen::new(&board);
        for m in &move_gen.pseudo_moves {
            if m.capture.is_none() {
                return alpha;
            }

            let mut new_board = board.clone();
            new_board.do_move(m.from, m.to);

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

    fn negamax(
        &mut self,
        board: &mut Board,
        depth: i32,
        ply: i32,
        mut alpha: i32,
        beta: i32,
    ) -> i32 {
        self.node_count += 1;

        if depth == 0 {
            // TODO(AdeAttwood): Work out how this work currently this is checking around 10000
            // more nodes that the base evaluate function.
            // return self.quiesce(&board, alpha, beta);
            return self.evaluate(&board);
        }

        let move_gen = MoveGen::new(&board);

        let mut moved = false;
        for m in &move_gen.pseudo_moves {
            let mut new_board = board.clone();
            new_board.do_move(m.from, m.to);

            let mg = MoveGen::new(&new_board);
            if mg.is_check || mg.checking {
                continue;
            }

            moved = true;

            let score = -self.negamax(&mut new_board, depth - 1, ply + 1, -beta, -alpha);

            if score >= beta {
                return beta;
            }

            if score > alpha {
                alpha = score;

                self.pv_list.insert(
                    ply as usize - 1,
                    SearchNode {
                        depth: ply,
                        score,
                        nodes: self.node_count,
                        mate: -1,
                        pv: m.format_move(),
                    },
                );

                // if self.node_count % 1000 == 0 {
                match PvList::uci_info(&self.pv_list) {
                    Ok(info) => println!("{info}"),
                    Err(..) => {}
                }
                // }
            }
        }

        if !moved {
            println!("------------------------------------------- MATE");
        }

        alpha
    }

    pub fn search(&mut self, board: &mut Board, writer: &mut dyn UciWriter) {
        self.negamax(board, self.depth, 1, -50000, 50000);

        match PvList::uci_bestmove(&self.pv_list) {
            Ok(bestmove) => writer.writeln(&bestmove),
            Err(_) => writer.writeln(&format!("Err: No move found")),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{bitboards::Square, fen::Fen};

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

    fn evaluate_fen(fen: &str) -> i32 {
        let mut board = Board::empty();
        let fen = Fen::from_str(fen).unwrap();
        board.load_fen(&fen);

        let mut search = Search::new(2);
        search.evaluate(&board)
    }

    #[test]
    fn evaluate_white() {
        let score = evaluate_fen("3kq3/8/8/8/8/8/8/3K4 w - - 0 1");
        assert_eq!(score, -900);
    }

    #[test]
    fn evaluate_black() {
        let score = evaluate_fen("3kq3/8/8/8/8/8/8/3K4 b - - 0 1");
        assert_eq!(score, 900);
    }

    #[test]
    fn will_get_out_of_check() {
        let mut writer = UciTestWriter::new();
        let mut board =
            Board::from_fen_str("r1br2k1/ppp2ppp/5n2/4n3/1bP5/2N1P3/P2K2PP/1RB2BNR w - - 4 12")
                .unwrap();

        let mut search = Search::new(2);
        search.search(&mut board, &mut writer);

        let bestmove = match PvList::to_list(&search.pv_list).first() {
            Some(m) => m.clone(),
            None => panic!("There should be a bestmove"),
        };

        let pv = bestmove.pv.clone();
        let s = pv.as_str();
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

        board.do_move(from, to);

        let move_gen = MoveGen::new(&board);
        assert!(!move_gen.is_check);
        assert!(!move_gen.checking);
    }
}
