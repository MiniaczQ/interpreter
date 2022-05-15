use crate::scannable::Scannable;

use self::{
    grammar::program::{parse_program, Program},
    position::Position,
    token::Token,
    token_scanner::TokenScanner,
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
    err: ParserErrorVariant,
    pos: Position,
}

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

    pub fn parse(&mut self) -> Result<Program, ParserError> {
        parse_program(self)
    }
}

/// Trait for error and warning handling
pub trait ErrorHandler {
    /// Reports errors that can be omited.
    /// They can be recovered after parsing is over.
    fn warn(&mut self, err: ParserWarningVariant);

    /// Creates a critical error which aborts parsing.
    fn error(&mut self, err: ParserErrorVariant) -> ParserError;
}

impl ErrorHandler for Parser {
    fn warn(&mut self, err: ParserWarningVariant) {
        let err = ParserWarning {
            err,
            start: self.curr().unwrap().start,
            stop: self.curr().unwrap().stop,
        };
        println!("[WARNING] {:?}", err);
        self.errors.push(err);
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
            .ok_or_else(|| self.error(ParserErrorVariant::OutOfTokens))
    }
}
