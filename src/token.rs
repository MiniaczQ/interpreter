use crate::position::Position;

#[derive(Debug, Clone)]
pub enum TokenType {
    //OpPlus,
    //OpMinus,

    //Comment,
    Identifier(String),
    //String(String),
    Float(f64),
    Int(i64),

    EndOfText,

    Error(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub start: Position,
    pub stop: Position,
}

impl Token {
    pub fn new(token_type: TokenType, start: Position, stop: Position) -> Self {
        Self {
            token_type,
            start,
            stop,
        }
    }
}
