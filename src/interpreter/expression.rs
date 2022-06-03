use crate::parser::grammar::{expressions::Expression, Value, literals::Literal};

use super::{context::Context, ExecutionError};

impl Expression {
    pub fn evaluate(&self, ctx: &mut dyn Context) -> Result<Value, ExecutionError> {
        match self {
            Expression::Literal(literal) => eval_literal(ctx, literal),
            Expression::List(list) => todo!(),
            Expression::Identifier(identifier) => eval_identifier(ctx, identifier),
            Expression::ListAccess { list, access } => todo!(),
            Expression::FunctionCall {
                identifier,
                arguments,
            } => todo!(),
            Expression::UnaryOperation {
                operator,
                expression,
            } => todo!(),
            Expression::BinaryOperation { operator, lhs, rhs } => todo!(),
            Expression::Assignment {
                identifier,
                expression,
            } => todo!(),
            Expression::Return(ret) => todo!(),
            Expression::Declaration {
                identifier,
                data_type,
                expression,
            } => todo!(),
            Expression::For(for_loop) => todo!(),
            Expression::While(for_loop) => todo!(),
            Expression::IfElse(if_else) => todo!(),
            Expression::CodeBlock(code_block) => todo!(),
        }
    }
}

fn eval_literal(ctx: &mut dyn Context, literal: &Literal) -> Result<Value, ExecutionError> {
    Ok(literal.0.clone())
}

fn eval_list(ctx: &mut dyn Context, list: Vec<Expression>) -> Result<Value, ExecutionError> {
    todo!()
}

fn eval_identifier(ctx: &mut dyn Context, identifier: &String) -> Result<Value, ExecutionError> {
    Ok(ctx.variable(identifier))
}
