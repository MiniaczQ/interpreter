use serde::{Deserialize, Serialize};

/// Possible keywords
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Keyword {
    Int,
    Float,
    Bool,
    String,
    Let,
    Fn,
    Return,
    While,
    For,
    In,
    If,
    Else,
    True,
    False,
}
