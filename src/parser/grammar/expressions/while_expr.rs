use crate::{
    interpreter::{context::Context, ExecutionError},
    parser::grammar::Value,
};

use super::{
    super::utility::*,
    parse_expression,
    statement::{parse_code_block, Statement},
    Evaluable, Expression,
};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WhileExpr {
    condition: Box<Expression>,
    body: Vec<Statement>,
}

impl WhileExpr {
    pub fn new(condition: Expression, body: Vec<Statement>) -> Self {
        Self {
            condition: Box::new(condition),
            body,
        }
    }
}

impl From<WhileExpr> for Expression {
    fn from(e: WhileExpr) -> Self {
        Expression::While(e)
    }
}

/// while_expression
///     = KW_WHILE, expression, code_block
///     ;
pub fn parse_while_expression(p: &mut Parser) -> OptRes<Expression> {
    if !p.keyword(Kw::While)? {
        return Ok(None);
    }
    let condition =
        parse_expression(p)?.ok_or_else(|| p.error(ErroVar::WhileLoopMissingCondition))?;
    let body = parse_code_block(p)?.ok_or_else(|| p.error(ErroVar::WhileLoopMissingBody))?;
    Ok(Some(WhileExpr::new(condition, body).into()))
}

impl Evaluable for WhileExpr {
    fn eval(&self, ctx: &dyn Context) -> Result<Value, ExecutionError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::grammar::expressions::{
        parse_expression,
        while_expr::{parse_while_expression, WhileExpr},
    };

    use super::super::super::test_utils::tests::*;

    #[test]
    fn miss_while_loop() {
        let (result, warnings) = partial_parse(
            vec![dummy_token(TokenType::Keyword(Kw::Let))],
            parse_while_expression,
        );
        assert_eq!(result, Ok(None));

        assert!(warnings.is_empty());
    }

    #[test]
    fn while_loop() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::While)),
                dummy_token(TokenType::Keyword(Kw::True)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            WhileExpr::new(Value::Bool(true).into(), vec![]).into()
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn while_loop_missing_body() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::While)),
                token(TokenType::Keyword(Kw::True), (6, 3), (6, 7)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::WhileLoopMissingBody,
                pos: Position::new(6, 7),
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn while_loop_missing_condition() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::While)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                token(TokenType::Operator(Op::CloseCurlyBracket), (9, 8), (9, 9)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::WhileLoopMissingBody,
                pos: Position::new(9, 9),
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn while_loop_out_of_tokens() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::While)),
                token(TokenType::Keyword(Kw::True), (2, 3), (2, 7)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::WhileLoopMissingBody,
                pos: Position::new(2, 7),
            }
        );

        assert!(warnings.is_empty());
    }
}
