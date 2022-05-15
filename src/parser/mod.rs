use std::error::Error;
use std::fmt::Display;

use crate::scannable::Scannable;

use self::{
    grammar::program::{parse_program, Program},
    position::Position,
    token::Token,
};

pub mod grammar;
pub mod keywords;
pub mod operators;
pub mod position;
pub mod token;
pub mod token_scanner;

/// Errors that prevent parser from working
#[derive(Debug)]
pub enum ParserErrorVariant {
    OutOfTokens,
    FunctionParameterMissingType,
    FunctionMissingIdentifier,
    FunctionMissingReturnType, // mby warning and default to none type?
    FunctionMissingBody,
    IfMissingCondition,
    IfMissingTrueBranch,
    IfMissingFalseBranch,
    WhileLoopMissingCondition,
    WhileLoopMissingBody,
    ForLoopMissingVariable,
    ForLoopMissingProvider,
    ForLoopMissingBody,
    InvalidBracketExpression,
    ListRangeAccessIncomplete,
    ListAccessEmpty,
    UnaryOperatorMissingExpression,
    BinaryOperatorMissingRHS,
    AssignmentMissingExpression,
    VariableDeclarationMissingType,
    VariableDeclarationMissingIdentifier,
    VariableDeclarationMissingExpression,
    ReturnMissingExpression,
    ExpectedFunctionDefinition,
}

/// Critical errors remember the last position before they happened
#[derive(Debug)]
pub struct ParserError {
    error: ParserErrorVariant,
    pos: Position,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Parser error at {}: {:?}",
            self.pos, self.error
        ))
    }
}

impl Error for ParserError {}

/// Errors that the parser can work around
#[derive(Debug)]
pub enum ParserWarningVariant {
    TrailingComma,
    MissingOpeningRoundBracket,
    MissingClosingRoundBracket,
    MissingClosingSquareBracket,
    MissingClosingCurlyBracket,
    MissingColon,
    VariableDeclarationMissingEqualsSign,
    VariableDeclarationMissingTypeSeparator,
    ForLoopMissingInKeyword,
}

/// Elusive errors remember the position where they were supposed to be
#[derive(Debug)]
pub struct ParserWarning {
    warning: ParserWarningVariant,
    start: Position,
    stop: Position,
}

impl Display for ParserWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Parser warning from {} to {}: {:?}",
            self.start, self.stop, self.warning
        ))
    }
}

impl Error for ParserWarning {}

/// Language parser.
///
pub struct Parser<'a> {
    warnings: Vec<ParserWarning>,
    pos: Position,
    scanner: Box<dyn Scannable<Option<Token>> + 'a>,
}

impl<'a> Parser<'a> {
    pub fn new(token_scanner: impl Scannable<Option<Token>> + 'a) -> Self {
        Self {
            warnings: vec![],
            pos: Position { row: 1, col: 1 },
            scanner: Box::new(token_scanner),
        }
    }

    /// Attempts to parse.
    /// Returns either a `Program` or critical parsing error.
    pub fn parse(&mut self) -> Result<Program, ParserError> {
        parse_program(self)
    }

    /// Consumes parser and returns all parser warnings.
    pub fn get_warnings(self) -> Vec<ParserWarning> {
        self.warnings
    }
}

/// Trait for error and warning handling
pub trait ErrorHandler {
    /// Reports errors that can be omited.
    /// They can be recovered after parsing is over.
    fn warn(&mut self, err: ParserWarningVariant);

    /// Creates a critical error which aborts parsing.
    fn error<T>(&mut self, err: ParserErrorVariant) -> Result<T, ParserError>;
}

impl<'a> ErrorHandler for Parser<'a> {
    fn warn(&mut self, err: ParserWarningVariant) {
        let err = ParserWarning {
            warning: err,
            start: self.curr().unwrap().start,
            stop: self.curr().unwrap().stop,
        };
        self.warnings.push(err);
    }

    fn error<T>(&mut self, err: ParserErrorVariant) -> Result<T, ParserError> {
        Err(ParserError {
            error: err,
            pos: self.pos,
        })
    }
}

impl<'a> Scannable<Option<Token>> for Parser<'a> {
    fn curr(&self) -> Option<Token> {
        self.scanner.curr()
    }

    fn pop(&mut self) -> bool {
        self.pos = self.curr().unwrap().stop;
        self.scanner.pop()
    }
}

/// Scannable extension
pub trait ExtScannable: Scannable<Option<Token>> {
    /// Returns a token or parser error if no tokens are available
    fn token(&mut self) -> Result<Token, ParserError>;
}

impl<T: Scannable<Option<Token>> + ErrorHandler> ExtScannable for T {
    fn token(&mut self) -> Result<Token, ParserError> {
        if let Some(t) = self.curr() {
            Ok(t)
        } else {
            self.error(ParserErrorVariant::OutOfTokens)
        }
    }
}
