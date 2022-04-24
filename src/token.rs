use std::fmt::Display;

use crate::{position::Position, scanner::Scanner, lexer::{operators::Operator, keywords::Keyword}};

#[derive(Debug, Clone)]
pub enum TokenType {
    Operator(Operator),

    Keyword(Keyword),
    Comment(String),
    Identifier(String),
    String(String),
    Float(f64),
    Int(i64),

    Error(String),
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Operator(op) => f.write_fmt(format_args!("Operator ({:?})", op)),
            TokenType::Keyword(kw) => f.write_fmt(format_args!("Keyword ({:?})", kw)),
            TokenType::Identifier(id) => f.write_fmt(format_args!("Identifier ({})", id)),
            TokenType::Float(v) => f.write_fmt(format_args!("Float ({})", v)),
            TokenType::Int(v) => f.write_fmt(format_args!("Int ({})", v)),
            TokenType::Error(s) => f.write_fmt(format_args!("Error ({})", s)),
            TokenType::String(s) => f.write_fmt(format_args!("String ({:})", s)),
            TokenType::Comment(s) => f.write_fmt(format_args!("Comment ({:})", s)),
            //TokenType:: => {
            //    f.write_fmt(format_args!(""))
            //},
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub start: Position,
    pub stop: Position,
}

pub struct TokenBuilder<'a> {
    scanner: &'a mut Scanner,
    start: Position,
}

impl<'a> TokenBuilder<'a> {
    pub fn new(scanner: &'a mut Scanner) -> Self {
        let start = (&*scanner).last_pos();
        Self { scanner, start }
    }

    #[inline]
    pub fn pop(&mut self) {
        self.scanner.pop()
    }

    #[inline]
    pub fn peek(&self) -> char {
        self.scanner.peek()
    }

    pub fn bake(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            start: self.start,
            stop: self.scanner.last_pos(),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} from ({}) to ({})",
            self.token_type, self.start, self.stop
        ))
    }
}
