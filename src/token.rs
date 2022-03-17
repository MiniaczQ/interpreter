use crate::position::Position;

#[derive(Debug, Clone)]
pub enum TokenType {
    OpPlus,
    OpMinus,

    Comment,

    Identifier(String),
    String(String),

    Float(f64),
    Int(i64),

    EndOfText,

    Error(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub byte: usize,
    pub position: Position,
}
