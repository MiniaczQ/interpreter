use std::{error::Error, fmt::Display};

use crate::{
    lexer::position::Position,
    lexer::{keywords::Keyword, operators::Operator},
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
    errors: &'a mut Vec<LexerWarning>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LexerWarningVariant {
    CommentNeverEnds,
    CommentTooLong,
    StringNeverEnds,
    StringTooLong,
    IntegerPartTooBig,
    DecimalPartTooBig,
    IdentifierTooLong,
    InvalidEscapeCharacter(char),
    InvalidSequence(String),
}

impl Display for LexerWarningVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerWarningVariant::CommentNeverEnds => f.write_str("comment never ends"),
            LexerWarningVariant::CommentTooLong => f.write_str("comment too long"),
            LexerWarningVariant::StringNeverEnds => f.write_str("string never ends"),
            LexerWarningVariant::StringTooLong => f.write_str("string too long"),
            LexerWarningVariant::IntegerPartTooBig => f.write_str("integer part too big"),
            LexerWarningVariant::DecimalPartTooBig => f.write_str("decimal part too big"),
            LexerWarningVariant::IdentifierTooLong => f.write_str("identifier too long"),
            LexerWarningVariant::InvalidEscapeCharacter(c) => {
                f.write_fmt(format_args!("invalid escape character `\\{}`", c))
            }
            LexerWarningVariant::InvalidSequence(s) => {
                f.write_fmt(format_args!("invalid character sequence `{}`", s))
            }
        }
    }
}

#[derive(Debug)]
pub struct LexerWarning {
    pub start: Position,
    pub end: Position,
    pub variant: LexerWarningVariant,
}

impl Display for LexerWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Lexer warning from {} to {}: {}",
            self.start, self.end, self.variant
        ))
    }
}

impl Error for LexerWarning {}

impl<'a> LexemBuilder<'a> {
    pub fn new(scanner: &'a mut CharScanner, errors: &'a mut Vec<LexerWarning>) -> Self {
        let start = (&*scanner).last_pos();
        Self {
            scanner,
            start,
            errors,
        }
    }

    /// Create a lexem
    pub fn bake_raw(&self, token_type: LexemType) -> Lexem {
        Lexem::new(token_type, self.start, self.scanner.last_pos())
    }

    /// Create a positive result of lexem matching
    pub fn bake(&self, token_type: LexemType) -> Option<Lexem> {
        Some(self.bake_raw(token_type))
    }

    /// Reports an error that happen during building
    pub fn error(&mut self, e: LexerWarningVariant) {
        self.errors.push(LexerWarning {
            start: self.start,
            end: self.scanner.last_pos(),
            variant: e,
        });
    }
}

impl<'a> Scannable<char> for LexemBuilder<'a> {
    #[inline]
    fn pop(&mut self) -> bool {
        self.scanner.pop()
    }

    #[inline]
    fn curr(&self) -> char {
        self.scanner.curr()
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
