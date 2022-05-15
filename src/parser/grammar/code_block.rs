use super::{
    expressions::{parse_expression, Expression},
    utility::*,
};

/// Block of code that returns the last espression.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CodeBlock {
    pub statements: Vec<Statement>,
}

/// A single statement.
/// Either an expression or a `;`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Statement {
    Expression(Expression),
    Semicolon,
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
        return Ok(Some(Statement::Expression(expression)));
    }
    Ok(None)
}

/// code_block
///     = OPEN_CODEBLOCK, statements, CLOSE_CODEBLOCK
///     ;
pub fn parse_code_block(p: &mut Parser) -> OptRes<CodeBlock> {
    if !p.operator(Op::OpenCurlyBracket)? {
        return Ok(None);
    }
    let statements = parse_statements(p)?;
    if !p.operator(Op::CloseCurlyBracket)? {
        p.warn(WarnVar::MissingClosingCurlyBracket);
    }
    Ok(Some(CodeBlock { statements }))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
