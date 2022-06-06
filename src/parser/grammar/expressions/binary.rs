use crate::{
    interpreter::{context::Context, types::validate_types, ExecutionError, ExecutionErrorVariant},
    parser::grammar::{test_utils::tests::TokenType, Value},
};

use super::{super::utility::*, unary::parse_unary_operator_expression, Evaluable, Expression};

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum BinaryOperator {
    Multiplication,
    Division,
    Modulo,
    Addition,
    Subtraction,
    Equal,
    Unequal,
    Lesser,
    LesserEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
}

/// Binary operation expression
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BinaryExpr {
    lhs: Box<Expression>,
    operator: BinaryOperator,
    rhs: Box<Expression>,
}

impl BinaryExpr {
    pub fn new(lhs: Expression, operator: BinaryOperator, rhs: Expression) -> Self {
        Self {
            lhs: Box::new(lhs),
            operator,
            rhs: Box::new(rhs),
        }
    }
}

impl From<BinaryExpr> for Expression {
    fn from(e: BinaryExpr) -> Self {
        Expression::Binary(e)
    }
}

impl Evaluable for BinaryExpr {
    fn eval(&self, ctx: &dyn Context) -> Result<Value, ExecutionError> {
        let lhs = self.lhs.eval(ctx)?;
        let rhs = self.rhs.eval(ctx)?;
        validate_types(&lhs, &rhs)?;
        match (lhs, rhs) {
            (Value::Int(lhs), Value::Int(rhs)) => int_op(lhs, rhs, self.operator),
            (Value::Float(lhs), Value::Float(rhs)) => float_op(lhs, rhs, self.operator),
            (Value::Bool(lhs), Value::Bool(rhs)) => bool_op(lhs, rhs, self.operator),
            (Value::String(lhs), Value::String(rhs)) => string_op(lhs, rhs, self.operator),
            _ => Err(ExecutionError::new(
                ExecutionErrorVariant::UnsupportedBinaryOperation,
            )),
        }
    }
}

/// mul_div_operators
///     = OP_MULTIPLICATION | OP_DIVISION | OP_REMAINDER
///     ;
fn parse_mul_div_operators(p: &mut Parser) -> OptRes<BinaryOperator> {
    match p.curr().token_type {
        TokenType::Operator(Op::Asterisk) => {
            p.pop();
            Ok(Some(BinaryOperator::Multiplication))
        }
        TokenType::Operator(Op::Slash) => {
            p.pop();
            Ok(Some(BinaryOperator::Division))
        }
        TokenType::Operator(Op::Modulo) => {
            p.pop();
            Ok(Some(BinaryOperator::Modulo))
        }
        _ => Ok(None),
    }
}

/// mul_div_expression
///     = unary_operator_expression, {mul_div_operators, unary_operator_expression}
///     ;
fn parse_mul_div_expression(p: &mut Parser) -> OptRes<Expression> {
    if let Some(mut lhs) = parse_unary_operator_expression(p)? {
        while let Some(operator) = parse_mul_div_operators(p)? {
            let rhs = parse_unary_operator_expression(p)?
                .ok_or_else(|| p.error(ErroVar::BinaryOperatorMissingRHS))?;
            lhs = BinaryExpr::new(lhs, operator, rhs).into();
        }
        Ok(Some(lhs))
    } else {
        Ok(None)
    }
}

/// add_sub_operators
///     = OP_PLUS | OP_MINUS
///     ;
fn parse_add_sub_operators(p: &mut Parser) -> OptRes<BinaryOperator> {
    match p.curr().token_type {
        TokenType::Operator(Op::Plus) => {
            p.pop();
            Ok(Some(BinaryOperator::Addition))
        }
        TokenType::Operator(Op::Minus) => {
            p.pop();
            Ok(Some(BinaryOperator::Subtraction))
        }
        _ => Ok(None),
    }
}

/// add_sub_expression
///     = mul_div_expression, {add_sub_operators, mul_div_expression}
///     ;
fn parse_add_sub_expression(p: &mut Parser) -> OptRes<Expression> {
    if let Some(mut lhs) = parse_mul_div_expression(p)? {
        while let Some(operator) = parse_add_sub_operators(p)? {
            let rhs = parse_mul_div_expression(p)?
                .ok_or_else(|| p.error(ErroVar::BinaryOperatorMissingRHS))?;
            lhs = BinaryExpr::new(lhs, operator, rhs).into();
        }
        return Ok(Some(lhs));
    }
    Ok(None)
}

/// comparison_operators
///     = OP_EQUAL | OP_UNEQUAL | OP_LESSER | OP_LESSER_EQUAL | OP_GREATER | OP_GREATER_EQUAL
///     ;
fn parse_comparison_operators(p: &mut Parser) -> OptRes<BinaryOperator> {
    match p.curr().token_type {
        TokenType::Operator(Op::DoubleEqual) => {
            p.pop();
            Ok(Some(BinaryOperator::Equal))
        }
        TokenType::Operator(Op::Unequal) => {
            p.pop();
            Ok(Some(BinaryOperator::Unequal))
        }
        TokenType::Operator(Op::Lesser) => {
            p.pop();
            Ok(Some(BinaryOperator::Lesser))
        }
        TokenType::Operator(Op::LesserEqual) => {
            p.pop();
            Ok(Some(BinaryOperator::LesserEqual))
        }
        TokenType::Operator(Op::Greater) => {
            p.pop();
            Ok(Some(BinaryOperator::Greater))
        }
        TokenType::Operator(Op::GreaterEqual) => {
            p.pop();
            Ok(Some(BinaryOperator::GreaterEqual))
        }
        _ => Ok(None),
    }
}

/// comparison_expression
///     = add_sub_expression, {comparison_operators, add_sub_expression}
///     ;
fn parse_comparison_expression(p: &mut Parser) -> OptRes<Expression> {
    if let Some(mut lhs) = parse_add_sub_expression(p)? {
        while let Some(operator) = parse_comparison_operators(p)? {
            let rhs = parse_add_sub_expression(p)?
                .ok_or_else(|| p.error(ErroVar::BinaryOperatorMissingRHS))?;
            lhs = BinaryExpr::new(lhs, operator, rhs).into();
        }
        return Ok(Some(lhs));
    }
    Ok(None)
}

/// logical_conjunction_expression
///     = comparison_expression, {OP_AND, comparison_expression}
///     ;
fn parse_logical_conjunction_expression(p: &mut Parser) -> OptRes<Expression> {
    if let Some(mut lhs) = parse_comparison_expression(p)? {
        while p.operator(Op::And)? {
            let rhs = parse_comparison_expression(p)?
                .ok_or_else(|| p.error(ErroVar::BinaryOperatorMissingRHS))?;
            lhs = BinaryExpr::new(lhs, BinaryOperator::And, rhs).into();
        }
        return Ok(Some(lhs));
    }
    Ok(None)
}

/// logical_alternative_expression
///     = logical_conjunction_expression, {OP_OR, logical_conjunction_expression}
///     ;
pub fn parse_logical_alternative_expression(p: &mut Parser) -> OptRes<Expression> {
    if let Some(mut lhs) = parse_logical_conjunction_expression(p)? {
        while p.operator(Op::Or)? {
            let rhs = parse_logical_conjunction_expression(p)?
                .ok_or_else(|| p.error(ErroVar::BinaryOperatorMissingRHS))?;
            lhs = BinaryExpr::new(lhs, BinaryOperator::Or, rhs).into();
        }
        return Ok(Some(lhs));
    }
    Ok(None)
}

fn int_op(lhs: i64, rhs: i64, op: BinaryOperator) -> Result<Value, ExecutionError> {
    match op {
        BinaryOperator::Addition => lhs
            .checked_add(rhs)
            .map(Value::Int)
            .ok_or_else(|| ExecutionError::new(ExecutionErrorVariant::Overflow)),
        BinaryOperator::Subtraction => lhs
            .checked_sub(rhs)
            .map(Value::Int)
            .ok_or_else(|| ExecutionError::new(ExecutionErrorVariant::Overflow)),
        BinaryOperator::Multiplication => lhs
            .checked_mul(rhs)
            .map(Value::Int)
            .ok_or_else(|| ExecutionError::new(ExecutionErrorVariant::Overflow)),
        BinaryOperator::Division => {
            if rhs == 0 {
                return Err(ExecutionError::new(ExecutionErrorVariant::DivisionByZero));
            }
            lhs.checked_div(rhs)
                .map(Value::Int)
                .ok_or_else(|| ExecutionError::new(ExecutionErrorVariant::Overflow))
        }
        BinaryOperator::Modulo => {
            if rhs == 0 {
                return Err(ExecutionError::new(ExecutionErrorVariant::DivisionByZero));
            }
            lhs.checked_rem(rhs)
                .map(Value::Int)
                .ok_or_else(|| ExecutionError::new(ExecutionErrorVariant::Overflow))
        }
        BinaryOperator::Equal => Ok(Value::Bool(lhs == rhs)),
        BinaryOperator::Unequal => Ok(Value::Bool(lhs != rhs)),
        BinaryOperator::Lesser => Ok(Value::Bool(lhs < rhs)),
        BinaryOperator::LesserEqual => Ok(Value::Bool(lhs <= rhs)),
        BinaryOperator::Greater => Ok(Value::Bool(lhs > rhs)),
        BinaryOperator::GreaterEqual => Ok(Value::Bool(lhs >= rhs)),
        _ => Err(ExecutionError::new(
            ExecutionErrorVariant::UnsupportedBinaryOperation,
        )),
    }
}

fn float_op(lhs: f64, rhs: f64, op: BinaryOperator) -> Result<Value, ExecutionError> {
    match op {
        BinaryOperator::Addition => Ok(Value::Float(lhs + rhs)),
        BinaryOperator::Subtraction => Ok(Value::Float(lhs - rhs)),
        BinaryOperator::Multiplication => Ok(Value::Float(lhs * rhs)),
        BinaryOperator::Division => {
            if rhs == 0.0 {
                return Err(ExecutionError::new(ExecutionErrorVariant::DivisionByZero));
            }
            Ok(Value::Float(lhs / rhs))
        }
        BinaryOperator::Equal => Ok(Value::Bool(lhs == rhs)),
        BinaryOperator::Unequal => Ok(Value::Bool(lhs != rhs)),
        BinaryOperator::Lesser => Ok(Value::Bool(lhs < rhs)),
        BinaryOperator::LesserEqual => Ok(Value::Bool(lhs <= rhs)),
        BinaryOperator::Greater => Ok(Value::Bool(lhs > rhs)),
        BinaryOperator::GreaterEqual => Ok(Value::Bool(lhs >= rhs)),
        _ => Err(ExecutionError::new(
            ExecutionErrorVariant::UnsupportedBinaryOperation,
        )),
    }
}

fn bool_op(lhs: bool, rhs: bool, op: BinaryOperator) -> Result<Value, ExecutionError> {
    match op {
        BinaryOperator::Equal => Ok(Value::Bool(lhs == rhs)),
        BinaryOperator::Unequal => Ok(Value::Bool(lhs != rhs)),
        BinaryOperator::And => Ok(Value::Bool(lhs & rhs)),
        BinaryOperator::Or => Ok(Value::Bool(lhs | rhs)),
        _ => Err(ExecutionError::new(
            ExecutionErrorVariant::UnsupportedBinaryOperation,
        )),
    }
}

fn string_op(lhs: String, rhs: String, op: BinaryOperator) -> Result<Value, ExecutionError> {
    match op {
        BinaryOperator::Addition => Ok(Value::String(lhs + &rhs)),
        BinaryOperator::Equal => Ok(Value::Bool(lhs == rhs)),
        BinaryOperator::Unequal => Ok(Value::Bool(lhs != rhs)),
        _ => Err(ExecutionError::new(
            ExecutionErrorVariant::UnsupportedBinaryOperation,
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        interpreter::{test_utils::tests::TestCtx, ExecutionErrorVariant},
        parser::grammar::expressions::parse_expression,
    };

    use super::{super::super::test_utils::tests::*, BinaryExpr, BinaryOperator};

    fn parse_helper(
        t_operator: TokenType,
        t_literal: TokenType,
        operator: BinaryOperator,
        literal: Value,
    ) {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(t_literal.clone()),
                dummy_token(t_operator.clone()),
                dummy_token(t_literal.clone()),
                dummy_token(t_operator),
                dummy_token(t_literal),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            BinaryExpr::new(
                BinaryExpr::new(literal.clone().into(), operator, literal.clone().into()).into(),
                operator,
                literal.into()
            )
            .into()
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn parse_multiplication() {
        parse_helper(
            TokenType::Operator(Op::Asterisk),
            TokenType::Float(2.71),
            BinaryOperator::Multiplication,
            Value::Float(2.71),
        )
    }

    #[test]
    fn parse_division() {
        parse_helper(
            TokenType::Operator(Op::Slash),
            TokenType::Float(2.71),
            BinaryOperator::Division,
            Value::Float(2.71),
        )
    }

    #[test]
    fn parse_modulo() {
        parse_helper(
            TokenType::Operator(Op::Modulo),
            TokenType::Int(5),
            BinaryOperator::Modulo,
            Value::Int(5),
        )
    }

    #[test]
    fn parse_addition() {
        parse_helper(
            TokenType::Operator(Op::Plus),
            TokenType::Float(2.71),
            BinaryOperator::Addition,
            Value::Float(2.71),
        )
    }

    #[test]
    fn parse_subtraction() {
        parse_helper(
            TokenType::Operator(Op::Minus),
            TokenType::Float(2.71),
            BinaryOperator::Subtraction,
            Value::Float(2.71),
        )
    }

    #[test]
    fn parse_equal() {
        parse_helper(
            TokenType::Operator(Op::DoubleEqual),
            TokenType::String("a".to_owned()),
            BinaryOperator::Equal,
            Value::String("a".to_owned()),
        )
    }

    #[test]
    fn parse_unequal() {
        parse_helper(
            TokenType::Operator(Op::Unequal),
            TokenType::Float(2.71),
            BinaryOperator::Unequal,
            Value::Float(2.71),
        )
    }

    #[test]
    fn parse_lesser() {
        parse_helper(
            TokenType::Operator(Op::Lesser),
            TokenType::Float(2.71),
            BinaryOperator::Lesser,
            Value::Float(2.71),
        )
    }

    #[test]
    fn parse_lesser_equal() {
        parse_helper(
            TokenType::Operator(Op::LesserEqual),
            TokenType::Float(2.71),
            BinaryOperator::LesserEqual,
            Value::Float(2.71),
        )
    }

    #[test]
    fn parse_greater() {
        parse_helper(
            TokenType::Operator(Op::Greater),
            TokenType::Float(2.71),
            BinaryOperator::Greater,
            Value::Float(2.71),
        )
    }

    #[test]
    fn parse_greater_equal() {
        parse_helper(
            TokenType::Operator(Op::GreaterEqual),
            TokenType::Float(2.71),
            BinaryOperator::GreaterEqual,
            Value::Float(2.71),
        )
    }

    #[test]
    fn parse_and() {
        parse_helper(
            TokenType::Operator(Op::And),
            TokenType::Keyword(Kw::True),
            BinaryOperator::And,
            Value::Bool(true),
        )
    }

    #[test]
    fn parse_or() {
        parse_helper(
            TokenType::Operator(Op::Or),
            TokenType::Keyword(Kw::False),
            BinaryOperator::Or,
            Value::Bool(false),
        )
    }

    #[test]
    fn parse_missing_rhs() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Int(45)),
                token(TokenType::Operator(Op::Plus), (5, 9), (5, 10)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::BinaryOperatorMissingRHS,
                pos: Position::new(5, 10),
            }
        );

        assert!(warnings.is_empty());
    }

    fn expr(lhs: Value, operator: BinaryOperator, rhs: Value) -> BinaryExpr {
        BinaryExpr::new(lhs.into(), operator, rhs.into())
    }

    #[test]
    fn eval_int_arithmetic_ok() {
        let ctx = TestCtx::new();
        assert_eq!(
            expr(Value::Int(4), BinaryOperator::Addition, Value::Int(4))
                .eval(&ctx)
                .unwrap(),
            Value::Int(8)
        );
        assert_eq!(
            expr(Value::Int(12), BinaryOperator::Subtraction, Value::Int(4))
                .eval(&ctx)
                .unwrap(),
            Value::Int(8)
        );
        assert_eq!(
            expr(Value::Int(2), BinaryOperator::Multiplication, Value::Int(4))
                .eval(&ctx)
                .unwrap(),
            Value::Int(8)
        );
        assert_eq!(
            expr(Value::Int(32), BinaryOperator::Division, Value::Int(4))
                .eval(&ctx)
                .unwrap(),
            Value::Int(8)
        );
        assert_eq!(
            expr(Value::Int(17), BinaryOperator::Modulo, Value::Int(9))
                .eval(&ctx)
                .unwrap(),
            Value::Int(8)
        );
    }

    #[test]
    fn eval_int_arithmetic_fail() {
        let ctx = TestCtx::new();
        assert_eq!(
            expr(Value::Int(8), BinaryOperator::Division, Value::Int(0))
                .eval(&ctx)
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::DivisionByZero
        );
        assert_eq!(
            expr(Value::Int(8), BinaryOperator::Modulo, Value::Int(0))
                .eval(&ctx)
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::DivisionByZero
        );
        assert_eq!(
            expr(
                Value::Int(i64::MAX),
                BinaryOperator::Addition,
                Value::Int(1)
            )
            .eval(&ctx)
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::Overflow
        );
        assert_eq!(
            expr(
                Value::Int(i64::MAX),
                BinaryOperator::Subtraction,
                Value::Int(-1)
            )
            .eval(&ctx)
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::Overflow
        );
        assert_eq!(
            expr(
                Value::Int(i64::MAX),
                BinaryOperator::Multiplication,
                Value::Int(2)
            )
            .eval(&ctx)
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::Overflow
        );
    }

    #[test]
    fn eval_int_comparison_ok() {
        let ctx = TestCtx::new();
        assert_eq!(
            expr(Value::Int(4), BinaryOperator::Equal, Value::Int(4))
                .eval(&ctx)
                .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            expr(Value::Int(8), BinaryOperator::Equal, Value::Int(4))
                .eval(&ctx)
                .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            expr(Value::Int(8), BinaryOperator::Unequal, Value::Int(4))
                .eval(&ctx)
                .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            expr(Value::Int(4), BinaryOperator::Unequal, Value::Int(4))
                .eval(&ctx)
                .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            expr(Value::Int(8), BinaryOperator::Lesser, Value::Int(4))
                .eval(&ctx)
                .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            expr(Value::Int(4), BinaryOperator::Lesser, Value::Int(4))
                .eval(&ctx)
                .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            expr(Value::Int(4), BinaryOperator::Lesser, Value::Int(8))
                .eval(&ctx)
                .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            expr(Value::Int(8), BinaryOperator::LesserEqual, Value::Int(4))
                .eval(&ctx)
                .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            expr(Value::Int(4), BinaryOperator::LesserEqual, Value::Int(4))
                .eval(&ctx)
                .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            expr(Value::Int(4), BinaryOperator::LesserEqual, Value::Int(8))
                .eval(&ctx)
                .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            expr(Value::Int(4), BinaryOperator::Greater, Value::Int(8))
                .eval(&ctx)
                .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            expr(Value::Int(4), BinaryOperator::Greater, Value::Int(4))
                .eval(&ctx)
                .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            expr(Value::Int(8), BinaryOperator::Greater, Value::Int(4))
                .eval(&ctx)
                .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            expr(Value::Int(4), BinaryOperator::GreaterEqual, Value::Int(8))
                .eval(&ctx)
                .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            expr(Value::Int(4), BinaryOperator::GreaterEqual, Value::Int(4))
                .eval(&ctx)
                .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            expr(Value::Int(8), BinaryOperator::GreaterEqual, Value::Int(4))
                .eval(&ctx)
                .unwrap(),
            Value::Bool(true)
        );
    }

    #[test]
    fn eval_float_arithmetic_ok() {
        let ctx = TestCtx::new();
        assert_eq!(
            expr(
                Value::Float(4.0),
                BinaryOperator::Addition,
                Value::Float(4.0)
            )
            .eval(&ctx)
            .unwrap(),
            Value::Float(8.0)
        );
        assert_eq!(
            expr(
                Value::Float(12.0),
                BinaryOperator::Subtraction,
                Value::Float(4.0)
            )
            .eval(&ctx)
            .unwrap(),
            Value::Float(8.0)
        );
        assert_eq!(
            expr(
                Value::Float(2.0),
                BinaryOperator::Multiplication,
                Value::Float(4.0)
            )
            .eval(&ctx)
            .unwrap(),
            Value::Float(8.0)
        );
        assert_eq!(
            expr(
                Value::Float(32.0),
                BinaryOperator::Division,
                Value::Float(4.0)
            )
            .eval(&ctx)
            .unwrap(),
            Value::Float(8.0)
        );
    }

    #[test]
    fn eval_float_comparison_ok() {
        let ctx = TestCtx::new();
        assert_eq!(
            expr(Value::Float(4.0), BinaryOperator::Equal, Value::Float(4.0))
                .eval(&ctx)
                .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            expr(Value::Float(8.0), BinaryOperator::Equal, Value::Float(4.0))
                .eval(&ctx)
                .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            expr(
                Value::Float(8.0),
                BinaryOperator::Unequal,
                Value::Float(4.0)
            )
            .eval(&ctx)
            .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            expr(
                Value::Float(4.0),
                BinaryOperator::Unequal,
                Value::Float(4.0)
            )
            .eval(&ctx)
            .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            expr(Value::Float(8.0), BinaryOperator::Lesser, Value::Float(4.0))
                .eval(&ctx)
                .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            expr(Value::Float(4.0), BinaryOperator::Lesser, Value::Float(4.0))
                .eval(&ctx)
                .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            expr(Value::Float(4.0), BinaryOperator::Lesser, Value::Float(8.0))
                .eval(&ctx)
                .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            expr(
                Value::Float(8.0),
                BinaryOperator::LesserEqual,
                Value::Float(4.0)
            )
            .eval(&ctx)
            .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            expr(
                Value::Float(4.0),
                BinaryOperator::LesserEqual,
                Value::Float(4.0)
            )
            .eval(&ctx)
            .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            expr(
                Value::Float(4.0),
                BinaryOperator::LesserEqual,
                Value::Float(8.0)
            )
            .eval(&ctx)
            .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            expr(
                Value::Float(4.0),
                BinaryOperator::Greater,
                Value::Float(8.0)
            )
            .eval(&ctx)
            .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            expr(
                Value::Float(4.0),
                BinaryOperator::Greater,
                Value::Float(4.0)
            )
            .eval(&ctx)
            .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            expr(
                Value::Float(8.0),
                BinaryOperator::Greater,
                Value::Float(4.0)
            )
            .eval(&ctx)
            .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            expr(
                Value::Float(4.0),
                BinaryOperator::GreaterEqual,
                Value::Float(8.0)
            )
            .eval(&ctx)
            .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            expr(
                Value::Float(4.0),
                BinaryOperator::GreaterEqual,
                Value::Float(4.0)
            )
            .eval(&ctx)
            .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            expr(
                Value::Float(8.0),
                BinaryOperator::GreaterEqual,
                Value::Float(4.0)
            )
            .eval(&ctx)
            .unwrap(),
            Value::Bool(true)
        );
    }

    #[test]
    fn eval_float_arithmetic_fail() {
        let ctx = TestCtx::new();
        assert_eq!(
            expr(
                Value::Float(8.0),
                BinaryOperator::Division,
                Value::Float(0.0)
            )
            .eval(&ctx)
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::DivisionByZero
        );
    }

    #[test]
    fn eval_bool_ok() {
        let ctx = TestCtx::new();
        assert_eq!(
            expr(Value::Bool(true), BinaryOperator::Equal, Value::Bool(true))
                .eval(&ctx)
                .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            expr(
                Value::Bool(true),
                BinaryOperator::Unequal,
                Value::Bool(true)
            )
            .eval(&ctx)
            .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            expr(Value::Bool(true), BinaryOperator::And, Value::Bool(true))
                .eval(&ctx)
                .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            expr(Value::Bool(true), BinaryOperator::And, Value::Bool(false))
                .eval(&ctx)
                .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            expr(Value::Bool(false), BinaryOperator::Or, Value::Bool(true))
                .eval(&ctx)
                .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            expr(Value::Bool(false), BinaryOperator::Or, Value::Bool(false))
                .eval(&ctx)
                .unwrap(),
            Value::Bool(false)
        );
    }

    #[test]
    fn eval_string_ok() {
        let ctx = TestCtx::new();
        assert_eq!(
            expr(
                Value::String("abc".to_owned()),
                BinaryOperator::Equal,
                Value::String("abc".to_owned())
            )
            .eval(&ctx)
            .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            expr(
                Value::String("abc".to_owned()),
                BinaryOperator::Unequal,
                Value::String("abc".to_owned())
            )
            .eval(&ctx)
            .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            expr(
                Value::String("abc".to_owned()),
                BinaryOperator::Addition,
                Value::String("abc".to_owned())
            )
            .eval(&ctx)
            .unwrap(),
            Value::String("abcabc".to_owned())
        );
    }

    #[test]
    fn eval_fail() {
        let ctx = TestCtx::new();
        assert_eq!(
            expr(
                Value::String("abc".to_owned()),
                BinaryOperator::Equal,
                Value::Int(0)
            )
            .eval(&ctx)
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::InvalidType
        );
        assert_eq!(
            expr(Value::Float(0.0), BinaryOperator::Equal, Value::Int(0))
                .eval(&ctx)
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::InvalidType
        );
        assert_eq!(
            expr(Value::Float(0.0), BinaryOperator::Equal, Value::Bool(true))
                .eval(&ctx)
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::InvalidType
        );
        assert_eq!(
            expr(Value::None, BinaryOperator::Equal, Value::Bool(true))
                .eval(&ctx)
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::InvalidType
        );
        assert_eq!(
            expr(Value::None, BinaryOperator::Equal, Value::List(vec![]))
                .eval(&ctx)
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::InvalidType
        );
        assert_eq!(
            expr(
                Value::String("abc".to_owned()),
                BinaryOperator::Subtraction,
                Value::String("abc".to_owned())
            )
            .eval(&ctx)
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::UnsupportedBinaryOperation
        );
        assert_eq!(
            expr(Value::Int(0), BinaryOperator::And, Value::Int(0))
                .eval(&ctx)
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::UnsupportedBinaryOperation
        );
        assert_eq!(
            expr(Value::Float(0.0), BinaryOperator::And, Value::Float(0.0))
                .eval(&ctx)
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::UnsupportedBinaryOperation
        );
        assert_eq!(
            expr(
                Value::Bool(true),
                BinaryOperator::Addition,
                Value::Bool(true)
            )
            .eval(&ctx)
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::UnsupportedBinaryOperation
        );
        assert_eq!(
            expr(
                Value::List(vec![]),
                BinaryOperator::Addition,
                Value::List(vec![])
            )
            .eval(&ctx)
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::UnsupportedBinaryOperation
        );
        assert_eq!(
            expr(Value::None, BinaryOperator::Addition, Value::None)
                .eval(&ctx)
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::InvalidType
        );
    }
}
