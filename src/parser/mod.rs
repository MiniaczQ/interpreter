use crate::scannable::Scannable;

use self::{position::Position, token::Token, token_scanner::TokenScanner};

pub mod grammar;
mod keywords;
mod operators;
mod position;
mod token;
mod token_scanner;

/// Errors that prevent parser from working
pub enum ParserErrorVariant {
    OutOfTokens,
    MissingType,
    MissingFunctionIdentifier,
    MissingFunctionReturnType, // mby warning and default to none type?
    MissingFunctionBody,
    MissingIfCondition,
    MissingIfTrueBranch,
    MissingIfFalseBranch,
    MissingWhileLoopCondition,
    MissingWhileLoopBody,
    MissingForLoopVariable,
    MissingForLoopProvider,
    MissingForLoopBody,
    InvalidBracketExpression,
    IncompleteRange,
    EmptyListAccess,
}

/// Critical errors remember the last position before they happened
pub struct ParserError {
    err: ParserErrorVariant,
    pos: Position,
}

/// Errors that the parser can work around
pub enum ParserWarningVariant {
    TrailingComma,
    MissingOpeningRoundBracket,
    MissingClosingRoundBracket,
    MissingClosingSquareBracket,
    MissingClosingCurlyBracket,
    MissingColon,
}

/// Elusive errors remember the position where they were supposed to be
pub struct ParserWarning {
    err: ParserWarningVariant,
    start: Position,
    stop: Position,
}

/// Language parser.
///
pub struct Parser {
    errors: Vec<ParserWarning>,
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
}

pub trait ErrorHandler {
    /// Reports errors that can be omited.
    /// They can be recovered after parsing is over.
    fn warn(&mut self, err: ParserWarningVariant);

    /// Creates a critical error which aborts parsing.
    fn error(&mut self, err: ParserErrorVariant) -> ParserError;
}

impl ErrorHandler for Parser {
    fn warn(&mut self, err: ParserWarningVariant) {
        self.errors.push(ParserWarning {
            err,
            start: self.curr().unwrap().start,
            stop: self.curr().unwrap().stop,
        });
    }

    fn error(&mut self, err: ParserErrorVariant) -> ParserError {
        ParserError { err, pos: self.pos }
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
    fn token(&mut self) -> Result<Token, ParserError>;
}

impl<T: Scannable<Option<Token>> + ErrorHandler> ExtScannable for T {
    fn token(&mut self) -> Result<Token, ParserError> {
        self.curr()
            .ok_or(self.error(ParserErrorVariant::OutOfTokens))
    }
}
