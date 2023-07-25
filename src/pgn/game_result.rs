#[derive(Debug, PartialEq)]
pub enum GameResult {
    WinBlack,
    WinWhite,
    Draw,
    InProgress,
}

impl GameResult {
    pub fn is_win(&self, ply: i32) -> bool {
        let white_win = ply % 2 == 0 && self == &GameResult::WinWhite;
        let black_win = ply % 2 == 1 && self == &GameResult::WinBlack;

        white_win || black_win
    }

    pub fn is_loss(&self, ply: i32) -> bool {
        !self.is_win(ply) && (self == &GameResult::WinWhite && self == &GameResult::WinBlack)
    }
}

// TODO(AdeAttwood): Add tests
