use std::fmt::Display;

/// Position of a token
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    row: usize,
    col: usize,
}

impl Position {
    /// Forwards the column
    pub fn next_col(&mut self) {
        self.col += 1;
    }

    /// Forwards the line and resets the column
    pub fn next_line(&mut self) {
        self.row += 1;
        self.col = 1;
    }
}

impl Default for Position {
    fn default() -> Self {
        Self { row: 1, col: 1 }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Ln {}, Col {}", self.row, self.col))
    }
}

impl From<(usize, usize)> for Position {
    fn from((row, col): (usize, usize)) -> Self {
        Self { row, col }
    }
}
