use crate::parser::grammar::{function::FunctionDefinition, Value};

use super::{context::Context, types::validate_type, ExecutionError, ExecutionErrorVariant};

pub trait Callable {
    fn call(&self, ctx: &dyn Context, args: Vec<Value>) -> Result<Value, ExecutionError>;
}

impl Callable for FunctionDefinition {
    fn call(&self, ctx: &dyn Context, args: Vec<Value>) -> Result<Value, ExecutionError> {
        if self.params.len() != args.len() {
            return Err(ExecutionError::new(ExecutionErrorVariant::InvalidArgumentCount));
        }
        for (parameter, argument) in self.params.iter().zip(args.iter()) {
            validate_type(parameter.data_type, argument)?;
        }

        todo!()
    }
}
