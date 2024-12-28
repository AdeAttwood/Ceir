use std::collections::HashMap;

use common::{Board, ResolvedMovement};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Bound {
    Exact,
    LowerBound,
    UpperBound,
}

#[derive(Clone, Debug)]
pub struct TTEntry {
    pub depth: usize,
    pub value: i32,
    pub movement: Option<ResolvedMovement>,
    pub bound: Bound,
    pub seen: i32,
}

#[derive(Clone, Debug)]
pub struct TranspositionTable {
    pub table: HashMap<u64, TTEntry>,
}

impl TranspositionTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    pub fn store(&mut self, key: u64, entry: TTEntry) {
        if let Some(found) = self.table.get_mut(&key) {
            found.depth = entry.depth;
            found.value = entry.value;
            found.bound = entry.bound;
            found.movement = entry.movement;
            found.seen += entry.seen;
        } else {
            self.table.insert(key, entry);
        }
    }

    pub fn retrieve(&self, key: u64) -> Option<&TTEntry> {
        self.table.get(&key)
    }

    pub fn get_pv(&self, board: &Board) -> Vec<ResolvedMovement> {
        let mut pv = Vec::new();
        let mut current_pos = board.clone();

        while let Some(entry) = self.retrieve(current_pos.hash().unwrap()) {
            if let Some(best_move) = entry.movement {
                pv.push(best_move);
                current_pos.move_piece(best_move);
            } else {
                break;
            }

            if pv.len() > 1000 {
                break;
            }
        }

        pv
    }

    pub fn clean(&mut self) {
        self.table.retain(|_, v| v.seen < 3)
    }

    pub fn uci_best_move(&self, board: &Board) -> Result<String, String> {
        let list = self.get_pv(board);
        match list.first() {
            Some(node) => Ok(format!("bestmove {}", node.uci())),
            None => Err("Empty PV list".to_string()),
        }
    }

    pub fn uci_info(&self, board: &Board, nodes: i32) -> Result<String, String> {
        let node = match self.retrieve(board.hash().unwrap()) {
            Some(node) => node,
            None => return Err("No entry for this node".to_string()),
        };

        let list = self.get_pv(board);

        let score_unit = "cp"; // if last.mate { "mate" } else { "cp" };;
        let score_value = node.value; //  if last.mate {
                                      //     list.len() as i32
                                      // } else {
                                      //     last.score
                                      // };

        Ok(format!(
            "info depth {} nodes {} score {score_unit} {score_value} pv {}",
            list.len(),
            nodes,
            list.iter()
                .map(|node| node.uci())
                .collect::<Vec<_>>()
                .join(" ")
        ))
    }
}
