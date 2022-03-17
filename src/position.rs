use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    row: usize,
    col: usize,
}

impl Position {
    pub fn next_char(&mut self) {
        self.col += 1;
    }

    pub fn next_line(&mut self) {
        self.row += 1;
        self.col = 0;
    }
}

impl Default for Position {
    fn default() -> Self {
        Self { row: 1, col: 0 }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}:{}", self.row, self.col))
    }
}
