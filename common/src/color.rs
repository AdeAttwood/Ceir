use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Color {
    #[default]
    White,
    Black,
}

impl Color {
    /// Returns the opposite color.
    ///
    /// For example, if the color is `Color::White` it will return `Color::Black`. If the color is
    /// `Color::Black` it will return `Color::White`.
    ///
    /// ```
    /// use common::Color;
    ///
    /// assert_eq!(Color::White.opposite(), Color::Black);
    /// assert_eq!(Color::Black.opposite(), Color::White);
    /// ```
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Color::Black => write!(f, "black"),
            Color::White => write!(f, "white"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn will_create_the_default_color() {
        assert_eq!(Color::default(), Color::White);
    }

    #[test]
    fn gets_the_opposite_color() {
        let black = Color::Black;
        let white = Color::White;

        assert_eq!(black.opposite(), Color::White);
        assert_eq!(white.opposite(), Color::Black);
    }

    #[test]
    fn formats_it_to_the_correct_word() {
        assert_eq!(format!("{}", Color::Black), "black");
        assert_eq!(format!("{}", Color::White), "white");
    }
}
