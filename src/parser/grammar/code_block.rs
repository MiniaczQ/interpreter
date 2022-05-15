use crate::{
    parser::{
        operators::Operator, token::TokenType, ErrorHandler, ExtScannable, Parser, ParserError,
        ParserWarningVariant,
    },
    scannable::Scannable,
};

use super::{
    expressions::{parse_expression, Expression},
    ParseResult,
};

/// Block of code that returns the last espression
#[derive(Debug, Clone)]
pub struct CodeBlock {
    statements: Vec<Statement>,
}

/// code_block
///     = OPEN_CODEBLOCK, statements, CLOSE_CODEBLOCK
///     ;
pub fn parse_code_block(p: &mut Parser) -> ParseResult<CodeBlock> {
    if let TokenType::Operator(Operator::OpenCurlyBracket) = p.token()?.token_type {
        p.pop();
        let statements = parse_statements(p)?;
        if let TokenType::Operator(Operator::CloseCurlyBracket) = p.token()?.token_type {
            p.pop();
        } else {
            p.warn(ParserWarningVariant::MissingClosingCurlyBracket);
        }
        Ok(Some(CodeBlock { statements }))
    } else {
        Ok(None)
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    Semicolon,
}

/// statements
///     = {statement}
///     ;
fn parse_statements(p: &mut Parser) -> Result<Vec<Statement>, ParserError> {
    let mut statements = vec![];
    while let Some(statement) = parse_statement(p)? {
        statements.push(statement);
    }
    Ok(statements)
}

/// statement
///     = expression
///     | EXPRESSION_END
///     ;
fn parse_statement(p: &mut Parser) -> ParseResult<Statement> {
    if let TokenType::Operator(Operator::Semicolon) = p.token()?.token_type {
        p.pop();
        Ok(Some(Statement::Semicolon))
    } else if let Some(expression) = parse_expression(p)? {
        Ok(Some(Statement::Expression(expression)))
    } else {
        Ok(None)
    }
}
