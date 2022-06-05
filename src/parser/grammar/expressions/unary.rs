use crate::{
    interpreter::{context::Context, ExecutionError, ExecutionErrorVariant},
    parser::grammar::{test_utils::tests::TokenType, Value},
};

use super::{super::utility::*, list_access::parse_list_access_expression, Evaluable, Expression};

/// Algebraic negation and logical negation
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum UnaryOperator {
    AlgebraicNegation,
    LogicalNegation,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UnaryExpr {
    operator: UnaryOperator,
    expression: Box<Expression>,
}

impl UnaryExpr {
    pub fn new(operator: UnaryOperator, expression: Expression) -> Self {
        Self {
            operator,
            expression: Box::new(expression),
        }
    }
}

impl From<UnaryExpr> for Expression {
    fn from(e: UnaryExpr) -> Self {
        Expression::Unary(e)
    }
}

impl Evaluable for UnaryExpr {
    fn eval(&self, ctx: &dyn Context) -> Result<Value, ExecutionError> {
        let value = self.expression.eval(ctx)?;
        match (self.operator, value) {
            (UnaryOperator::AlgebraicNegation, Value::Int(value)) => Ok(Value::Int(-value)),
            (UnaryOperator::AlgebraicNegation, Value::Float(value)) => Ok(Value::Float(-value)),
            (UnaryOperator::LogicalNegation, Value::Bool(value)) => Ok(Value::Bool(!value)),
            _ => Err(ExecutionError::new(
                ExecutionErrorVariant::UnsupportedUnaryOperation,
            )),
        }
    }
}

/// unary_operators
///     = OP_NEGATE | OP_MINUS
///     ;
fn parse_unary_operators(p: &mut Parser) -> OptRes<UnaryOperator> {
    match p.curr().token_type {
        TokenType::Operator(Op::Minus) => {
            p.pop();
            Ok(Some(UnaryOperator::AlgebraicNegation))
        }
        TokenType::Operator(Op::ExclamationMark) => {
            p.pop();
            Ok(Some(UnaryOperator::LogicalNegation))
        }
        _ => Ok(None),
    }
}

/// unary_operator_expression
///     = {unary_operators}, list_access_expression
///     ;
pub fn parse_unary_operator_expression(p: &mut Parser) -> OptRes<Expression> {
    if let Some(operator) = parse_unary_operators(p)? {
        let expression = parse_unary_operator_expression(p)?
            .ok_or_else(|| p.error(ErroVar::UnaryOperatorMissingExpression))?;
        Ok(Some(UnaryExpr::new(operator, expression).into()))
    } else if let Some(expression) = parse_list_access_expression(p)? {
        Ok(Some(expression))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        interpreter::{test_utils::tests::TestCtx, ExecutionErrorVariant},
        parser::grammar::expressions::parse_expression,
    };

    use super::{super::super::test_utils::tests::*, UnaryExpr, UnaryOperator};

    fn parse_helper(
        t_operator: TokenType,
        t_literal: TokenType,
        operator: UnaryOperator,
        value: Value,
    ) {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(t_operator.clone()),
                dummy_token(t_operator),
                dummy_token(t_literal),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            UnaryExpr::new(operator, UnaryExpr::new(operator, value.into()).into()).into()
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn parse_algebraic_negation() {
        parse_helper(
            TokenType::Operator(Op::Minus),
            TokenType::Float(5.37),
            UnaryOperator::AlgebraicNegation,
            Value::Float(5.37),
        )
    }

    #[test]
    fn parse_logical_negation() {
        parse_helper(
            TokenType::Operator(Op::ExclamationMark),
            TokenType::Keyword(Kw::True),
            UnaryOperator::LogicalNegation,
            Value::Bool(true),
        )
    }

    #[test]
    fn parse_missing_expression() {
        let (result, warnings) = partial_parse(
            vec![
                token(TokenType::Operator(Op::ExclamationMark), (5, 7), (5, 8)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::UnaryOperatorMissingExpression,
                pos: Position::new(5, 8),
            }
        );

        assert!(warnings.is_empty());
    }

    fn expr(operator: UnaryOperator, value: Value) -> UnaryExpr {
        UnaryExpr::new(operator, value.into())
    }

    #[test]
    fn eval_ok() {
        let ctx = TestCtx::new();
        assert_eq!(
            expr(UnaryOperator::AlgebraicNegation, Value::Int(8))
                .eval(&ctx)
                .unwrap(),
            Value::Int(-8)
        );
        assert_eq!(
            expr(UnaryOperator::AlgebraicNegation, Value::Float(8.0))
                .eval(&ctx)
                .unwrap(),
            Value::Float(-8.0)
        );
        assert_eq!(
            expr(UnaryOperator::LogicalNegation, Value::Bool(true))
                .eval(&ctx)
                .unwrap(),
            Value::Bool(false)
        );
    }

    #[test]
    fn eval_fail() {
        let ctx = TestCtx::new();
        assert_eq!(
            expr(UnaryOperator::LogicalNegation, Value::Int(8))
                .eval(&ctx)
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::UnsupportedUnaryOperation
        );
        assert_eq!(
            expr(UnaryOperator::AlgebraicNegation, Value::Bool(true))
                .eval(&ctx)
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::UnsupportedUnaryOperation
        );
    }
}
