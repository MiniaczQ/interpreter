#[derive(Debug, Clone)]
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
}
