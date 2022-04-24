use std::fmt::Display;

use crate::{
    lexer::{keywords::Keyword, operators::Operator},
    position::Position,
    scannable::Scannable,
};

use super::char_scanner::CharScanner;

#[derive(Debug, Clone)]
pub enum LexemType {
    Operator(Operator),

    Keyword(Keyword),
    Comment(String),
    Identifier(String),
    String(String),
    Float(f64),
    Int(i64),

    Error(String),
}

impl Display for LexemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexemType::Operator(op) => f.write_fmt(format_args!("Operator ({:?})", op)),
            LexemType::Keyword(kw) => f.write_fmt(format_args!("Keyword ({:?})", kw)),
            LexemType::Identifier(id) => f.write_fmt(format_args!("Identifier ({})", id)),
            LexemType::Float(v) => f.write_fmt(format_args!("Float ({})", v)),
            LexemType::Int(v) => f.write_fmt(format_args!("Int ({})", v)),
            LexemType::Error(s) => f.write_fmt(format_args!("Error ({})", s)),
            LexemType::String(s) => f.write_fmt(format_args!("String ({:})", s)),
            LexemType::Comment(s) => f.write_fmt(format_args!("Comment ({:})", s)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Lexem {
    pub lexem_type: LexemType,
    pub start: Position,
    pub stop: Position,
}

pub struct LexemBuilder<'a> {
    scanner: &'a mut CharScanner,
    start: Position,
}

impl<'a> LexemBuilder<'a> {
    pub fn new(scanner: &'a mut CharScanner) -> Self {
        let start = (&*scanner).last_pos();
        Self { scanner, start }
    }

    

    pub fn bake(&self, token_type: LexemType) -> Lexem {
        Lexem {
            lexem_type: token_type,
            start: self.start,
            stop: self.scanner.last_pos(),
        }
    }
}

impl<'a> Scannable<char> for LexemBuilder<'a> {
    #[inline]
    fn pop(&mut self) {
        self.scanner.pop()
    }

    #[inline]
    fn peek(&self) -> char {
        self.scanner.peek()
    }
}

impl Display for Lexem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} from ({}) to ({})",
            self.lexem_type, self.start, self.stop
        ))
    }
}
