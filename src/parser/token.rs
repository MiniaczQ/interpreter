use serde::{Deserialize, Serialize};

use super::{keywords::Keyword, operators::Operator, position::Position};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum TokenType {
    Operator(Operator),
    Keyword(Keyword),
    Identifier(String),
    String(String),
    Float(f64),
    Int(i64),
    EndOfTokens,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Token {
    pub token_type: TokenType,
    pub start: Position,
    pub stop: Position,
}

impl Token {
    pub fn empty() -> Self {
        Token {
            token_type: TokenType::EndOfTokens,
            start: Position::new(0, 0),
            stop: Position::new(0, 0),
        }
    }
}
