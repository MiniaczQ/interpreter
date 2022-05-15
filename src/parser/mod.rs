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
    OutOfTokens,
    FunctionParameterMissingType,
    FunctionMissingIdentifier,
    FunctionMissingReturnType, // could default to `none`
    FunctionMissingBody,
    IfMissingCondition, // unreachable (following code block is assumed to be the condition)
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

#[cfg(test)]
mod tests {
    use crate::{
        parser::{
            grammar::{
                code_block::{CodeBlock, Statement},
                expressions::Expression,
                function::FunctionDef,
                literals::Literal,
                program::Program,
                DataType, Value,
            },
            ParserErrorVariant, ParserWarningVariant, test_utils::tests::{dummy_token, parse, token},
        },
    };

    use super::{
        keywords::Keyword,
        operators::Operator,
        position::Position,
        token::{TokenType},
        ParserError,
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

        let program = Program {
            functions: vec![FunctionDef {
                identifier: "main".to_owned(),
                params: vec![],
                code_block: CodeBlock {
                    statements: vec![
                        Statement::Expression(Expression::Declaration {
                            identifier: "a".to_owned(),
                            data_type: DataType::Integer,
                            expression: Box::new(Expression::Literal(Literal(Value::Integer(5)))),
                        }),
                        Statement::Semicolon,
                    ],
                },
                data_type: DataType::None,
            }],
        };

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

        let program = Program {
            functions: vec![FunctionDef {
                identifier: "main".to_owned(),
                params: vec![],
                code_block: CodeBlock {
                    statements: vec![
                        Statement::Expression(Expression::Declaration {
                            identifier: "a".to_owned(),
                            data_type: DataType::Integer,
                            expression: Box::new(Expression::Literal(Literal(Value::Integer(5)))),
                        }),
                        Statement::Semicolon,
                    ],
                },
                data_type: DataType::None,
            }],
        };

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
