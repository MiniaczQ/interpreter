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

/// For loop expression
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ForExpr {
    variable: String,
    provider: Box<Expression>,
    body: Vec<Statement>,
}

impl ForExpr {
    pub fn new(variable: String, provider: Expression, body: Vec<Statement>) -> Self {
        Self {
            variable,
            provider: Box::new(provider),
            body,
        }
    }
}

impl From<ForExpr> for Expression {
    fn from(e: ForExpr) -> Self {
        Expression::For(e)
    }
}

impl Evaluable for ForExpr {
    fn eval(&self, ctx: &dyn Context) -> Result<Value, ExecutionError> {
        let provider = self.provider.eval(ctx)?;
        if let Value::List(list) = provider {
            let ctx = BlockCtx::new(ctx, "for loop".to_owned());
            let mut results = vec![];
            for v in list {
                ctx.variables.borrow_mut().insert(self.variable.clone(), v);
                results.push(alternate_statements(&self.body, &ctx)?);
                if ctx.is_ret() {
                    break;
                }
            }
            Ok(Value::List(results))
        } else {
            Err(ExecutionError::new(ExecutionErrorVariant::InvalidType))
        }
    }
}

/// for_expression
///     = KW_FOR, IDENTIFIER, KW_IN, expression, code_block
///     ;
pub fn parse_for_expression(p: &mut Parser) -> OptRes<Expression> {
    if !p.keyword(Kw::For)? {
        return Ok(None);
    }
    let variable = p
        .identifier()?
        .ok_or_else(|| p.error(ErroVar::ForLoopMissingVariable))?;
    if !p.keyword(Kw::In)? {
        p.warn(WarnVar::ForLoopMissingInKeyword)?;
    }
    let provider = parse_expression(p)?.ok_or_else(|| p.error(ErroVar::ForLoopMissingProvider))?;
    let body = parse_code_block(p)?.ok_or_else(|| p.error(ErroVar::ForLoopMissingBody))?;
    Ok(Some(ForExpr::new(variable, provider, body).into()))
}

#[cfg(test)]
mod tests {
    use crate::{
        interpreter::{test_utils::tests::TestCtx, ExecutionErrorVariant},
        parser::grammar::expressions::{
            for_expr::{parse_for_expression, ForExpr},
            identifier::IdentifierExpr,
            parse_expression,
            return_expr::ReturnExpr,
            statement::Statement,
        },
    };

    use super::super::super::test_utils::tests::*;

    #[test]
    fn parse_miss() {
        let (result, warnings) = partial_parse(
            vec![dummy_token(TokenType::Keyword(Kw::Let))],
            parse_for_expression,
        );
        assert_eq!(result, Ok(None));

        assert!(warnings.is_empty());
    }

    #[test]
    fn parse() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::For)),
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Keyword(Kw::In)),
                dummy_token(TokenType::Identifier("b".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            ForExpr::new(
                "a".to_owned(),
                IdentifierExpr::new("b".to_owned()).into(),
                vec![]
            )
            .into()
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn parse_missing_in_keyword() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::For)),
                dummy_token(TokenType::Identifier("a".to_owned())),
                token(TokenType::Identifier("b".to_owned()), (7, 6), (7, 7)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            ForExpr::new(
                "a".to_owned(),
                IdentifierExpr::new("b".to_owned()).into(),
                vec![]
            )
            .into()
        );

        assert_eq!(warnings.len(), 1);
        assert_eq!(
            warnings[0],
            ParserWarning {
                warning: ParserWarningVariant::ForLoopMissingInKeyword,
                start: Position::new(7, 6),
                stop: Position::new(7, 7)
            }
        );
    }

    #[test]
    fn parse_missing_body() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::For)),
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Keyword(Kw::In)),
                token(TokenType::Identifier("b".to_owned()), (7, 2), (7, 3)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::ForLoopMissingBody,
                pos: Position::new(7, 3),
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn parse_missing_provider() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::For)),
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Keyword(Kw::In)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                token(TokenType::Operator(Op::CloseCurlyBracket), (9, 2), (9, 3)),
                dummy_token(TokenType::Keyword(Kw::Let)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::ForLoopMissingBody,
                pos: Position::new(9, 3),
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn parse_missing_variable() {
        let (result, warnings) = partial_parse(
            vec![
                token(TokenType::Keyword(Kw::For), (5, 6), (5, 9)),
                dummy_token(TokenType::Keyword(Kw::In)),
                dummy_token(TokenType::Identifier("b".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::ForLoopMissingVariable,
                pos: Position::new(5, 9),
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn parse_out_of_tokens() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::For)),
                dummy_token(TokenType::Identifier("a".to_owned())),
                token(TokenType::Keyword(Kw::In), (2, 3), (2, 5)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::ForLoopMissingProvider,
                pos: Position::new(2, 5),
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn eval_empty() {
        let ctx = TestCtx::new();
        assert_eq!(
            ForExpr::new("a".to_owned(), Value::List(vec![]).into(), vec![])
                .eval(&ctx)
                .unwrap(),
            Value::List(vec![])
        );
    }

    #[test]
    fn eval_single() {
        let ctx = TestCtx::new();
        assert_eq!(
            ForExpr::new(
                "a".to_owned(),
                Value::List(vec![Value::Int(8)]).into(),
                vec![IdentifierExpr::new("a".to_owned()).into()]
            )
            .eval(&ctx)
            .unwrap(),
            Value::List(vec![Value::Int(8)])
        );
    }

    #[test]
    fn eval_many_mixed() {
        let ctx = TestCtx::new();
        assert_eq!(
            ForExpr::new(
                "a".to_owned(),
                Value::List(vec![Value::Int(8), Value::Float(9.0), Value::List(vec![])]).into(),
                vec![IdentifierExpr::new("a".to_owned()).into()]
            )
            .eval(&ctx)
            .unwrap(),
            Value::List(vec![Value::Int(8), Value::Float(9.0), Value::List(vec![])])
        );
    }

    #[test]
    fn eval_no_ending() {
        let ctx = TestCtx::new();
        assert_eq!(
            ForExpr::new(
                "a".to_owned(),
                Value::List(vec![Value::Int(8), Value::Int(8)]).into(),
                vec![
                    IdentifierExpr::new("a".to_owned()).into(),
                    Statement::Semicolon
                ]
            )
            .eval(&ctx)
            .unwrap(),
            Value::List(vec![Value::None, Value::None])
        );
    }

    #[test]
    fn eval_forward_return() {
        let ctx = TestCtx::new();
        assert_eq!(
            ForExpr::new(
                "a".to_owned(),
                Value::List(vec![Value::Int(8), Value::Int(9)]).into(),
                vec![
                    ReturnExpr::new(IdentifierExpr::new("a".to_owned()).into()).into(),
                    Statement::Semicolon,
                    IdentifierExpr::new("a".to_owned()).into()
                ]
            )
            .eval(&ctx)
            .unwrap(),
            Value::List(vec![Value::None])
        );
        assert_eq!(ctx.returning.take().unwrap(), Value::Int(8));
    }

    #[test]
    fn eval_wrong_provider() {
        let ctx = TestCtx::new();
        assert_eq!(
            ForExpr::new(
                "a".to_owned(),
                Value::Int(8).into(),
                vec![
                    ReturnExpr::new(IdentifierExpr::new("a".to_owned()).into()).into(),
                    Statement::Semicolon,
                    IdentifierExpr::new("a".to_owned()).into()
                ]
            )
            .eval(&ctx)
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::InvalidType
        );
    }
}
