use crate::{
    interpreter::{context::Context, ExecutionError, ExecutionErrorVariant},
    parser::grammar::Value,
};

use super::{
    super::utility::*, parse_constant_or_identifier_or_bracket_expression, parse_expression,
    Evaluable, Expression,
};

/// Two ways of accessing list elements
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum IndexOrRange {
    Index(Box<Expression>),
    Range(Box<Expression>, Box<Expression>),
}

impl IndexOrRange {
    pub fn index(i: Expression) -> Self {
        IndexOrRange::Index(Box::new(i))
    }

    pub fn range(li: Expression, ri: Expression) -> Self {
        IndexOrRange::Range(Box::new(li), Box::new(ri))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ListAccessExpr {
    list: Box<Expression>,
    access: IndexOrRange,
}

impl ListAccessExpr {
    pub fn new(list: Expression, access: IndexOrRange) -> Self {
        Self {
            list: Box::new(list),
            access,
        }
    }
}

impl From<ListAccessExpr> for Expression {
    fn from(e: ListAccessExpr) -> Self {
        Expression::ListAccess(e)
    }
}

impl Evaluable for ListAccessExpr {
    fn eval(&self, ctx: &dyn Context) -> Result<Value, ExecutionError> {
        let value = self.list.eval(ctx)?;
        if let Value::List(list) = value {
            eval_list(ctx, &self.access, list)
        } else if let Value::String(list) = value {
            eval_string(ctx, &self.access, list)
        } else {
            Err(ExecutionError::new(
                ExecutionErrorVariant::UnsupportedListAccess,
            ))
        }
    }
}

/// index_or_range_access
///     = expression, [RANGE, expression]
///     ;
fn parse_index_or_range_access(p: &mut Parser) -> Res<IndexOrRange> {
    let left_index = parse_expression(p)?.ok_or_else(|| p.error(ErroVar::ListAccessEmpty))?;
    if !p.operator(Op::DoubleColon)? {
        return Ok(IndexOrRange::index(left_index));
    }
    let right_index =
        parse_expression(p)?.ok_or_else(|| p.error(ErroVar::ListRangeAccessIncomplete))?;
    Ok(IndexOrRange::range(left_index, right_index))
}

/// list_access
///     = OPEN_LIST, index_or_range_access, CLOSE_LIST
///     ;
fn parse_list_access(p: &mut Parser) -> OptRes<IndexOrRange> {
    if !p.operator(Op::OpenSquareBracket)? {
        return Ok(None);
    }
    let index_or_range = parse_index_or_range_access(p)?;
    if !p.operator(Op::CloseSquareBracket)? {
        p.warn(WarnVar::MissingClosingSquareBracket)?;
    }
    Ok(Some(index_or_range))
}

/// list_access_expression
///     = const_or_identifier_or_function_call_expression, [list_access]
///     ;
pub fn parse_list_access_expression(p: &mut Parser) -> OptRes<Expression> {
    if let Some(mut expression) = parse_constant_or_identifier_or_bracket_expression(p)? {
        if let Some(access) = parse_list_access(p)? {
            expression = ListAccessExpr::new(expression, access).into();
        }
        return Ok(Some(expression));
    }
    Ok(None)
}

fn eval_list(
    ctx: &dyn Context,
    access: &IndexOrRange,
    list: Vec<Value>,
) -> Result<Value, ExecutionError> {
    match access {
        IndexOrRange::Index(idx) => {
            let idx = idx.eval(ctx)?;
            eval_index(list, idx)
        }
        IndexOrRange::Range(lidx, ridx) => {
            let lidx = lidx.eval(ctx)?;
            let ridx = ridx.eval(ctx)?;
            eval_range(list, lidx, ridx).map(Value::List)
        }
    }
}

fn eval_string(
    ctx: &dyn Context,
    access: &IndexOrRange,
    list: String,
) -> Result<Value, ExecutionError> {
    let list: Vec<char> = list.chars().collect();
    match access {
        IndexOrRange::Index(idx) => {
            let idx = idx.eval(ctx)?;
            eval_index(list, idx).map(|c| Value::String(c.into()))
        }
        IndexOrRange::Range(lidx, ridx) => {
            let lidx = lidx.eval(ctx)?;
            let ridx = ridx.eval(ctx)?;
            eval_range(list, lidx, ridx).map(|v| Value::String(v.into_iter().collect()))
        }
    }
}

fn eval_range<T: Clone>(
    mut list: Vec<T>,
    lidx: Value,
    ridx: Value,
) -> Result<Vec<T>, ExecutionError> {
    let list_size = list.len() as i64;
    let lidx = if let Value::Int(lidx) = lidx {
        if 0 <= lidx && lidx < list_size {
            lidx as usize
        } else {
            return Err(ExecutionError::new(ExecutionErrorVariant::IndexOutOfBounds));
        }
    } else {
        return Err(ExecutionError::new(ExecutionErrorVariant::NonIntegerIndex));
    };
    let ridx = if let Value::Int(ridx) = ridx {
        if 0 < ridx && ridx <= list_size {
            ridx as usize
        } else {
            return Err(ExecutionError::new(ExecutionErrorVariant::IndexOutOfBounds));
        }
    } else {
        return Err(ExecutionError::new(ExecutionErrorVariant::NonIntegerIndex));
    };
    Ok(list.drain(lidx..ridx).collect())
}

fn eval_index<T: Clone>(list: Vec<T>, idx: Value) -> Result<T, ExecutionError> {
    let list_size = list.len() as i64;
    let idx = if let Value::Int(idx) = idx {
        if 0 <= idx && idx < list_size {
            idx as usize
        } else {
            return Err(ExecutionError::new(ExecutionErrorVariant::IndexOutOfBounds));
        }
    } else {
        return Err(ExecutionError::new(ExecutionErrorVariant::NonIntegerIndex));
    };
    Ok(list[idx].clone())
}

#[cfg(test)]
mod tests {
    use crate::{
        interpreter::{test_utils::tests::TestCtx, ExecutionErrorVariant},
        parser::grammar::expressions::{
            identifier::IdentifierExpr,
            list_access::{IndexOrRange, ListAccessExpr},
            parse_expression,
        },
    };

    use super::super::super::test_utils::tests::*;

    #[test]
    fn list_access_index() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenSquareBracket)),
                dummy_token(TokenType::Int(1)),
                dummy_token(TokenType::Operator(Op::CloseSquareBracket)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            ListAccessExpr::new(
                IdentifierExpr::new("a".to_owned()).into(),
                IndexOrRange::index(Value::Int(1).into())
            )
            .into()
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn list_access_empty() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Identifier("a".to_owned())),
                token(TokenType::Operator(Op::OpenSquareBracket), (3, 4), (3, 5)),
                dummy_token(TokenType::Operator(Op::CloseSquareBracket)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::ListAccessEmpty,
                pos: Position::new(3, 5),
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn list_access_range() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenSquareBracket)),
                dummy_token(TokenType::Int(1)),
                dummy_token(TokenType::Operator(Op::DoubleColon)),
                dummy_token(TokenType::Int(5)),
                dummy_token(TokenType::Operator(Op::CloseSquareBracket)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            ListAccessExpr::new(
                IdentifierExpr::new("a".to_owned()).into(),
                IndexOrRange::range(Value::Int(1).into(), Value::Int(5).into())
            )
            .into()
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn list_access_missing_closing_bracket() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenSquareBracket)),
                dummy_token(TokenType::Int(1)),
                dummy_token(TokenType::Operator(Op::DoubleColon)),
                dummy_token(TokenType::Int(5)),
                token(TokenType::Operator(Op::Semicolon), (7, 8), (7, 9)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            ListAccessExpr::new(
                IdentifierExpr::new("a".to_owned()).into(),
                IndexOrRange::range(Value::Int(1).into(), Value::Int(5).into())
            )
            .into()
        );

        assert_eq!(warnings.len(), 1);
        assert_eq!(
            warnings[0],
            ParserWarning {
                warning: ParserWarningVariant::MissingClosingSquareBracket,
                start: Position::new(7, 8),
                stop: Position::new(7, 9)
            }
        );
    }

    #[test]
    fn list_access_range_incomplete() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenSquareBracket)),
                dummy_token(TokenType::Int(1)),
                token(TokenType::Operator(Op::DoubleColon), (3, 8), (3, 10)),
                dummy_token(TokenType::Operator(Op::CloseSquareBracket)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::ListRangeAccessIncomplete,
                pos: Position::new(3, 10)
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn list_index_ok() {
        let ctx = TestCtx::new();
        assert_eq!(
            ListAccessExpr::new(
                Value::List(vec![Value::Int(8), Value::Int(9), Value::Int(10)]).into(),
                IndexOrRange::index(Value::Int(0).into())
            )
            .eval(&ctx)
            .unwrap(),
            Value::Int(8)
        );
        assert_eq!(
            ListAccessExpr::new(
                Value::List(vec![Value::Int(8), Value::Int(9), Value::Int(10)]).into(),
                IndexOrRange::index(Value::Int(1).into())
            )
            .eval(&ctx)
            .unwrap(),
            Value::Int(9)
        );
    }

    #[test]
    fn list_index_fail() {
        let ctx = TestCtx::new();
        assert_eq!(
            ListAccessExpr::new(
                Value::List(vec![]).into(),
                IndexOrRange::index(Value::Int(0).into())
            )
            .eval(&ctx)
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::IndexOutOfBounds,
        );
        assert_eq!(
            ListAccessExpr::new(
                Value::List(vec![Value::Int(8), Value::Int(9), Value::Int(10)]).into(),
                IndexOrRange::index(Value::Int(4).into())
            )
            .eval(&ctx)
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::IndexOutOfBounds,
        );
        assert_eq!(
            ListAccessExpr::new(
                Value::List(vec![Value::Int(8), Value::Int(9), Value::Int(10)]).into(),
                IndexOrRange::index(Value::Float(4.0).into())
            )
            .eval(&ctx)
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::NonIntegerIndex,
        );
        assert_eq!(
            ListAccessExpr::new(
                Value::Int(8).into(),
                IndexOrRange::index(Value::Int(4).into())
            )
            .eval(&ctx)
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::UnsupportedListAccess,
        );
    }

    #[test]
    fn list_range_ok() {
        let ctx = TestCtx::new();
        assert_eq!(
            ListAccessExpr::new(
                Value::List(vec![Value::Int(8), Value::Int(9), Value::Int(10)]).into(),
                IndexOrRange::range(Value::Int(0).into(), Value::Int(3).into())
            )
            .eval(&ctx)
            .unwrap(),
            Value::List(vec![Value::Int(8), Value::Int(9), Value::Int(10)])
        );
        assert_eq!(
            ListAccessExpr::new(
                Value::List(vec![Value::Int(8), Value::Int(9), Value::Int(10)]).into(),
                IndexOrRange::range(Value::Int(1).into(), Value::Int(2).into())
            )
            .eval(&ctx)
            .unwrap(),
            Value::List(vec![Value::Int(9)])
        );
    }

    #[test]
    fn list_range_fail() {
        let ctx = TestCtx::new();
        assert_eq!(
            ListAccessExpr::new(
                Value::List(vec![]).into(),
                IndexOrRange::range(Value::Int(1).into(), Value::Int(2).into())
            )
            .eval(&ctx)
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::IndexOutOfBounds,
        );
        assert_eq!(
            ListAccessExpr::new(
                Value::List(vec![Value::Int(8), Value::Int(9), Value::Int(10)]).into(),
                IndexOrRange::range(Value::Int(0).into(), Value::Int(7).into())
            )
            .eval(&ctx)
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::IndexOutOfBounds,
        );
        assert_eq!(
            ListAccessExpr::new(
                Value::List(vec![Value::Int(8), Value::Int(9), Value::Int(10)]).into(),
                IndexOrRange::range(Value::Int(0).into(), Value::Float(1.0).into())
            )
            .eval(&ctx)
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::NonIntegerIndex,
        );
        assert_eq!(
            ListAccessExpr::new(
                Value::List(vec![Value::Int(8), Value::Int(9), Value::Int(10)]).into(),
                IndexOrRange::range(Value::Float(0.0).into(), Value::Int(2).into())
            )
            .eval(&ctx)
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::NonIntegerIndex,
        );
        assert_eq!(
            ListAccessExpr::new(
                Value::Int(8).into(),
                IndexOrRange::range(Value::Int(0).into(), Value::Int(2).into())
            )
            .eval(&ctx)
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::UnsupportedListAccess,
        );
    }

    #[test]
    fn string_index_ok() {
        let ctx = TestCtx::new();
        assert_eq!(
            ListAccessExpr::new(
                Value::String("abcd".to_owned()).into(),
                IndexOrRange::index(Value::Int(0).into())
            )
            .eval(&ctx)
            .unwrap(),
            Value::String("a".to_owned())
        );
        assert_eq!(
            ListAccessExpr::new(
                Value::String("abcd".to_owned()).into(),
                IndexOrRange::index(Value::Int(1).into())
            )
            .eval(&ctx)
            .unwrap(),
            Value::String("b".to_owned())
        );
    }

    #[test]
    fn string_index_fail() {
        let ctx = TestCtx::new();
        assert_eq!(
            ListAccessExpr::new(
                Value::String("a".to_owned()).into(),
                IndexOrRange::index(Value::Int(-1).into())
            )
            .eval(&ctx)
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::IndexOutOfBounds,
        );
        assert_eq!(
            ListAccessExpr::new(
                Value::String("abcd".to_owned()).into(),
                IndexOrRange::index(Value::Int(4).into())
            )
            .eval(&ctx)
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::IndexOutOfBounds,
        );
    }

    #[test]
    fn string_range_ok() {
        let ctx = TestCtx::new();
        assert_eq!(
            ListAccessExpr::new(
                Value::String("abcd".to_owned()).into(),
                IndexOrRange::range(Value::Int(0).into(), Value::Int(3).into())
            )
            .eval(&ctx)
            .unwrap(),
            Value::String("abc".to_owned())
        );
        assert_eq!(
            ListAccessExpr::new(
                Value::String("abcd".to_owned()).into(),
                IndexOrRange::range(Value::Int(1).into(), Value::Int(2).into())
            )
            .eval(&ctx)
            .unwrap(),
            Value::String("b".to_owned())
        );
    }

    #[test]
    fn string_range_fail() {
        let ctx = TestCtx::new();
        assert_eq!(
            ListAccessExpr::new(
                Value::String("a".to_owned()).into(),
                IndexOrRange::range(Value::Int(-1).into(), Value::Int(2).into())
            )
            .eval(&ctx)
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::IndexOutOfBounds,
        );
        assert_eq!(
            ListAccessExpr::new(
                Value::String("abcd".to_owned()).into(),
                IndexOrRange::range(Value::Int(0).into(), Value::Int(7).into())
            )
            .eval(&ctx)
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::IndexOutOfBounds,
        );
    }
}
