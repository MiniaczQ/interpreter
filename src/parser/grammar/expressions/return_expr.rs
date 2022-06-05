use crate::{
    interpreter::{context::Context, ExecutionError},
    parser::grammar::Value,
};

use super::{super::utility::*, parse_control_flow_expression, Evaluable, Expression};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ReturnExpr(Option<Box<Expression>>);

impl ReturnExpr {
    pub fn new(value: Expression) -> Self {
        Self(Some(Box::new(value)))
    }

    pub fn empty() -> Self {
        Self(None)
    }
}

impl From<ReturnExpr> for Expression {
    fn from(e: ReturnExpr) -> Self {
        Expression::Return(e)
    }
}

impl Evaluable for ReturnExpr {
    fn eval(&self, ctx: &dyn Context) -> Result<Value, ExecutionError> {
        if let Some(value) = &self.0 {
            let value = value.eval(ctx)?;
            ctx.ret(value);
        } else {
            ctx.ret(Value::None);
        }
        Ok(Value::None)
    }
}

/// KW_RETURN
pub fn parse_return(p: &mut Parser) -> OptRes<Expression> {
    if !p.keyword(Kw::Return)? {
        return Ok(None);
    }
    if let Some(expression) = parse_control_flow_expression(p)? {
        Ok(Some(ReturnExpr::new(expression).into()))
    } else {
        Ok(Some(ReturnExpr::empty().into()))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::grammar::expressions::{parse_expression, return_expr::ReturnExpr};

    use super::super::super::test_utils::tests::*;

    #[test]
    fn return_expr() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::Return)),
                dummy_token(TokenType::Int(0)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            ReturnExpr::new(Value::Int(0).into()).into()
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn return_no_expression() {
        let (result, warnings) = partial_parse(
            vec![
                token(TokenType::Keyword(Kw::Return), (4, 2), (4, 8)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(result.unwrap().unwrap(), ReturnExpr::empty().into());

        assert!(warnings.is_empty());
    }
}
