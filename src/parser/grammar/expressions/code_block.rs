use crate::{
    interpreter::{context::Context, ExecutionError},
    parser::grammar::Value,
};

use super::{
    super::utility::*,
    statement::{parse_code_block, Statement},
    Evaluable, Expression,
};

/// Block of code expression
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CodeBlockExpr(Vec<Statement>);

impl CodeBlockExpr {
    #[allow(dead_code)]
    pub fn new(statements: Vec<Statement>) -> Self {
        Self(statements)
    }
}

impl From<CodeBlockExpr> for Expression {
    fn from(e: CodeBlockExpr) -> Self {
        Expression::CodeBlock(e)
    }
}

impl Evaluable for CodeBlockExpr {
    fn eval(&self, ctx: &dyn Context) -> Result<Value, ExecutionError> {
        todo!()
    }
}

/// code_block
pub fn parse_code_block_expression(p: &mut Parser) -> OptRes<Expression> {
    parse_code_block(p).map(|v| v.map(|v| CodeBlockExpr(v).into()))
}

#[cfg(test)]
mod tests {
    use crate::parser::grammar::expressions::{
        code_block::{parse_code_block_expression, CodeBlockExpr, Statement},
        identifier::IdentifierExpr,
        parse_expression,
    };

    use super::super::super::test_utils::tests::*;

    #[test]
    fn parse_miss() {
        let (result, warnings) = partial_parse(
            vec![dummy_token(TokenType::Operator(Op::CloseCurlyBracket))],
            parse_code_block_expression,
        );
        assert_eq!(result, Ok(None));

        assert!(warnings.is_empty());
    }

    #[test]
    fn parse_ok() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Operator(Op::Semicolon)),
                dummy_token(TokenType::Int(5)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            CodeBlockExpr::new(vec![
                IdentifierExpr::new("a".to_owned()).into(),
                Statement::Semicolon,
                Value::Int(5).into(),
                Statement::Semicolon,
            ])
            .into()
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn parse_missing_bracket() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
                token(TokenType::Keyword(Kw::Fn), (2, 6), (2, 8)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            CodeBlockExpr::new(vec![Statement::Semicolon]).into()
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
    fn parse_out_of_tokens() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                token(TokenType::Operator(Op::Semicolon), (2, 5), (2, 6)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            CodeBlockExpr::new(vec![Statement::Semicolon]).into()
        );

        assert_eq!(
            warnings[0],
            ParserWarning {
                warning: ParserWarningVariant::MissingClosingCurlyBracket,
                start: Position::new(2, 6),
                stop: Position::new(2, 6),
            }
        );
    }

    #[test]
    fn parse_no_statements() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_expression,
        );
        assert_eq!(result.unwrap().unwrap(), CodeBlockExpr::new(vec![]).into());

        assert!(warnings.is_empty());
    }

    #[test]
    fn parse_code_block() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Operator(Op::Semicolon)),
                dummy_token(TokenType::Int(5)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            CodeBlockExpr::new(vec![
                IdentifierExpr::new("a".to_owned()).into(),
                Statement::Semicolon,
                Value::Int(5).into(),
                Statement::Semicolon,
            ])
            .into()
        );

        assert!(warnings.is_empty());
    }
}