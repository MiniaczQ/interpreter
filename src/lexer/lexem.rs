use std::fmt::Display;

use crate::{
    lexer::{keywords::Keyword, operators::Operator},
    position::Position,
    scannable::Scannable,
};

use super::char_scanner::CharScanner;

#[derive(Debug, Clone, PartialEq)]
pub enum LexemType {
    Operator(Operator),
    Keyword(Keyword),
    Comment(String),
    Identifier(String),
    String(String),
    Float(f64),
    Int(i64),
}

impl Eq for LexemType {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Display for LexemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexemType::Operator(op) => f.write_fmt(format_args!("Operator ({:?})", op)),
            LexemType::Keyword(kw) => f.write_fmt(format_args!("Keyword ({:?})", kw)),
            LexemType::Identifier(id) => f.write_fmt(format_args!("Identifier ({})", id)),
            LexemType::Float(v) => f.write_fmt(format_args!("Float ({})", v)),
            LexemType::Int(v) => f.write_fmt(format_args!("Int ({})", v)),
            LexemType::String(s) => f.write_fmt(format_args!("String ({:})", s)),
            LexemType::Comment(s) => f.write_fmt(format_args!("Comment ({:})", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lexem {
    pub lexem_type: LexemType,
    pub start: Position,
    pub stop: Position,
}

impl Lexem {
    pub fn new(
        lexem_type: LexemType,
        start: impl Into<Position>,
        stop: impl Into<Position>,
    ) -> Self {
        Lexem {
            lexem_type,
            start: start.into(),
            stop: stop.into(),
        }
    }
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

    /// Lexem start position
    pub fn get_start(&self) -> Position {
        self.start
    }

    /// Scanner position
    pub fn get_here(&self) -> Position {
        self.scanner.last_pos()
    }

    /// Create a lexem
    pub fn bake_raw(&self, token_type: LexemType) -> Lexem {
        Lexem::new(token_type, self.start, self.scanner.last_pos())
    }

    /// Create a positive result of lexem matching
    pub fn bake(&self, token_type: LexemType) -> Option<Lexem> {
        Some(self.bake_raw(token_type))
    }
}

impl<'a> Scannable<char> for LexemBuilder<'a> {
    #[inline]
    fn pop(&mut self) -> bool {
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
