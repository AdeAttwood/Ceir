use crate::bitboards::BitBoardIterator;
use crate::board::{Board, Color, Piece};
use crate::move_gen::MoveGen;
use crate::moves::Move;
use crate::uci::UciWriter;

const MAX_POSITIVE: i32 = 500000;
const MAX_NEGATIVE: i32 = -500000;
const MATE_SCORE: i32 = 400000;

#[rustfmt::skip]
const PAWN_SCORE: [i32; 64] = [
    0,  0,  0,   0,   0,   0,   0,  0,
    50, 50, 50,  50,  50,  50,  50, 50,
    10, 10, 20,  30,  30,  20,  10, 10,
    5,  5,  10,  25,  25,  10,  5,  5,
    0,  0,  0,   20,  20,  0,   0,  0,
    5,  -5, -10, 0,   0,   -10, -5, 5,
    5,  10, 10,  -20, -20, 10,  10, 5,
    0,  0,  0,   0,   0,   0,   0,  0
];

#[rustfmt::skip]
const KNIGHT_SCORE: [i32; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

#[rustfmt::skip]
const BISHOP_SCORE: [i32; 64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20,
];

#[rustfmt::skip]
const ROOK_SCORE: [i32; 64] = [
     0,  0,  0,  0,  0,  0,  0,  0,
     5, 10, 10, 10, 10, 10, 10,  5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
     0,  0,  0,  5,  5,  0,  0,  0
];

#[rustfmt::skip]
const QUEEN_SCORE: [i32; 64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
     -5,  0,  5,  5,  5,  5,  0, -5,
      0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20
];

#[rustfmt::skip]
const KING_SCORE: [i32; 64] = [
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
     20, 20,  0,  0,  0,  0, 20, 20,
     20, 30, 10,  0,  0, 10, 30, 20
];

#[derive(Debug, Clone)]
pub struct SearchNode {
    pub depth: i32,
    pub score: i32,
    pub nodes: i32,
    pub mate: i32,
    pub pv: String,
}

impl SearchNode {
    pub fn empty() -> Self {
        Self {
            depth: 0,
            score: 0,
            nodes: 0,
            mate: 0,
            pv: String::from(""),
        }
    }
}

struct PvList {}

impl PvList {
    pub fn to_list(map: &Vec<Vec<SearchNode>>) -> Vec<SearchNode> {
        map.first().unwrap().to_vec()
    }

    pub fn uci_bestmove(map: &Vec<Vec<SearchNode>>) -> Result<String, String> {
        let list = PvList::to_list(map);

        let node = match list.first() {
            Some(node) => node,
            None => {
                return Err("Empty PV list".to_string());
            }
        };

        Ok(format!("bestmove {}", node.pv))
    }

    pub fn uci_info(map: &Vec<Vec<SearchNode>>, nodes: i32) -> Result<String, String> {
        let list = map.first().unwrap();

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
            .filter_map(|node| {
                if first.mate > 0 && node.mate < 0 {
                    None
                } else {
                    Some(node.pv.clone())
                }
            })
            .collect::<Vec<String>>();

        let score_unit = if first.mate > 0 { "mate" } else { "cp" };
        let score_value = if first.mate > 0 {
            line.len() as i32
        } else {
            first.score
        };

        Ok(format!(
            "info depth {} nodes {} score {score_unit} {score_value} pv {}",
            last.depth,
            nodes,
            line.join(" ")
        ))
    }
}

pub struct Search {
    pub depth: i32,
    pub node_count: i32,
    pub pv_length: Vec<usize>,
    pub pv_list: Vec<Vec<SearchNode>>,
}

impl Search {
    pub fn new(depth: i32) -> Self {
        let mut search = Self {
            depth,
            node_count: 0,
            pv_length: Vec::new(),
            pv_list: Vec::new(),
        };

        let mut inner: Vec<SearchNode> = Vec::new();

        for _i in 0..depth + 2 {
            search.pv_length.push(0);
        }

        for _i in 0..depth {
            inner.push(SearchNode::empty());
        }

        for _j in 0..depth {
            search.pv_list.push(inner.clone());
        }

        search
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

        let offset = match current_board.turn {
            Color::White => 1,
            Color::Black => -1,
        };

        (white_pieces - black_pieces) * offset
    }

    pub fn update_pv_list(&mut self, ply: i32, score: i32, m: &Move) {
        let ply_index: usize = ply as usize - 1;
        self.pv_list[ply_index][ply_index] = SearchNode {
            depth: ply,
            score,
            nodes: self.node_count,
            mate: if score == MATE_SCORE || score == -MATE_SCORE {
                ply
            } else {
                -1
            },
            pv: m.format_move(),
        };

        self.pv_length[ply_index] = ply as usize;
        for i in ply as usize..self.pv_length[ply_index + 1] {
            self.pv_list[ply_index][i] = self.pv_list[ply_index + 1][i].clone();
            self.pv_length[ply_index] = self.pv_length[ply_index + 1];
        }
    }

    pub fn evaluate(&mut self, board: &Board) -> i32 {
        let mut score = self.material_score(&board);

        let bitboards = match board.turn {
            Color::White => board.white_boards(),
            Color::Black => board.black_boards(),
        };

        for (color, piece, bitboard) in bitboards {
            let mut it = BitBoardIterator::new(if color == Color::White {
                bitboard
            } else {
                bitboard.reverse_bits()
            });

            while let Some(index) = it.next() {
                score += match piece {
                    Piece::Pawn => PAWN_SCORE[index],
                    Piece::Rook => ROOK_SCORE[index],
                    Piece::Queen => QUEEN_SCORE[index],
                    Piece::Bishop => BISHOP_SCORE[index],
                    Piece::Knight => KNIGHT_SCORE[index],
                    Piece::King => KING_SCORE[index],
                }
            }
        }

        score
    }

    fn quiesce(&mut self, board: &Board, mut alpha: i32, beta: i32) -> i32 {
        let move_gen = MoveGen::new(&board);
        let score = self.evaluate(board);

        if score >= beta {
            return beta;
        }

        if alpha < score {
            alpha = score;
        }

        for m in &move_gen.pseudo_moves {
            if m.capture.is_none() {
                return alpha;
            }

            let mut new_board = board.clone();
            new_board.do_move(m);

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
            return self.quiesce(&board, alpha, beta);
        }

        let move_gen = MoveGen::new(&board);
        let mut moved = false;

        for m in &move_gen.pseudo_moves {
            let mut new_board = board.clone();
            new_board.do_move(m);

            let move_gen = MoveGen::new(&new_board);
            if move_gen.is_in_check(new_board.turn.opposite()) {
                continue;
            }

            moved = true;

            let score = -self.negamax(&mut new_board, depth - 1, ply + 1, -beta, -alpha);

            if score >= beta {
                return beta;
            }

            if score > alpha {
                alpha = score;
                self.update_pv_list(ply, score, &m);

                if self.node_count % 1000 == 0 {
                    match PvList::uci_info(&self.pv_list, self.node_count) {
                        Ok(info) => println!("{info}"),
                        Err(..) => {}
                    }
                }
            }
        }

        if !moved {
            return -MATE_SCORE;
        }

        alpha
    }

    pub fn search(&mut self, board: &mut Board, writer: &mut dyn UciWriter) {
        self.negamax(board, self.depth, 1, MAX_NEGATIVE, MAX_POSITIVE);

        match PvList::uci_info(&self.pv_list, self.node_count) {
            Ok(info) => writer.writeln(&info),
            Err(_) => {}
        }

        match PvList::uci_bestmove(&self.pv_list) {
            Ok(bestmove) => writer.writeln(&bestmove),
            Err(_) => writer.writeln(&format!("Err: No move found")),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{bitboards::Square, fen::Fen, moves::Move};

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
        assert_eq!(score, -950);
    }

    #[test]
    fn evaluate_black() {
        let score = evaluate_fen("3kq3/8/8/8/8/8/8/3K4 b - - 0 1");
        assert_eq!(score, 845);
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

        let piece = match board.piece_at(from) {
            Some(p) => p,
            None => {
                writer.writeln(&format!("There is no piece on the source square of {s}"));
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

        let m = Move {
            piece,
            from,
            to,
            capture: board.piece_at(to),
        };

        board.do_move(&m);

        let move_gen = MoveGen::new(&board);
        assert!(!move_gen.is_check);
    }

    // #[test]
    // fn will_find_a_lader_mate() {
    //     let mut writer = UciTestWriter::new();
    //     let mut board = Board::from_fen_str("5k2/8/R7/1R6/8/8/4K3/8 w - - 14 8").unwrap();
    //
    //     let mut search = Search::new(6);
    //     search.search(&mut board, &mut writer);
    //
    //     let info = match PvList::uci_info(&search.pv_list, 0) {
    //         Ok(m) => m.clone(),
    //         Err(message) => panic!("{message}"),
    //     };
    //
    //     assert!(info.ends_with("score mate 3 pv b5b7 f8g8 a6a8"));
    // }

    #[test]
    fn will_find_another_lader_mate() {
        let mut writer = UciTestWriter::new();
        let mut board = Board::from_fen_str("5k2/8/8/8/7R/R7/8/4K3 w - - 0 1").unwrap();

        let mut search = Search::new(6);
        search.search(&mut board, &mut writer);

        let info = match PvList::uci_info(&search.pv_list, 0) {
            Ok(m) => m.clone(),
            Err(message) => panic!("{message}"),
        };

        assert!(info.ends_with("score mate 5 pv a3a7 f8g8 h4h1 g8f8 h1h8"));
    }
}
