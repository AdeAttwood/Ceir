use serde::{Deserialize, Serialize};

use crate::pgn::{game::PgnMove, game_result::GameResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub san: String,
    wins: i32,
    losses: i32,
    ply_count: i32,

    next_move: Option<Box<Node>>,
    next_sibling: Option<Box<Node>>,
}

impl Node {
    pub fn empty() -> Self {
        Node {
            san: String::from(""),
            wins: 0,
            losses: 0,
            ply_count: 0,
            next_move: None,
            next_sibling: None,
        }
    }

    pub fn from_move(pgn_move: &PgnMove, result: &GameResult) -> Box<Self> {
        Box::new(Node {
            san: pgn_move.san.clone(),
            wins: if result.is_win(pgn_move.ply) { 1 } else { 0 },
            losses: if result.is_loss(pgn_move.ply) { 1 } else { 0 },
            ply_count: 1,
            next_move: None,
            next_sibling: None,
        })
    }

    // TODO(AdeAttwood): Add the book into the engine move gen
    #[allow(dead_code)]
    pub fn next_move(&self) -> Option<&Node> {
        let mut output = self;
        let mut current = self;
        while let Some(node) = &current.next_sibling {
            // println!("Node {}", node.wins / node.ply_count);
            if node.ply_count > output.ply_count {
                output = &node;
            }

            current = &node;
        }

        Some(output)
    }

    #[allow(dead_code)]
    fn print_impl(&self, depth: usize) {
        if self.ply_count > 0 {
            println!(
                "{}{} ({} / {} / {})",
                String::from(" . ").repeat(depth),
                self.san,
                self.wins,
                self.losses,
                self.ply_count
            );
        }

        match &self.next_move {
            Some(node) => node.print_impl(depth + 1),
            None => {}
        }

        match &self.next_sibling {
            Some(node) => node.print_impl(depth),
            None => {}
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        self.print_impl(0);
    }

    fn update_node(&mut self, current: &PgnMove, moves: &mut Vec<PgnMove>, result: &GameResult) {
        if current.san == self.san {
            if result.is_win(current.ply) {
                self.wins += 1;
            } else if result.is_loss(current.ply) {
                self.losses += 1;
            }

            self.ply_count += 1;
            self.add_next_move(moves, result);
        } else {
            self.add_sibling(&current, moves, result);
        }
    }

    fn add_next_move(&mut self, moves: &mut Vec<PgnMove>, result: &GameResult) {
        if moves.is_empty() {
            return;
        }

        let next = moves.remove(0);
        match &mut self.next_move {
            Some(node) => node.update_node(&next, moves, result),
            None => {
                self.next_move = Some(Node::from_move(&next, result));
                self.next_move
                    .as_mut()
                    .unwrap()
                    .add_next_move(moves, result);
            }
        }
    }

    fn add_sibling(&mut self, current: &PgnMove, moves: &mut Vec<PgnMove>, result: &GameResult) {
        match &mut self.next_sibling {
            Some(node) => node.update_node(&current, moves, result),
            None => {
                self.next_sibling = Some(Node::from_move(&current, result));
                self.next_sibling
                    .as_mut()
                    .unwrap()
                    .add_next_move(moves, result);
            }
        }
    }

    pub fn add_line(&mut self, moves: &mut Vec<PgnMove>, result: &GameResult) {
        if moves.is_empty() {
            return;
        }

        let current = moves.remove(0);
        self.add_sibling(&current, moves, result);
    }
}
