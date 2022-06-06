use crate::parser::grammar::Value;

use super::{ExecutionError, ExecutionErrorVariant};

pub trait Context {
    fn get_variable(&self, _id: &str) -> Result<Value, ExecutionError> {
        Err(ExecutionError::new(
            ExecutionErrorVariant::VariableDoesNotExist,
        ))
    }

    fn set_variable(&self, _id: &str, _value: Value) -> Result<(), ExecutionError> {
        Err(ExecutionError::new(
            ExecutionErrorVariant::VariableDoesNotExist,
        ))
    }

    fn new_variable(&self, _identifier: &str, _value: Value) -> Result<(), ExecutionError> {
        Err(ExecutionError::new(
            ExecutionErrorVariant::VariableDoesNotExist,
        ))
    }
    fn escalate_error(&self, r: Result<Value, ExecutionError>) -> Result<Value, ExecutionError> {
        r.map_err(|mut e| {
            e.contexts.push(self.name());
            e
        })
    }
    fn ret(&self, value: Value);
    fn is_ret(&self) -> bool;
    fn call_function(&self, identifier: &str, args: Vec<Value>) -> Result<Value, ExecutionError>;
    fn name(&self) -> String;
}
