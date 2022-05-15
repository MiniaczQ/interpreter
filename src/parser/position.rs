use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Position of a token
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    #[allow(dead_code)]
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Ln {}, Col {}", self.row, self.col))
    }
}
