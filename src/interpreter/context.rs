use std::collections::HashMap;

use crate::parser::grammar::{function::FunctionDef, program::Program, Value};

use super::ExecutionError;

pub trait Context {
    fn variable(&mut self, id: &String) -> Result<&mut Value, ExecutionError>;
    fn call_function(&self, id: &String, args: Vec<Value>) -> Result<Value, ExecutionError>;
}

pub struct ProgramCtx {
    std_functions: HashMap<String, fn(Vec<Value>) -> Result<Value, ExecutionError>>,
    functions: HashMap<String, FunctionDef>,
}

impl From<Program> for ProgramCtx {
    fn from(p: Program) -> Self {
        let ctx = ProgramCtx {
            std_functions: HashMap::new(),
            functions: p.functions,
        };

        ctx
    }
}

impl Context for ProgramCtx {
    fn variable(&mut self, id: &String) -> Result<&mut Value, ExecutionError> {
        Err(ExecutionError {})
    }

    fn call_function(&self, id: &String, args: Vec<Value>) -> Result<Value, ExecutionError> {
        Err(ExecutionError {})
    }
}

pub struct FunctionCtx<'a> {
    parent: &'a mut dyn Context,
    variables: HashMap<String, Value>,
}

impl Context for FunctionCtx<'_> {
    fn variable(&mut self, id: &String) -> Result<&mut Value, ExecutionError> {
        if let Some(v) = self.variables.get_mut(id) {
            Ok(v)
        } else {
            self.parent.variable(id)
        }
    }

    fn call_function(&self, id: &String, args: Vec<Value>) -> Result<Value, ExecutionError> {
        self.parent.call_function(id, args)
    }
}

pub struct BlockCtx<'a> {
    parent: &'a mut dyn Context,
    variables: HashMap<String, Value>,
}

impl Context for BlockCtx<'_> {
    fn variable(&mut self, id: &String) -> Result<&mut Value, ExecutionError> {
        if let Some(v) = self.variables.get_mut(id) {
            Ok(v)
        } else {
            self.parent.variable(id)
        }
    }

    fn call_function(&self, id: &String, args: Vec<Value>) -> Result<Value, ExecutionError> {
        self.parent.call_function(id, args)
    }
}
