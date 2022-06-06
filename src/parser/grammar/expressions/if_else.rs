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

/// If-else expression
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct IfElseExpr {
    condition: Box<Expression>,
    true_case: Vec<Statement>,
    false_case: Option<Vec<Statement>>,
}

impl IfElseExpr {
    pub fn new(
        condition: Expression,
        true_case: Vec<Statement>,
        false_case: Option<Vec<Statement>>,
    ) -> Self {
        Self {
            condition: Box::new(condition),
            true_case,
            false_case,
        }
    }
}

impl From<IfElseExpr> for Expression {
    fn from(e: IfElseExpr) -> Self {
        Expression::IfElse(e)
    }
}

impl Evaluable for IfElseExpr {
    fn eval(&self, ctx: &dyn Context) -> Result<Value, ExecutionError> {
        let cond = self.condition.eval(ctx)?;
        match cond {
            Value::Bool(true) => {
                let ctx = BlockCtx::new(ctx, "if branch".to_owned());
                alternate_statements(&self.true_case, &ctx)
            }
            Value::Bool(false) => {
                if let Some(statements) = &self.false_case {
                    let ctx = BlockCtx::new(ctx, "else branch".to_owned());
                    alternate_statements(statements, &ctx)
                } else {
                    Ok(Value::None)
                }
            }
            _ => Err(ExecutionError::new(ExecutionErrorVariant::InvalidType)),
        }
    }
}

/// if_expression
///     = KW_IF, expression, code_block, [KW_ELSE, code_block]
///     ;
pub fn parse_if_else_expression(p: &mut Parser) -> OptRes<Expression> {
    if !p.keyword(Kw::If)? {
        return Ok(None);
    }
    let condition = parse_expression(p)?.ok_or_else(|| p.error(ErroVar::IfMissingCondition))?;
    let true_case = parse_code_block(p)?.ok_or_else(|| p.error(ErroVar::IfMissingTrueBranch))?;
    let false_case = if p.keyword(Kw::Else)? {
        Some(parse_code_block(p)?.ok_or_else(|| p.error(ErroVar::IfMissingFalseBranch))?)
    } else {
        None
    };
    Ok(Some(
        IfElseExpr::new(condition, true_case, false_case).into(),
    ))
}

#[cfg(test)]
mod tests {
    use crate::{
        interpreter::{test_utils::tests::TestCtx, ExecutionErrorVariant},
        parser::grammar::expressions::{
            if_else::{parse_if_else_expression, IfElseExpr},
            parse_expression,
            return_expr::ReturnExpr,
            statement::Statement,
        },
    };

    use super::super::super::test_utils::tests::*;

    #[test]
    fn miss() {
        let (result, warnings) = partial_parse(
            vec![dummy_token(TokenType::Identifier("aaa".to_owned()))],
            parse_if_else_expression,
        );
        assert_eq!(result, Ok(None));

        assert!(warnings.is_empty());
    }

    #[test]
    fn just_if() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::If)),
                dummy_token(TokenType::Keyword(Kw::True)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
                dummy_token(TokenType::Keyword(Kw::If)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            IfElseExpr::new(Value::Bool(true).into(), vec![], None,).into()
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn if_else() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::If)),
                dummy_token(TokenType::Keyword(Kw::True)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
                dummy_token(TokenType::Keyword(Kw::Else)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            IfElseExpr::new(Value::Bool(true).into(), vec![], Some(vec![]),).into()
        );

        assert!(warnings.is_empty());
    }

    // considers code block as the condition, so results in a different error
    #[test]
    fn missing_condition() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::If)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                token(TokenType::Operator(Op::CloseCurlyBracket), (3, 5), (3, 6)),
                dummy_token(TokenType::Keyword(Kw::Else)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::IfMissingTrueBranch,
                pos: Position::new(3, 6)
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn missing_true_branch() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::If)),
                token(TokenType::Keyword(Kw::True), (7, 4), (7, 8)),
                dummy_token(TokenType::Keyword(Kw::Else)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::IfMissingTrueBranch,
                pos: Position::new(7, 8)
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn missing_false_branch() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::If)),
                dummy_token(TokenType::Keyword(Kw::True)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
                token(TokenType::Keyword(Kw::Else), (9, 6), (9, 10)),
                dummy_token(TokenType::Keyword(Kw::If)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::IfMissingFalseBranch,
                pos: Position::new(9, 10)
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn out_of_tokens() {
        let (result, warnings) = partial_parse(
            vec![token(TokenType::Keyword(Kw::If), (1, 2), (1, 4))],
            parse_expression,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::IfMissingCondition,
                pos: Position::new(1, 4)
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn eval_only_if_true() {
        let ctx = TestCtx::new();
        assert_eq!(
            IfElseExpr::new(Value::Bool(true).into(), vec![Value::Int(8).into()], None)
                .eval(&ctx)
                .unwrap(),
            Value::Int(8)
        );
    }

    #[test]
    fn eval_no_value() {
        let ctx = TestCtx::new();
        assert_eq!(
            IfElseExpr::new(
                Value::Bool(true).into(),
                vec![Value::Int(8).into(), Statement::Semicolon],
                None
            )
            .eval(&ctx)
            .unwrap(),
            Value::None
        );
    }

    #[test]
    fn eval_only_if_false() {
        let ctx = TestCtx::new();
        assert_eq!(
            IfElseExpr::new(Value::Bool(false).into(), vec![Value::Int(8).into()], None)
                .eval(&ctx)
                .unwrap(),
            Value::None
        );
    }

    #[test]
    fn eval_if_else_true() {
        let ctx = TestCtx::new();
        assert_eq!(
            IfElseExpr::new(
                Value::Bool(true).into(),
                vec![Value::Int(8).into()],
                Some(vec![Value::Int(-8).into()])
            )
            .eval(&ctx)
            .unwrap(),
            Value::Int(8)
        );
    }

    #[test]
    fn eval_if_else_false() {
        let ctx = TestCtx::new();
        assert_eq!(
            IfElseExpr::new(
                Value::Bool(false).into(),
                vec![Value::Int(8).into()],
                Some(vec![Value::Int(-8).into()])
            )
            .eval(&ctx)
            .unwrap(),
            Value::Int(-8)
        );
    }

    #[test]
    fn eval_invalid_condition() {
        let ctx = TestCtx::new();
        assert_eq!(
            IfElseExpr::new(
                Value::Int(8).into(),
                vec![Value::Int(8).into()],
                Some(vec![Value::Int(-8).into()])
            )
            .eval(&ctx)
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::InvalidType
        );
    }

    #[test]
    fn eval_only_if_false_forward_return() {
        let ctx = TestCtx::new();
        assert_eq!(
            IfElseExpr::new(
                Value::Bool(false).into(),
                vec![ReturnExpr::new(Value::Int(5).into()).into()],
                None
            )
            .eval(&ctx)
            .unwrap(),
            Value::None
        );
        assert_eq!(ctx.returning.take(), None);
    }

    #[test]
    fn eval_if_else_true_forward_return() {
        let ctx = TestCtx::new();
        assert_eq!(
            IfElseExpr::new(
                Value::Bool(true).into(),
                vec![ReturnExpr::new(Value::Int(5).into()).into()],
                Some(vec![ReturnExpr::new(Value::Int(-5).into()).into()])
            )
            .eval(&ctx)
            .unwrap(),
            Value::None
        );
        assert_eq!(ctx.returning.take().unwrap(), Value::Int(5));
    }

    #[test]
    fn eval_if_else_false_forward_return() {
        let ctx = TestCtx::new();
        assert_eq!(
            IfElseExpr::new(
                Value::Bool(false).into(),
                vec![ReturnExpr::new(Value::Int(5).into()).into()],
                Some(vec![ReturnExpr::new(Value::Int(-5).into()).into()])
            )
            .eval(&ctx)
            .unwrap(),
            Value::None
        );
        assert_eq!(ctx.returning.take().unwrap(), Value::Int(-5));
    }
}
