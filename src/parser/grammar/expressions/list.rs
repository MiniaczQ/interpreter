use crate::{
    interpreter::{context::Context, ExecutionError},
    parser::grammar::Value,
};

use super::{super::utility::*, parse_expression, Evaluable, Expression};

/// List expression
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ListExpr(Vec<Expression>);

impl ListExpr {
    pub fn new(list: Vec<Expression>) -> Self {
        Self(list)
    }
}

impl From<ListExpr> for Expression {
    fn from(e: ListExpr) -> Self {
        Expression::List(e)
    }
}

impl Evaluable for ListExpr {
    fn eval(&self, ctx: &dyn Context) -> Result<Value, ExecutionError> {
        let values: Vec<Value> = self
            .0
            .iter()
            .map(|e| e.eval(ctx))
            .collect::<Result<_, ExecutionError>>()?;
        Ok(Value::List(values))
    }
}

/// list_expression
///     = OPEN_LIST, [expression, {SPLIT, expression}], CLOSE_LIST
///     ;
pub fn parse_list_expression(p: &mut Parser) -> OptRes<Expression> {
    let mut list: Vec<Expression> = vec![];
    if !p.operator(Op::OpenSquareBracket)? {
        return Ok(None);
    }
    if let Some(expression) = parse_expression(p)? {
        list.push(expression);
        while p.operator(Op::Split)? {
            if let Some(expression) = parse_expression(p)? {
                list.push(expression);
            } else {
                p.warn(WarnVar::ExpectedExpression)?;
            }
        }
    }
    if !p.operator(Op::CloseSquareBracket)? {
        p.warn(WarnVar::MissingClosingSquareBracket)?;
    }
    Ok(Some(ListExpr::new(list).into()))
}

#[cfg(test)]
mod tests {
    use crate::{
        interpreter::test_utils::tests::TestCtx,
        parser::grammar::expressions::list::{parse_list_expression, ListExpr},
    };

    use super::super::super::test_utils::tests::*;

    #[test]
    fn parse_ok() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Operator(Op::OpenSquareBracket)),
                dummy_token(TokenType::Int(5)),
                dummy_token(TokenType::Operator(Op::Split)),
                dummy_token(TokenType::Int(6)),
                dummy_token(TokenType::Operator(Op::CloseSquareBracket)),
            ],
            parse_list_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            ListExpr::new(vec![Value::Int(5).into(), Value::Int(6).into()]).into()
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn parse_empty() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Operator(Op::OpenSquareBracket)),
                dummy_token(TokenType::Operator(Op::CloseSquareBracket)),
                token(TokenType::Int(5), (2, 3), (2, 4)),
            ],
            parse_list_expression,
        );
        assert_eq!(result.unwrap().unwrap(), ListExpr::new(vec![]).into());

        assert!(warnings.is_empty());
    }

    #[test]
    fn parse_trailing_comma() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Operator(Op::OpenSquareBracket)),
                dummy_token(TokenType::Int(5)),
                dummy_token(TokenType::Operator(Op::Split)),
                dummy_token(TokenType::Int(6)),
                dummy_token(TokenType::Operator(Op::Split)),
                token(TokenType::Operator(Op::CloseSquareBracket), (5, 6), (5, 7)),
            ],
            parse_list_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            ListExpr::new(vec![Value::Int(5).into(), Value::Int(6).into()]).into()
        );

        assert_eq!(warnings.len(), 1);
        assert_eq!(
            warnings[0],
            ParserWarning {
                warning: ParserWarningVariant::ExpectedExpression,
                start: Position::new(5, 6),
                stop: Position::new(5, 7)
            }
        );
    }

    #[test]
    fn parse_missing_bracket() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Operator(Op::OpenSquareBracket)),
                dummy_token(TokenType::Int(5)),
                dummy_token(TokenType::Operator(Op::Split)),
                dummy_token(TokenType::Int(6)),
                token(TokenType::Keyword(Kw::Let), (7, 3), (7, 6)),
            ],
            parse_list_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            ListExpr::new(vec![Value::Int(5).into(), Value::Int(6).into()]).into()
        );

        assert_eq!(warnings.len(), 1);
        assert_eq!(
            warnings[0],
            ParserWarning {
                warning: ParserWarningVariant::MissingClosingSquareBracket,
                start: Position::new(7, 3),
                stop: Position::new(7, 6)
            }
        );
    }

    #[test]
    fn list_ok() {
        let ctx = TestCtx::new();
        assert_eq!(
            ListExpr::new(vec![]).eval(&ctx).unwrap(),
            Value::List(vec![])
        );
        assert_eq!(
            ListExpr::new(vec![Value::Int(8).into(), Value::Float(8.0).into()])
                .eval(&ctx)
                .unwrap(),
            Value::List(vec![Value::Int(8), Value::Float(8.0)])
        );
    }
}
