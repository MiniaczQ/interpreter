use std::{cell::RefCell, collections::HashMap};

use crate::parser::grammar::{function::FunctionDefinition, program::Program, Value};

use super::{ExecutionError, ExecutionErrorVariant};

pub trait Context {
    fn get_variable(&self, id: &str) -> Result<Value, ExecutionError>;
    fn set_variable(&self, id: &str, value: Value) -> Result<(), ExecutionError>;
    fn new_variable(&self, id: &str, value: Value) -> Result<(), ExecutionError>;
    fn call_function(&self, id: &str, args: Vec<Value>) -> Result<Value, ExecutionError>;
}

pub struct ProgramCtx {
    functions: HashMap<String, FunctionDefinition>,
}

impl From<Program> for ProgramCtx {
    fn from(p: Program) -> Self {
        ProgramCtx {
            functions: p.functions,
        }
    }
}

impl Context for ProgramCtx {
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

    fn new_variable(&self, id: &str, value: Value) -> Result<(), ExecutionError> {
        Err(ExecutionError::new(
            ExecutionErrorVariant::VariableAlreadyExists,
        ))
    }

    fn call_function(&self, id: &str, args: Vec<Value>) -> Result<Value, ExecutionError> {
        Err(ExecutionError::new(
            ExecutionErrorVariant::FunctionDoesNotExist,
        ))
    }
}

pub struct FunctionCtx<'a> {
    parent: &'a mut dyn Context,
    variables: RefCell<HashMap<String, Value>>,
}

impl Context for FunctionCtx<'_> {
    fn get_variable(&self, id: &str) -> Result<Value, ExecutionError> {
        if let Some(v) = self.variables.borrow().get(id) {
            Ok(v.clone())
        } else {
            self.parent.get_variable(id)
        }
    }

    fn set_variable(&self, id: &str, value: Value) -> Result<(), ExecutionError> {
        if let Some(v) = self.variables.borrow_mut().get_mut(id) {
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
}
