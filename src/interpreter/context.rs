use std::{cell::RefCell, collections::HashMap};

use crate::parser::grammar::Value;

use super::{types::validate_types, ExecutionError, ExecutionErrorVariant};

pub trait Context {
    fn get_variable(&self, id: &str) -> Result<Value, ExecutionError> {
        Err(ExecutionError::new(
            ExecutionErrorVariant::VariableDoesNotExist,
        ))
    }

    fn set_variable(&self, id: &str, value: Value) -> Result<(), ExecutionError> {
        Err(ExecutionError::new(
            ExecutionErrorVariant::VariableDoesNotExist,
        ))
    }

    fn new_variable(&self, identifier: &str, value: Value) -> Result<(), ExecutionError> {
        Err(ExecutionError::new(
            ExecutionErrorVariant::VariableDoesNotExist,
        ))
    }
    fn escalate_error(&self, mut e: ExecutionError) -> ExecutionError {
        e.contexts.push(self.name());
        e
    }
    fn call_function(&self, identifier: &str, args: Vec<Value>) -> Result<Value, ExecutionError>;
    fn run(self) -> Result<Value, ExecutionError>;
    fn name(&self) -> String;
}

pub struct BlockCtx<'a> {
    parent: &'a mut dyn Context,
    variables: RefCell<HashMap<String, Value>>,
}

impl Context for BlockCtx<'_> {
    fn get_variable(&self, id: &str) -> Result<Value, ExecutionError> {
        if let Some(v) = self.variables.borrow().get(id) {
            Ok(v.clone())
        } else {
            self.parent.get_variable(id)
        }
    }

    fn set_variable(&self, id: &str, value: Value) -> Result<(), ExecutionError> {
        if let Some(v) = self.variables.borrow_mut().get_mut(id) {
            validate_types(v, &value)?;
            *v = value;
            Ok(())
        } else {
            self.parent.set_variable(id, value)
        }
    }

    fn new_variable(&self, id: &str, value: Value) -> Result<(), ExecutionError> {
        if self.variables.borrow().contains_key(id) {
            return Err(ExecutionError::new(
                ExecutionErrorVariant::VariableAlreadyExists,
            ));
        }
        self.variables.borrow_mut().insert(id.to_owned(), value);
        Ok(())
    }

    fn call_function(&self, id: &str, args: Vec<Value>) -> Result<Value, ExecutionError> {
        self.parent.call_function(id, args)
    }

    fn name(&self) -> String {
        "code block".to_owned()
    }

    fn run(self) -> Result<Value, ExecutionError> {
        todo!()
    }
}
