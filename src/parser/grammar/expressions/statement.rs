use super::{parse_expression, Expression};

use super::super::utility::*;

/// A single statement.
/// Either an expression or a `;`.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Statement {
    Expression(Expression),
    Semicolon,
}

impl<T: Into<Expression>> From<T> for Statement {
    fn from(e: T) -> Self {
        Self::Expression(e.into())
    }
}

/// statements
///     = {statement}
///     ;
fn parse_statements(p: &mut Parser) -> Res<Vec<Statement>> {
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
fn parse_statement(p: &mut Parser) -> OptRes<Statement> {
    if p.operator(Op::Semicolon)? {
        return Ok(Some(Statement::Semicolon));
    } else if let Some(expression) = parse_expression(p)? {
        return Ok(Some(expression.into()));
    }
    Ok(None)
}

/// code_block
///     = OPEN_CODEBLOCK, statements, CLOSE_CODEBLOCK
///     ;
pub fn parse_code_block(p: &mut Parser) -> OptRes<Vec<Statement>> {
    if !p.operator(Op::OpenCurlyBracket)? {
        return Ok(None);
    }
    let statements = parse_statements(p)?;
    if !p.operator(Op::CloseCurlyBracket)? {
        p.warn(WarnVar::MissingClosingCurlyBracket)?;
    }
    Ok(Some(statements))
}
