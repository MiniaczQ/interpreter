use super::{keywords::Keyword, operators::Operator, position::Position};

#[derive(Clone)]
pub enum TokenType {
    Operator(Operator),
    Keyword(Keyword),
    Identifier(String),
    String(String),
    Float(f64),
    Int(i64),
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub start: Position,
    pub stop: Position,
}
