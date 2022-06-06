use crate::{
    interpreter::{
        context::{BlockCtx, Context},
        ExecutionError, ExecutionErrorVariant,
    },
    parser::grammar::Value,
};

use super::{
    super::utility::*,
    parse_expression,
    statement::{alternate_statements, parse_code_block, Statement},
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

impl Evaluable for WhileExpr {
    fn eval(&self, ctx: &dyn Context) -> Result<Value, ExecutionError> {
        let ctx = BlockCtx::new(ctx, "while loop".to_owned());
        let mut results = vec![];
        while match self.condition.eval(&ctx)? {
            Value::Bool(b) => b,
            _ => return Err(ExecutionError::new(ExecutionErrorVariant::InvalidType)),
        } {
            results.push(alternate_statements(&self.body, &ctx)?);
            if ctx.is_ret() {
                break;
            }
        }
        Ok(Value::List(results))
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

#[cfg(test)]
mod tests {
    use crate::{
        interpreter::{test_utils::tests::TestCtx, ExecutionErrorVariant},
        parser::grammar::expressions::{
            assignment::AssignmentExpr,
            binary::{BinaryExpr, BinaryOperator},
            identifier::IdentifierExpr,
            parse_expression,
            return_expr::ReturnExpr,
            while_expr::{parse_while_expression, WhileExpr},
        },
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

    #[test]
    fn eval_empty() {
        let ctx = TestCtx::new();
        assert_eq!(
            WhileExpr::new(Value::Bool(false).into(), vec![])
                .eval(&ctx)
                .unwrap(),
            Value::List(vec![])
        );
    }

    #[test]
    fn eval_range() {
        let ctx = TestCtx::new();
        ctx.variables
            .borrow_mut()
            .insert("a".to_owned(), Value::Int(3));
        assert_eq!(
            WhileExpr::new(
                BinaryExpr::new(
                    IdentifierExpr::new("a".to_owned()).into(),
                    BinaryOperator::Greater,
                    Value::Int(0).into()
                )
                .into(),
                vec![AssignmentExpr::new(
                    IdentifierExpr::new("a".to_owned()).into(),
                    BinaryExpr::new(
                        IdentifierExpr::new("a".to_owned()).into(),
                        BinaryOperator::Subtraction,
                        Value::Int(1).into()
                    )
                    .into()
                )
                .into()]
            )
            .eval(&ctx)
            .unwrap(),
            Value::List(vec![Value::Int(2), Value::Int(1), Value::Int(0)])
        );
    }

    #[test]
    fn eval_forward_return() {
        let ctx = TestCtx::new();
        assert_eq!(
            WhileExpr::new(
                Value::Bool(true).into(),
                vec![ReturnExpr::new(Value::Int(8).into()).into()]
            )
            .eval(&ctx)
            .unwrap(),
            Value::List(vec![Value::None])
        );
        assert_eq!(ctx.returning.take().unwrap(), Value::Int(8));
    }

    #[test]
    fn eval_invalid_condition() {
        let ctx = TestCtx::new();
        assert_eq!(
            WhileExpr::new(Value::Int(3).into(), vec![])
                .eval(&ctx)
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::InvalidType
        );
    }
}
