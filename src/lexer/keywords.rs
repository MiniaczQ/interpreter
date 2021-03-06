/// Possible keywords
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
