use std::ops::Index;

use ron::value;
use utf8_chars::BufReadCharsExt;

use crate::parser::grammar::{
    code_block::CodeBlock,
    conditional::IfElse,
    expressions::{BinaryOperator, Expression, IndexOrRange, UnaryOperator},
    literals::Literal,
    loops::{ForLoop, WhileLoop},
    DataType, Value,
};

use super::{
    context::Context,
    types::{validate_type, validate_types},
    ExecutionError, ExecutionErrorVariant,
};

impl Expression {
    pub fn evaluate(&self, ctx: &dyn Context) -> Result<Value, ExecutionError> {
        match self {
            Expression::Literal(literal) => eval_literal(ctx, literal),
            Expression::List(list) => eval_list(ctx, list),
            Expression::Identifier(identifier) => eval_identifier(ctx, identifier),
            Expression::ListAccess { list, access } => eval_list_access(ctx, list, access),
            Expression::FunctionCall {
                identifier,
                arguments,
            } => eval_function_call(ctx, identifier, arguments),
            Expression::UnaryOperation {
                operator,
                expression,
            } => eval_unary(ctx, *operator, expression),
            Expression::BinaryOperation { operator, lhs, rhs } => {
                eval_binary(ctx, *operator, lhs, rhs)
            }
            Expression::Assignment {
                identifier,
                expression,
            } => eval_assignment(ctx, identifier, expression),
            Expression::Return(ret) => eval_return(ctx, ret),
            Expression::Declaration {
                identifier,
                data_type,
                expression,
            } => eval_declaration(ctx, identifier, *data_type, expression),
            Expression::For(for_loop) => eval_for_loop(ctx, for_loop),
            Expression::While(while_loop) => eval_while_loop(ctx, while_loop),
            Expression::IfElse(if_else) => eval_ifelse(ctx, if_else),
            Expression::CodeBlock(code_block) => eval_codeblock(ctx, code_block),
        }
    }
}

fn eval_while_loop(ctx: &dyn Context, while_loop: &WhileLoop) -> Result<Value, ExecutionError> {
    todo!()
    // Custom ctx
}

fn eval_for_loop(ctx: &dyn Context, for_loop: &ForLoop) -> Result<Value, ExecutionError> {
    todo!()
    // Custom ctx
}

fn eval_ifelse(ctx: &dyn Context, if_else: &IfElse) -> Result<Value, ExecutionError> {
    todo!()
    // Custom ctx?
}

fn eval_codeblock(ctx: &dyn Context, code_block: &CodeBlock) -> Result<Value, ExecutionError> {
    todo!()
    // Custom ctx
}

fn eval_declaration(
    ctx: &dyn Context,
    identifier: &str,
    data_type: DataType,
    expression: &Expression,
) -> Result<Value, ExecutionError> {
    let value = expression.evaluate(ctx)?;
    validate_type(data_type, &value)?;
    ctx.new_variable(identifier, value)?;
    Ok(Value::None)
}

fn eval_return(ctx: &dyn Context, ret: &Expression) -> Result<Value, ExecutionError> {
    todo!()
    // Ctx support
}

fn eval_binary(
    ctx: &dyn Context,
    operator: BinaryOperator,
    lhs: &Expression,
    rhs: &Expression,
) -> Result<Value, ExecutionError> {
    let lhs = lhs.evaluate(ctx)?;
    let rhs = rhs.evaluate(ctx)?;
    validate_types(&lhs, &rhs)?;
    match (lhs, operator, rhs) {
        (Value::Int(lhs), BinaryOperator::Addition, Value::Int(rhs)) => Ok(Value::Int(lhs + rhs)),
        (Value::Int(lhs), BinaryOperator::Subtraction, Value::Int(rhs)) => {
            Ok(Value::Int(lhs - rhs))
        }
        (Value::Int(lhs), BinaryOperator::Multiplication, Value::Int(rhs)) => {
            Ok(Value::Int(lhs * rhs))
        }
        (Value::Int(lhs), BinaryOperator::Division, Value::Int(rhs)) => {
            if rhs == 0 {
                return Err(ExecutionError::new(ExecutionErrorVariant::DivisionByZero));
            }
            Ok(Value::Int(lhs / rhs))
        }
        (Value::Int(lhs), BinaryOperator::Modulo, Value::Int(rhs)) => {
            if rhs == 0 {
                return Err(ExecutionError::new(ExecutionErrorVariant::DivisionByZero));
            }
            Ok(Value::Int(lhs % rhs))
        }

        (Value::Int(lhs), BinaryOperator::Equal, Value::Int(rhs)) => Ok(Value::Bool(lhs == rhs)),
        (Value::Int(lhs), BinaryOperator::Unequal, Value::Int(rhs)) => Ok(Value::Bool(lhs != rhs)),
        (Value::Int(lhs), BinaryOperator::Lesser, Value::Int(rhs)) => Ok(Value::Bool(lhs < rhs)),
        (Value::Int(lhs), BinaryOperator::LesserEqual, Value::Int(rhs)) => {
            Ok(Value::Bool(lhs <= rhs))
        }
        (Value::Int(lhs), BinaryOperator::Greater, Value::Int(rhs)) => Ok(Value::Bool(lhs > rhs)),
        (Value::Int(lhs), BinaryOperator::GreaterEqual, Value::Int(rhs)) => {
            Ok(Value::Bool(lhs >= rhs))
        }

        (Value::Float(lhs), BinaryOperator::Addition, Value::Float(rhs)) => {
            Ok(Value::Float(lhs + rhs))
        }
        (Value::Float(lhs), BinaryOperator::Subtraction, Value::Float(rhs)) => {
            Ok(Value::Float(lhs - rhs))
        }
        (Value::Float(lhs), BinaryOperator::Multiplication, Value::Float(rhs)) => {
            Ok(Value::Float(lhs * rhs))
        }
        (Value::Float(lhs), BinaryOperator::Division, Value::Float(rhs)) => {
            if rhs == 0.0 {
                return Err(ExecutionError::new(ExecutionErrorVariant::DivisionByZero));
            }
            Ok(Value::Float(lhs / rhs))
        }

        (Value::Float(lhs), BinaryOperator::Equal, Value::Float(rhs)) => {
            Ok(Value::Bool(lhs == rhs))
        }
        (Value::Float(lhs), BinaryOperator::Unequal, Value::Float(rhs)) => {
            Ok(Value::Bool(lhs != rhs))
        }
        (Value::Float(lhs), BinaryOperator::Lesser, Value::Float(rhs)) => {
            Ok(Value::Bool(lhs < rhs))
        }
        (Value::Float(lhs), BinaryOperator::LesserEqual, Value::Float(rhs)) => {
            Ok(Value::Bool(lhs <= rhs))
        }
        (Value::Float(lhs), BinaryOperator::Greater, Value::Float(rhs)) => {
            Ok(Value::Bool(lhs > rhs))
        }
        (Value::Float(lhs), BinaryOperator::GreaterEqual, Value::Float(rhs)) => {
            Ok(Value::Bool(lhs >= rhs))
        }

        (Value::Bool(lhs), BinaryOperator::Equal, Value::Bool(rhs)) => Ok(Value::Bool(lhs == rhs)),
        (Value::Bool(lhs), BinaryOperator::Unequal, Value::Bool(rhs)) => {
            Ok(Value::Bool(lhs != rhs))
        }
        (Value::Bool(lhs), BinaryOperator::And, Value::Bool(rhs)) => Ok(Value::Bool(lhs & rhs)),
        (Value::Bool(lhs), BinaryOperator::Or, Value::Bool(rhs)) => Ok(Value::Bool(lhs | rhs)),

        (Value::String(lhs), BinaryOperator::Addition, Value::String(rhs)) => {
            Ok(Value::String(lhs + &rhs))
        }

        (Value::String(lhs), BinaryOperator::Equal, Value::String(rhs)) => {
            Ok(Value::Bool(lhs == rhs))
        }
        (Value::String(lhs), BinaryOperator::Unequal, Value::String(rhs)) => {
            Ok(Value::Bool(lhs != rhs))
        }

        _ => Err(ExecutionError::new(
            ExecutionErrorVariant::UnsupportedBinaryOperation,
        )),
    }
}

fn eval_unary(
    ctx: &dyn Context,
    operator: UnaryOperator,
    expression: &Expression,
) -> Result<Value, ExecutionError> {
    let value = expression.evaluate(ctx)?;
    match (operator, value) {
        (UnaryOperator::AlgebraicNegation, Value::Int(value)) => Ok(Value::Int(-value)),
        (UnaryOperator::AlgebraicNegation, Value::Float(value)) => Ok(Value::Float(-value)),
        (UnaryOperator::LogicalNegation, Value::Bool(value)) => Ok(Value::Bool(!value)),
        _ => Err(ExecutionError::new(
            ExecutionErrorVariant::UnsupportedUnaryOperation,
        )),
    }
}

fn eval_assignment(
    ctx: &dyn Context,
    identifier: &Expression,
    expression: &Expression,
) -> Result<Value, ExecutionError> {
    todo!()
    // Valid identifier expressions are identifier and list access
}

fn eval_function_call(
    ctx: &dyn Context,
    identifier: &Expression,
    arguments: &Vec<Expression>,
) -> Result<Value, ExecutionError> {
    todo!()
    // identifier has to be identifier
}

fn eval_list_access(
    ctx: &dyn Context,
    list: &Expression,
    access: &IndexOrRange,
) -> Result<Value, ExecutionError> {
    let value = list.evaluate(ctx)?;
    if let Value::List(list) = value {
        match access {
            IndexOrRange::Index(idx) => eval_list_index(ctx, list, idx),
            IndexOrRange::Range(lidx, ridx) => {
                eval_list_range(ctx, list, lidx, ridx).map(Value::List)
            }
        }
    } else if let Value::String(list) = value {
        let list: Vec<char> = list.chars().collect();
        match access {
            IndexOrRange::Index(idx) => {
                eval_list_index(ctx, list, idx).map(|c| Value::String(c.into()))
            }
            IndexOrRange::Range(lidx, ridx) => eval_list_range(ctx, list, lidx, ridx)
                .map(|v| Value::String(v.into_iter().collect())),
        }
    } else {
        Err(ExecutionError::new(
            ExecutionErrorVariant::UnsupportedListAccess,
        ))
    }
    // Valid list access expressions are strings and lists
    // Valid access indices are integers
    // Returns a smaller list
}

fn eval_list_range<T: Clone>(
    ctx: &dyn Context,
    mut list: Vec<T>,
    lidx: &Expression,
    ridx: &Expression,
) -> Result<Vec<T>, ExecutionError> {
    let list_size = list.len() as i64;
    let lidx = if let Value::Int(lidx) = lidx.evaluate(ctx)? {
        if 0 <= lidx && lidx < list_size {
            lidx as usize
        } else {
            return Err(ExecutionError::new(ExecutionErrorVariant::IndexOutOfBounds));
        }
    } else {
        return Err(ExecutionError::new(ExecutionErrorVariant::NonIntegerIndex));
    };
    let ridx = if let Value::Int(ridx) = ridx.evaluate(ctx)? {
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

fn eval_list_index<T: Clone>(
    ctx: &dyn Context,
    list: Vec<T>,
    idx: &Expression,
) -> Result<T, ExecutionError> {
    let list_size = list.len() as i64;
    let idx = if let Value::Int(idx) = idx.evaluate(ctx)? {
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

fn eval_literal(_ctx: &dyn Context, literal: &Literal) -> Result<Value, ExecutionError> {
    Ok(literal.0.clone())
}

fn eval_list(ctx: &dyn Context, list: &[Expression]) -> Result<Value, ExecutionError> {
    let values: Vec<Value> = list
        .iter()
        .map(|e| e.evaluate(ctx))
        .collect::<Result<_, ExecutionError>>()?;
    Ok(Value::List(values))
}

fn eval_identifier(ctx: &dyn Context, identifier: &str) -> Result<Value, ExecutionError> {
    ctx.get_variable(identifier)
}

#[cfg(test)]
mod tests {
    use crate::{
        interpreter::{
            expression::{
                eval_binary, eval_declaration, eval_list, eval_list_access, eval_literal,
                eval_unary,
            },
            test_utils::tests::TestCtx,
            ExecutionError, ExecutionErrorVariant,
        },
        parser::grammar::{
            expressions::{BinaryOperator, Expression, IndexOrRange, UnaryOperator},
            literals::Literal,
            DataType, Value,
        },
    };

    use super::eval_identifier;

    #[test]
    fn identifier_ok() {
        let ctx = TestCtx::new();
        assert_eq!(
            eval_identifier(&ctx, "a").unwrap_err().variant,
            ExecutionErrorVariant::VariableDoesNotExist
        );
    }

    #[test]
    fn identifier_fail() {
        let ctx = TestCtx::new();
        ctx.variables
            .borrow_mut()
            .insert("a".to_owned(), Value::Int(8));
        assert_eq!(eval_identifier(&ctx, "a").unwrap(), Value::Int(8));
    }

    #[test]
    fn declaration_ok() {
        let ctx = TestCtx::new();
        assert_eq!(
            eval_declaration(
                &ctx,
                "a",
                DataType::Integer,
                &Expression::Literal(Literal(Value::Int(8)))
            )
            .unwrap(),
            Value::None
        );
        assert_eq!(ctx.variables.borrow_mut().get("a").unwrap(), &Value::Int(8));
    }

    #[test]
    fn declaration_fail() {
        let ctx = TestCtx::new();
        ctx.variables
            .borrow_mut()
            .insert("a".to_owned(), Value::Int(8));
        assert_eq!(
            eval_declaration(
                &ctx,
                "a",
                DataType::Integer,
                &Expression::Literal(Literal(Value::Int(8)))
            )
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::VariableAlreadyExists
        );
    }

    #[test]
    fn literal_ok() {
        let ctx = TestCtx::new();
        assert_eq!(
            eval_literal(&ctx, &Literal(Value::Int(8))).unwrap(),
            Value::Int(8)
        );
    }

    #[test]
    fn list_index_ok() {
        let ctx = TestCtx::new();
        assert_eq!(
            eval_list_access(
                &ctx,
                &Expression::Literal(Literal(Value::List(vec![
                    Value::Int(8),
                    Value::Int(9),
                    Value::Int(10)
                ]))),
                &IndexOrRange::Index(Expression::Literal(Literal(Value::Int(0))))
            )
            .unwrap(),
            Value::Int(8)
        );
        assert_eq!(
            eval_list_access(
                &ctx,
                &Expression::Literal(Literal(Value::List(vec![
                    Value::Int(8),
                    Value::Int(9),
                    Value::Int(10)
                ]))),
                &IndexOrRange::Index(Expression::Literal(Literal(Value::Int(1))))
            )
            .unwrap(),
            Value::Int(9)
        );
    }

    #[test]
    fn list_index_fail() {
        let ctx = TestCtx::new();
        assert_eq!(
            eval_list_access(
                &ctx,
                &Expression::Literal(Literal(Value::List(vec![]))),
                &IndexOrRange::Index(Expression::Literal(Literal(Value::Int(0))))
            )
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::IndexOutOfBounds,
        );
        assert_eq!(
            eval_list_access(
                &ctx,
                &Expression::Literal(Literal(Value::List(vec![
                    Value::Int(8),
                    Value::Int(9),
                    Value::Int(10)
                ]))),
                &IndexOrRange::Index(Expression::Literal(Literal(Value::Int(4))))
            )
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::IndexOutOfBounds,
        );
        assert_eq!(
            eval_list_access(
                &ctx,
                &Expression::Literal(Literal(Value::List(vec![
                    Value::Int(8),
                    Value::Int(9),
                    Value::Int(10)
                ]))),
                &IndexOrRange::Index(Expression::Literal(Literal(Value::Float(4.0))))
            )
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::NonIntegerIndex,
        );
        assert_eq!(
            eval_list_access(
                &ctx,
                &Expression::Literal(Literal(Value::Int(8))),
                &IndexOrRange::Index(Expression::Literal(Literal(Value::Int(4))))
            )
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::UnsupportedListAccess,
        );
    }

    #[test]
    fn list_range_ok() {
        let ctx = TestCtx::new();
        assert_eq!(
            eval_list_access(
                &ctx,
                &Expression::Literal(Literal(Value::List(vec![
                    Value::Int(8),
                    Value::Int(9),
                    Value::Int(10)
                ]))),
                &IndexOrRange::Range(
                    Expression::Literal(Literal(Value::Int(0))),
                    Expression::Literal(Literal(Value::Int(3)))
                )
            )
            .unwrap(),
            Value::List(vec![Value::Int(8), Value::Int(9), Value::Int(10)])
        );
        assert_eq!(
            eval_list_access(
                &ctx,
                &Expression::Literal(Literal(Value::List(vec![
                    Value::Int(8),
                    Value::Int(9),
                    Value::Int(10)
                ]))),
                &IndexOrRange::Range(
                    Expression::Literal(Literal(Value::Int(1))),
                    Expression::Literal(Literal(Value::Int(2)))
                )
            )
            .unwrap(),
            Value::List(vec![Value::Int(9)])
        );
    }

    #[test]
    fn list_range_fail() {
        let ctx = TestCtx::new();
        assert_eq!(
            eval_list_access(
                &ctx,
                &Expression::Literal(Literal(Value::List(vec![]))),
                &IndexOrRange::Range(
                    Expression::Literal(Literal(Value::Int(1))),
                    Expression::Literal(Literal(Value::Int(2)))
                )
            )
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::IndexOutOfBounds,
        );
        assert_eq!(
            eval_list_access(
                &ctx,
                &Expression::Literal(Literal(Value::List(vec![
                    Value::Int(8),
                    Value::Int(9),
                    Value::Int(10)
                ]))),
                &IndexOrRange::Range(
                    Expression::Literal(Literal(Value::Int(0))),
                    Expression::Literal(Literal(Value::Int(7)))
                )
            )
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::IndexOutOfBounds,
        );
        assert_eq!(
            eval_list_access(
                &ctx,
                &Expression::Literal(Literal(Value::List(vec![
                    Value::Int(8),
                    Value::Int(9),
                    Value::Int(10)
                ]))),
                &IndexOrRange::Range(
                    Expression::Literal(Literal(Value::Int(0))),
                    Expression::Literal(Literal(Value::Float(1.0)))
                )
            )
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::NonIntegerIndex,
        );
        assert_eq!(
            eval_list_access(
                &ctx,
                &Expression::Literal(Literal(Value::List(vec![
                    Value::Int(8),
                    Value::Int(9),
                    Value::Int(10)
                ]))),
                &IndexOrRange::Range(
                    Expression::Literal(Literal(Value::Float(0.0))),
                    Expression::Literal(Literal(Value::Int(2)))
                )
            )
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::NonIntegerIndex,
        );
        assert_eq!(
            eval_list_access(
                &ctx,
                &Expression::Literal(Literal(Value::Int(8))),
                &IndexOrRange::Range(
                    Expression::Literal(Literal(Value::Int(0))),
                    Expression::Literal(Literal(Value::Int(2)))
                )
            )
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::UnsupportedListAccess,
        );
    }

    #[test]
    fn string_index_ok() {
        let ctx = TestCtx::new();
        assert_eq!(
            eval_list_access(
                &ctx,
                &Expression::Literal(Literal(Value::String("abcd".to_owned()))),
                &IndexOrRange::Index(Expression::Literal(Literal(Value::Int(0))))
            )
            .unwrap(),
            Value::String("a".to_owned())
        );
        assert_eq!(
            eval_list_access(
                &ctx,
                &Expression::Literal(Literal(Value::String("abcd".to_owned()))),
                &IndexOrRange::Index(Expression::Literal(Literal(Value::Int(1))))
            )
            .unwrap(),
            Value::String("b".to_owned())
        );
    }

    #[test]
    fn string_index_fail() {
        let ctx = TestCtx::new();
        assert_eq!(
            eval_list_access(
                &ctx,
                &Expression::Literal(Literal(Value::String("a".to_owned()))),
                &IndexOrRange::Index(Expression::Literal(Literal(Value::Int(-1))))
            )
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::IndexOutOfBounds,
        );
        assert_eq!(
            eval_list_access(
                &ctx,
                &Expression::Literal(Literal(Value::String("abcd".to_owned()))),
                &IndexOrRange::Index(Expression::Literal(Literal(Value::Int(4))))
            )
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::IndexOutOfBounds,
        );
    }

    #[test]
    fn string_range_ok() {
        let ctx = TestCtx::new();
        assert_eq!(
            eval_list_access(
                &ctx,
                &Expression::Literal(Literal(Value::String("abcd".to_owned()))),
                &IndexOrRange::Range(
                    Expression::Literal(Literal(Value::Int(0))),
                    Expression::Literal(Literal(Value::Int(3)))
                )
            )
            .unwrap(),
            Value::String("abc".to_owned())
        );
        assert_eq!(
            eval_list_access(
                &ctx,
                &Expression::Literal(Literal(Value::String("abcd".to_owned()))),
                &IndexOrRange::Range(
                    Expression::Literal(Literal(Value::Int(1))),
                    Expression::Literal(Literal(Value::Int(2)))
                )
            )
            .unwrap(),
            Value::String("b".to_owned())
        );
    }

    #[test]
    fn string_range_fail() {
        let ctx = TestCtx::new();
        assert_eq!(
            eval_list_access(
                &ctx,
                &Expression::Literal(Literal(Value::String("a".to_owned()))),
                &IndexOrRange::Range(
                    Expression::Literal(Literal(Value::Int(-1))),
                    Expression::Literal(Literal(Value::Int(2)))
                )
            )
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::IndexOutOfBounds,
        );
        assert_eq!(
            eval_list_access(
                &ctx,
                &Expression::Literal(Literal(Value::String("abcd".to_owned()))),
                &IndexOrRange::Range(
                    Expression::Literal(Literal(Value::Int(0))),
                    Expression::Literal(Literal(Value::Int(7)))
                )
            )
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::IndexOutOfBounds,
        );
    }

    #[test]
    fn list_ok() {
        let ctx = TestCtx::new();
        assert_eq!(eval_list(&ctx, &[]).unwrap(), Value::List(vec![]));
        assert_eq!(
            eval_list(
                &ctx,
                &[
                    Expression::Literal(Literal(Value::Int(8))),
                    Expression::Literal(Literal(Value::Float(8.0)))
                ]
            )
            .unwrap(),
            Value::List(vec![Value::Int(8), Value::Float(8.0)])
        );
    }

    #[test]
    fn unary_ok() {
        let ctx = TestCtx::new();
        assert_eq!(
            eval_unary(
                &ctx,
                UnaryOperator::AlgebraicNegation,
                &Expression::Literal(Literal(Value::Int(8)))
            )
            .unwrap(),
            Value::Int(-8)
        );
        assert_eq!(
            eval_unary(
                &ctx,
                UnaryOperator::AlgebraicNegation,
                &Expression::Literal(Literal(Value::Float(8.0)))
            )
            .unwrap(),
            Value::Float(-8.0)
        );
        assert_eq!(
            eval_unary(
                &ctx,
                UnaryOperator::LogicalNegation,
                &Expression::Literal(Literal(Value::Bool(true)))
            )
            .unwrap(),
            Value::Bool(false)
        );
    }

    #[test]
    fn unary_fail() {
        let ctx = TestCtx::new();
        assert_eq!(
            eval_unary(
                &ctx,
                UnaryOperator::LogicalNegation,
                &Expression::Literal(Literal(Value::Int(8)))
            )
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::UnsupportedUnaryOperation
        );
        assert_eq!(
            eval_unary(
                &ctx,
                UnaryOperator::AlgebraicNegation,
                &Expression::Literal(Literal(Value::Bool(true)))
            )
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::UnsupportedUnaryOperation
        );
    }

    #[test]
    fn binary_int_arithmetic_ok() {
        let ctx = TestCtx::new();
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Addition,
                &Expression::Literal(Literal(Value::Int(4))),
                &Expression::Literal(Literal(Value::Int(4)))
            )
            .unwrap(),
            Value::Int(8)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Subtraction,
                &Expression::Literal(Literal(Value::Int(12))),
                &Expression::Literal(Literal(Value::Int(4)))
            )
            .unwrap(),
            Value::Int(8)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Multiplication,
                &Expression::Literal(Literal(Value::Int(2))),
                &Expression::Literal(Literal(Value::Int(4)))
            )
            .unwrap(),
            Value::Int(8)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Division,
                &Expression::Literal(Literal(Value::Int(32))),
                &Expression::Literal(Literal(Value::Int(4)))
            )
            .unwrap(),
            Value::Int(8)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Modulo,
                &Expression::Literal(Literal(Value::Int(17))),
                &Expression::Literal(Literal(Value::Int(9)))
            )
            .unwrap(),
            Value::Int(8)
        );
    }

    #[test]
    fn binary_int_comparison_ok() {
        let ctx = TestCtx::new();
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Equal,
                &Expression::Literal(Literal(Value::Int(4))),
                &Expression::Literal(Literal(Value::Int(4)))
            )
            .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Equal,
                &Expression::Literal(Literal(Value::Int(8))),
                &Expression::Literal(Literal(Value::Int(4)))
            )
            .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Unequal,
                &Expression::Literal(Literal(Value::Int(8))),
                &Expression::Literal(Literal(Value::Int(4)))
            )
            .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Unequal,
                &Expression::Literal(Literal(Value::Int(4))),
                &Expression::Literal(Literal(Value::Int(4)))
            )
            .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Lesser,
                &Expression::Literal(Literal(Value::Int(8))),
                &Expression::Literal(Literal(Value::Int(4)))
            )
            .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Lesser,
                &Expression::Literal(Literal(Value::Int(4))),
                &Expression::Literal(Literal(Value::Int(4)))
            )
            .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Lesser,
                &Expression::Literal(Literal(Value::Int(4))),
                &Expression::Literal(Literal(Value::Int(8)))
            )
            .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::LesserEqual,
                &Expression::Literal(Literal(Value::Int(8))),
                &Expression::Literal(Literal(Value::Int(4)))
            )
            .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::LesserEqual,
                &Expression::Literal(Literal(Value::Int(4))),
                &Expression::Literal(Literal(Value::Int(4)))
            )
            .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::LesserEqual,
                &Expression::Literal(Literal(Value::Int(4))),
                &Expression::Literal(Literal(Value::Int(8)))
            )
            .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Greater,
                &Expression::Literal(Literal(Value::Int(4))),
                &Expression::Literal(Literal(Value::Int(8)))
            )
            .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Greater,
                &Expression::Literal(Literal(Value::Int(4))),
                &Expression::Literal(Literal(Value::Int(4)))
            )
            .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Greater,
                &Expression::Literal(Literal(Value::Int(8))),
                &Expression::Literal(Literal(Value::Int(4)))
            )
            .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::GreaterEqual,
                &Expression::Literal(Literal(Value::Int(4))),
                &Expression::Literal(Literal(Value::Int(8)))
            )
            .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::GreaterEqual,
                &Expression::Literal(Literal(Value::Int(4))),
                &Expression::Literal(Literal(Value::Int(4)))
            )
            .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::GreaterEqual,
                &Expression::Literal(Literal(Value::Int(8))),
                &Expression::Literal(Literal(Value::Int(4)))
            )
            .unwrap(),
            Value::Bool(true)
        );
    }

    #[test]
    fn binary_int_arithmetic_fail() {
        let ctx = TestCtx::new();
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Division,
                &Expression::Literal(Literal(Value::Int(8))),
                &Expression::Literal(Literal(Value::Int(0)))
            )
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::DivisionByZero
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Modulo,
                &Expression::Literal(Literal(Value::Int(8))),
                &Expression::Literal(Literal(Value::Int(0)))
            )
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::DivisionByZero
        );
    }

    #[test]
    fn binary_float_arithmetic_ok() {
        let ctx = TestCtx::new();
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Addition,
                &Expression::Literal(Literal(Value::Float(4.0))),
                &Expression::Literal(Literal(Value::Float(4.0)))
            )
            .unwrap(),
            Value::Float(8.0)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Subtraction,
                &Expression::Literal(Literal(Value::Float(12.0))),
                &Expression::Literal(Literal(Value::Float(4.0)))
            )
            .unwrap(),
            Value::Float(8.0)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Multiplication,
                &Expression::Literal(Literal(Value::Float(2.0))),
                &Expression::Literal(Literal(Value::Float(4.0)))
            )
            .unwrap(),
            Value::Float(8.0)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Division,
                &Expression::Literal(Literal(Value::Float(32.0))),
                &Expression::Literal(Literal(Value::Float(4.0)))
            )
            .unwrap(),
            Value::Float(8.0)
        );
    }

    #[test]
    fn binary_float_comparison_ok() {
        let ctx = TestCtx::new();
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Equal,
                &Expression::Literal(Literal(Value::Float(4.0))),
                &Expression::Literal(Literal(Value::Float(4.0)))
            )
            .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Equal,
                &Expression::Literal(Literal(Value::Float(8.0))),
                &Expression::Literal(Literal(Value::Float(4.0)))
            )
            .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Unequal,
                &Expression::Literal(Literal(Value::Float(8.0))),
                &Expression::Literal(Literal(Value::Float(4.0)))
            )
            .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Unequal,
                &Expression::Literal(Literal(Value::Float(4.0))),
                &Expression::Literal(Literal(Value::Float(4.0)))
            )
            .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Lesser,
                &Expression::Literal(Literal(Value::Float(8.0))),
                &Expression::Literal(Literal(Value::Float(4.0)))
            )
            .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Lesser,
                &Expression::Literal(Literal(Value::Float(4.0))),
                &Expression::Literal(Literal(Value::Float(4.0)))
            )
            .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Lesser,
                &Expression::Literal(Literal(Value::Float(4.0))),
                &Expression::Literal(Literal(Value::Float(8.0)))
            )
            .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::LesserEqual,
                &Expression::Literal(Literal(Value::Float(8.0))),
                &Expression::Literal(Literal(Value::Float(4.0)))
            )
            .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::LesserEqual,
                &Expression::Literal(Literal(Value::Float(4.0))),
                &Expression::Literal(Literal(Value::Float(4.0)))
            )
            .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::LesserEqual,
                &Expression::Literal(Literal(Value::Float(4.0))),
                &Expression::Literal(Literal(Value::Float(8.0)))
            )
            .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Greater,
                &Expression::Literal(Literal(Value::Float(4.0))),
                &Expression::Literal(Literal(Value::Float(8.0)))
            )
            .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Greater,
                &Expression::Literal(Literal(Value::Float(4.0))),
                &Expression::Literal(Literal(Value::Float(4.0)))
            )
            .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Greater,
                &Expression::Literal(Literal(Value::Float(8.0))),
                &Expression::Literal(Literal(Value::Float(4.0)))
            )
            .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::GreaterEqual,
                &Expression::Literal(Literal(Value::Float(4.0))),
                &Expression::Literal(Literal(Value::Float(8.0)))
            )
            .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::GreaterEqual,
                &Expression::Literal(Literal(Value::Float(4.0))),
                &Expression::Literal(Literal(Value::Float(4.0)))
            )
            .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::GreaterEqual,
                &Expression::Literal(Literal(Value::Float(8.0))),
                &Expression::Literal(Literal(Value::Float(4.0)))
            )
            .unwrap(),
            Value::Bool(true)
        );
    }

    #[test]
    fn binary_float_arithmetic_fail() {
        let ctx = TestCtx::new();
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Division,
                &Expression::Literal(Literal(Value::Float(8.0))),
                &Expression::Literal(Literal(Value::Float(0.0)))
            )
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::DivisionByZero
        );
    }

    #[test]
    fn binary_bool_ok() {
        let ctx = TestCtx::new();
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Equal,
                &Expression::Literal(Literal(Value::Bool(true))),
                &Expression::Literal(Literal(Value::Bool(true)))
            )
            .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Unequal,
                &Expression::Literal(Literal(Value::Bool(true))),
                &Expression::Literal(Literal(Value::Bool(true)))
            )
            .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::And,
                &Expression::Literal(Literal(Value::Bool(true))),
                &Expression::Literal(Literal(Value::Bool(true)))
            )
            .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::And,
                &Expression::Literal(Literal(Value::Bool(true))),
                &Expression::Literal(Literal(Value::Bool(false)))
            )
            .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Or,
                &Expression::Literal(Literal(Value::Bool(false))),
                &Expression::Literal(Literal(Value::Bool(true)))
            )
            .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Or,
                &Expression::Literal(Literal(Value::Bool(false))),
                &Expression::Literal(Literal(Value::Bool(false)))
            )
            .unwrap(),
            Value::Bool(false)
        );
    }

    #[test]
    fn binary_string_ok() {
        let ctx = TestCtx::new();
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Equal,
                &Expression::Literal(Literal(Value::String("abc".to_owned()))),
                &Expression::Literal(Literal(Value::String("abc".to_owned())))
            )
            .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Unequal,
                &Expression::Literal(Literal(Value::String("abc".to_owned()))),
                &Expression::Literal(Literal(Value::String("abc".to_owned())))
            )
            .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Addition,
                &Expression::Literal(Literal(Value::String("abc".to_owned()))),
                &Expression::Literal(Literal(Value::String("abc".to_owned())))
            )
            .unwrap(),
            Value::String("abcabc".to_owned())
        );
    }

    #[test]
    fn binary_fail() {
        let ctx = TestCtx::new();
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Equal,
                &Expression::Literal(Literal(Value::String("abc".to_owned()))),
                &Expression::Literal(Literal(Value::Int(0)))
            )
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::InvalidType
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Equal,
                &Expression::Literal(Literal(Value::Float(0.0))),
                &Expression::Literal(Literal(Value::Int(0)))
            )
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::InvalidType
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Equal,
                &Expression::Literal(Literal(Value::Float(0.0))),
                &Expression::Literal(Literal(Value::Bool(true)))
            )
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::InvalidType
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Equal,
                &Expression::Literal(Literal(Value::None)),
                &Expression::Literal(Literal(Value::Bool(true)))
            )
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::InvalidType
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Equal,
                &Expression::Literal(Literal(Value::None)),
                &Expression::Literal(Literal(Value::List(vec![])))
            )
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::InvalidType
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Subtraction,
                &Expression::Literal(Literal(Value::String("abc".to_owned()))),
                &Expression::Literal(Literal(Value::String("abc".to_owned())))
            )
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::UnsupportedBinaryOperation
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::And,
                &Expression::Literal(Literal(Value::Int(0))),
                &Expression::Literal(Literal(Value::Int(0)))
            )
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::UnsupportedBinaryOperation
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::And,
                &Expression::Literal(Literal(Value::Float(0.0))),
                &Expression::Literal(Literal(Value::Float(0.0)))
            )
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::UnsupportedBinaryOperation
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Addition,
                &Expression::Literal(Literal(Value::Bool(true))),
                &Expression::Literal(Literal(Value::Bool(true)))
            )
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::UnsupportedBinaryOperation
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Addition,
                &Expression::Literal(Literal(Value::List(vec![]))),
                &Expression::Literal(Literal(Value::List(vec![])))
            )
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::UnsupportedBinaryOperation
        );
        assert_eq!(
            eval_binary(
                &ctx,
                BinaryOperator::Addition,
                &Expression::Literal(Literal(Value::None)),
                &Expression::Literal(Literal(Value::None))
            )
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::InvalidType
        );
    }
}
