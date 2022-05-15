use serde::{Deserialize, Serialize};

use super::{keywords::Keyword, operators::Operator, position::Position};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TokenType {
    Operator(Operator),
    Keyword(Keyword),
    Identifier(String),
    String(String),
    Float(f64),
    Int(i64),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Token {
    pub token_type: TokenType,
    pub start: Position,
    pub stop: Position,
}
