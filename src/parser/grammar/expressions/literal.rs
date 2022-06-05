use crate::{
    interpreter::{context::Context, ExecutionError},
    parser::grammar::Value,
};

use super::{super::utility::*, Evaluable, Expression};

/// Literal expression
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LiteralExpr(Value);

impl LiteralExpr {
    #[allow(dead_code)]
    pub fn new(v: Value) -> Self {
        Self(v)
    }
}

impl From<LiteralExpr> for Expression {
    fn from(e: LiteralExpr) -> Self {
        Expression::Literal(e)
    }
}

impl From<Value> for Expression {
    fn from(v: Value) -> Self {
        LiteralExpr(v).into()
    }
}

impl Evaluable for LiteralExpr {
    fn eval(&self, _ctx: &dyn Context) -> Result<Value, ExecutionError> {
        Ok(self.0.clone())
    }
}

/// CONST_INT
fn parse_integer(p: &mut Parser) -> OptRes<Value> {
    if let Some(v) = p.integer()? {
        return Ok(Some(Value::Int(v)));
    }
    Ok(None)
}

/// CONST_FLOAT
fn parse_float(p: &mut Parser) -> OptRes<Value> {
    if let Some(v) = p.float()? {
        return Ok(Some(Value::Float(v)));
    }
    Ok(None)
}

/// KW_TRUE | KW_FALSE
fn parse_bool(p: &mut Parser) -> OptRes<Value> {
    if p.keyword(Kw::True)? {
        return Ok(Some(Value::Bool(true)));
    }
    if p.keyword(Kw::False)? {
        return Ok(Some(Value::Bool(false)));
    }
    Ok(None)
}

/// CONST_STRING
fn parse_string(p: &mut Parser) -> OptRes<Value> {
    if let Some(v) = p.string()? {
        return Ok(Some(Value::String(v)));
    }
    Ok(None)
}

/// constant
///     = CONST_INT
///     | CONST_FLOAT
///     | KW_TRUE | KW_FALSE
///     | CONST_STRING
///     ;
pub fn parse_literal_expression(p: &mut Parser) -> OptRes<Expression> {
    parse_integer(p)
        .alt(|| parse_float(p))
        .alt(|| parse_bool(p))
        .alt(|| parse_string(p))
        .map(|v| v.map(|v| LiteralExpr(v).into()))
}

#[cfg(test)]
mod tests {
    use crate::{
        interpreter::test_utils::tests::TestCtx,
        parser::grammar::expressions::{literal::parse_literal_expression, parse_expression},
    };

    use super::{super::super::test_utils::tests::*, LiteralExpr};

    #[test]
    fn parse_miss() {
        let (result, warnings) = partial_parse(
            vec![dummy_token(TokenType::Keyword(Kw::Let))],
            parse_literal_expression,
        );
        assert_eq!(result, Ok(None));

        assert!(warnings.is_empty());
    }

    #[test]
    fn parse_int() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Int(5)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(result.unwrap().unwrap(), LiteralExpr(Value::Int(5)).into());

        assert!(warnings.is_empty());
    }

    #[test]
    fn parse_float() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Float(5.0)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            LiteralExpr(Value::Float(5.0)).into()
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn parse_string() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::String("ada".to_owned())),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            LiteralExpr(Value::String("ada".to_owned())).into()
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn parse_bool_true() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::True)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            LiteralExpr(Value::Bool(true)).into()
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn parse_bool_flase() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::False)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            LiteralExpr(Value::Bool(false)).into()
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn parse_out_of_tokens() {
        let (result, warnings) = partial_parse(vec![], parse_expression);
        assert_eq!(result.unwrap(), None);

        assert!(warnings.is_empty());
    }

    #[test]
    fn eval_ok() {
        let ctx = TestCtx::new();
        assert_eq!(
            LiteralExpr(Value::Int(8)).eval(&ctx).unwrap(),
            Value::Int(8)
        );
    }
}
