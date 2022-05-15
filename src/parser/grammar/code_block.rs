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
    use super::super::test_utils::tests::*;

    use grammar::{
        code_block::{CodeBlock, Statement},
        expressions::Expression,
        literals::Literal,
        Value,
    };

    use super::parse_code_block;

    #[test]
    fn miss() {
        let (result, warnings) = partial_parse(
            vec![dummy_token(TokenType::Operator(Op::CloseCurlyBracket))],
            parse_code_block,
        );
        assert_eq!(result, Ok(None));

        assert!(warnings.is_empty());
    }

    #[test]
    fn ok() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Operator(Op::Semicolon)),
                dummy_token(TokenType::Int(5)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_code_block,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            CodeBlock {
                statements: vec![
                    Statement::Expression(Expression::Identifier("a".to_owned())),
                    Statement::Semicolon,
                    Statement::Expression(Expression::Literal(Literal(Value::Integer(5)))),
                    Statement::Semicolon,
                ]
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn missing_bracket() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
                token(TokenType::Keyword(Kw::Fn), (2, 6), (2, 8)),
            ],
            parse_code_block,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            CodeBlock {
                statements: vec![Statement::Semicolon]
            }
        );

        assert_eq!(warnings.len(), 1);
        assert_eq!(
            warnings[0],
            ParserWarning {
                warning: ParserWarningVariant::MissingClosingCurlyBracket,
                start: Position::new(2, 6),
                stop: Position::new(2, 8),
            }
        );
    }

    #[test]
    fn out_of_tokens() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                token(TokenType::Operator(Op::Semicolon), (2, 5), (2, 6)),
            ],
            parse_code_block,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::OutOfTokens,
                pos: Position::new(2, 6),
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn no_statements() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_code_block,
        );
        assert_eq!(result.unwrap().unwrap(), CodeBlock { statements: vec![] });

        assert!(warnings.is_empty());
    }
}
