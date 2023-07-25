#[derive(Debug, PartialEq)]
pub enum GameResult {
    WinBlack,
    WinWhite,
    Draw,
    InProgress,
}

impl GameResult {
    pub fn is_win(&self, ply: i32) -> bool {
        let white_win = ply % 2 == 1 && self == &GameResult::WinWhite;
        let black_win = ply % 2 == 0 && self == &GameResult::WinBlack;

        white_win || black_win
    }

    pub fn is_loss(&self, ply: i32) -> bool {
        !self.is_win(ply) && (self == &GameResult::WinWhite || self == &GameResult::WinBlack)
    }
}

#[cfg(test)]
mod tests {
    use super::GameResult;

    #[test]
    fn will_be_a_white_win() {
        let result = GameResult::WinWhite;
        assert!(result.is_win(1));
        assert!(!result.is_win(2));
    }

    #[test]
    fn will_be_a_black_win() {
        let result = GameResult::WinBlack;
        assert!(!result.is_win(1));
        assert!(result.is_win(2));
    }

    #[test]
    fn will_be_a_loss_for_white() {
        let result = GameResult::WinBlack;
        assert!(result.is_loss(1));
        assert!(!result.is_loss(2));
    }

    #[test]
    fn will_not_be_a_win_if_its_a_draw() {
        let result = GameResult::Draw;

        assert!(!result.is_win(1));
        assert!(!result.is_win(2));

        assert!(!result.is_loss(1));
        assert!(!result.is_loss(2));
    }
}
