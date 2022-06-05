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
mod test_utils;
pub mod token;
pub mod token_scanner;

/// Errors that prevent parser from working
#[derive(Debug, PartialEq, Eq)]
pub enum ParserErrorVariant {
    FunctionParameterMissingType,
    FunctionMissingIdentifier,
    FunctionMissingReturnType, // technically, could default to `none`
    FunctionMissingBody,
    IfMissingCondition, // unreachable (following code block is assumed to be the condition)
    IfMissingTrueBranch,
    IfMissingFalseBranch,
    WhileLoopMissingCondition, // unreachable (following code block is assumed to be the provider)
    WhileLoopMissingBody,
    ForLoopMissingVariable,
    ForLoopMissingProvider, // unreachable (following code block is assumed to be the provider)
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
    TooManyWarnings,
}

/// Critical errors remember the last position before they happened
#[derive(Debug, PartialEq, Eq)]
pub struct ParserError {
    pub error: ParserErrorVariant,
    pub pos: Position,
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
#[derive(Debug, PartialEq, Eq)]
pub enum ParserWarningVariant {
    ExpectedExpression,
    MissingOpeningRoundBracket,
    MissingClosingRoundBracket,
    MissingClosingSquareBracket,
    MissingClosingCurlyBracket,
    MissingColon,
    VariableDeclarationMissingEqualsSign,
    VariableDeclarationMissingTypeSeparator,
    ForLoopMissingInKeyword,
    ExpectedParameter,
}

/// Elusive errors remember the position where they were supposed to be
#[derive(Debug, PartialEq, Eq)]
pub struct ParserWarning {
    pub warning: ParserWarningVariant,
    pub start: Position,
    pub stop: Position,
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
    scanner: Box<dyn Scannable<Token> + 'a>,
    max_warnings: i32,
}

impl<'a> Parser<'a> {
    #[allow(dead_code)]
    pub fn new_with_defaults(token_scanner: impl Scannable<Token> + 'a) -> Self {
        Self {
            warnings: vec![],
            pos: Position { row: 1, col: 1 },
            scanner: Box::new(token_scanner),
            max_warnings: -1,
        }
    }

    #[allow(dead_code)]
    pub fn new(token_scanner: impl Scannable<Token> + 'a, max_warnings: i32) -> Self {
        Self {
            warnings: vec![],
            pos: Position { row: 1, col: 1 },
            scanner: Box::new(token_scanner),
            max_warnings,
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
    fn warn(&mut self, err: ParserWarningVariant) -> Result<(), ParserError>;

    /// Creates a critical error which aborts parsing.
    #[must_use]
    fn error(&mut self, err: ParserErrorVariant) -> ParserError;
}

impl<'a> ErrorHandler for Parser<'a> {
    fn warn(&mut self, err: ParserWarningVariant) -> Result<(), ParserError> {
        let err = ParserWarning {
            warning: err,
            start: self.curr().start,
            stop: self.curr().stop,
        };
        self.warnings.push(err);
        if self.max_warnings > 0 && self.warnings.len() > self.max_warnings as usize {
            return Err(ParserError {
                error: ParserErrorVariant::TooManyWarnings,
                pos: self.pos,
            });
        }
        Ok(())
    }

    fn error(&mut self, err: ParserErrorVariant) -> ParserError {
        ParserError {
            error: err,
            pos: self.pos,
        }
    }
}

impl<'a> Scannable<Token> for Parser<'a> {
    fn curr(&self) -> Token {
        self.scanner.curr()
    }

    fn pop(&mut self) -> bool {
        self.pos = self.curr().stop;
        self.scanner.pop()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::parser::{
        grammar::{
            expressions::{declaration::DeclarationExpr, statement::Statement},
            function::FunctionDefinition,
            program::Program,
            DataType, Value,
        },
        test_utils::tests::{dummy_token, parse, token},
        ParserErrorVariant, ParserWarningVariant,
    };

    use super::{
        keywords::Keyword, operators::Operator, position::Position, token::TokenType, ParserError,
    };

    #[test]
    fn sole_parser() {
        let (result, warnings) = parse(vec![
            dummy_token(TokenType::Keyword(Keyword::Fn)),
            dummy_token(TokenType::Identifier("main".to_owned())),
            dummy_token(TokenType::Operator(Operator::OpenRoundBracket)),
            dummy_token(TokenType::Operator(Operator::CloseRoundBracket)),
            dummy_token(TokenType::Operator(Operator::OpenCurlyBracket)),
            dummy_token(TokenType::Keyword(Keyword::Let)),
            dummy_token(TokenType::Identifier("a".to_owned())),
            dummy_token(TokenType::Operator(Operator::Colon)),
            dummy_token(TokenType::Keyword(Keyword::Int)),
            dummy_token(TokenType::Operator(Operator::Equal)),
            dummy_token(TokenType::Int(5)),
            dummy_token(TokenType::Operator(Operator::Semicolon)),
            dummy_token(TokenType::Operator(Operator::CloseCurlyBracket)),
        ]);

        assert!(warnings.is_empty());

        let mut functions = HashMap::new();
        functions.insert(
            "main".to_owned(),
            FunctionDefinition {
                identifier: "main".to_owned(),
                params: vec![],
                statements: vec![
                    DeclarationExpr::new("a".to_owned(), DataType::Integer, Value::Int(5).into())
                        .into(),
                    Statement::Semicolon,
                ],
                data_type: DataType::None,
            },
        );

        let program = Program::new(functions);

        assert_eq!(program, result.unwrap());
    }

    #[test]
    fn sole_parser_warn() {
        let (result, warnings) = parse(vec![
            dummy_token(TokenType::Keyword(Keyword::Fn)),
            dummy_token(TokenType::Identifier("main".to_owned())),
            dummy_token(TokenType::Operator(Operator::OpenRoundBracket)),
            dummy_token(TokenType::Operator(Operator::CloseRoundBracket)),
            dummy_token(TokenType::Operator(Operator::OpenCurlyBracket)),
            dummy_token(TokenType::Keyword(Keyword::Let)),
            dummy_token(TokenType::Identifier("a".to_owned())),
            dummy_token(TokenType::Keyword(Keyword::Int)),
            dummy_token(TokenType::Operator(Operator::Equal)),
            dummy_token(TokenType::Int(5)),
            dummy_token(TokenType::Operator(Operator::Semicolon)),
            dummy_token(TokenType::Operator(Operator::CloseCurlyBracket)),
        ]);

        assert_eq!(warnings.len(), 1);
        assert_eq!(
            warnings[0].warning,
            ParserWarningVariant::VariableDeclarationMissingTypeSeparator
        );

        let mut functions = HashMap::new();
        functions.insert(
            "main".to_owned(),
            FunctionDefinition {
                identifier: "main".to_owned(),
                params: vec![],
                statements: vec![
                    DeclarationExpr::new("a".to_owned(), DataType::Integer, Value::Int(5).into())
                        .into(),
                    Statement::Semicolon,
                ],
                data_type: DataType::None,
            },
        );

        let program = Program::new(functions);

        assert_eq!(program, result.unwrap());
    }

    #[test]
    fn sole_parser_error() {
        let (result, warnings) = parse(vec![
            dummy_token(TokenType::Keyword(Keyword::Fn)),
            dummy_token(TokenType::Identifier("main".to_owned())),
            dummy_token(TokenType::Operator(Operator::OpenRoundBracket)),
            dummy_token(TokenType::Operator(Operator::CloseRoundBracket)),
            dummy_token(TokenType::Operator(Operator::OpenCurlyBracket)),
            dummy_token(TokenType::Keyword(Keyword::Let)),
            dummy_token(TokenType::Identifier("a".to_owned())),
            token(TokenType::Operator(Operator::Colon), (2, 5), (2, 6)),
            dummy_token(TokenType::Operator(Operator::Equal)),
            dummy_token(TokenType::Int(5)),
            dummy_token(TokenType::Operator(Operator::Semicolon)),
            dummy_token(TokenType::Operator(Operator::CloseCurlyBracket)),
        ]);

        assert!(warnings.is_empty());

        assert_eq!(
            ParserError {
                error: ParserErrorVariant::VariableDeclarationMissingType,
                pos: Position::new(2, 6),
            },
            result.unwrap_err()
        );
    }
}
