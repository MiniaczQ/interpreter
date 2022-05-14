use crate::scannable::Scannable;

use self::{position::Position, token::Token, token_scanner::TokenScanner};

pub mod grammar;
mod keywords;
mod operators;
mod position;
pub mod program;
mod token;
mod token_scanner;

pub enum CriticalParserErrorVariant {
    OutOfTokens,
}

pub struct CriticalParserError {
    err: CriticalParserErrorVariant,
    pos: Position,
}

pub enum ElusiveParserErrorVariant {
    TrailingComma,
    MissingClosingSquareBracket,
}

pub struct ElusiveParserError {
    err: ElusiveParserErrorVariant,
    start: Position,
    stop: Position,
}

pub struct Parser {
    errors: Vec<ElusiveParserError>,
    pos: Position,
    token_scanner: TokenScanner,
}

impl Parser {
    pub fn new(token_scanner: TokenScanner) -> Self {
        Self {
            errors: vec![],
            pos: Position { row: 1, col: 1 },
            token_scanner,
        }
    }

    pub fn error(&mut self, err: ElusiveParserErrorVariant) {
        self.errors.push(ElusiveParserError {
            err,
            start: self.curr().unwrap().start,
            stop: self.curr().unwrap().stop,
        });
    }
}

impl Scannable<Option<Token>> for Parser {
    fn curr(&self) -> Option<Token> {
        self.token_scanner.curr()
    }

    fn pop(&mut self) -> bool {
        self.pos = self.curr().unwrap().stop;
        self.token_scanner.pop()
    }
}

/// Scannable extension
pub trait ExtScannable: Scannable<Option<Token>> {
    /// Returns a token or parser error if no tokens are available
    fn token(&mut self) -> Result<Token, CriticalParserError>;
}

impl ExtScannable for Parser {
    fn token(&mut self) -> Result<Token, CriticalParserError> {
        self.curr().ok_or(CriticalParserError {
            err: CriticalParserErrorVariant::OutOfTokens,
            pos: self.pos,
        })
    }
}
