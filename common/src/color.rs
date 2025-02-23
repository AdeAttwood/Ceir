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

impl std::str::FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "black" => Ok(Color::Black),
            "white" => Ok(Color::White),
            "b" => Ok(Color::Black),
            "w" => Ok(Color::White),
            _ => Err(format!("Unable to parse '{}' into a color", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

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

    #[test]
    fn from_str() {
        assert_eq!(Color::from_str("w"), Ok(Color::White));
        assert_eq!(Color::from_str("white"), Ok(Color::White));

        assert_eq!(Color::from_str("b"), Ok(Color::Black));
        assert_eq!(Color::from_str("black"), Ok(Color::Black));
    }
}
