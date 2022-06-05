#[allow(dead_code)]
pub mod tests {
    use std::{cell::RefCell, collections::HashMap};

    use crate::{
        interpreter::{
            context::Context, function::Callable, ExecutionError, ExecutionErrorVariant,
        },
        parser::grammar::Value,
    };

    pub struct TestCtx {
        pub functions: HashMap<String, Box<dyn Callable>>,
        pub variables: RefCell<HashMap<String, Value>>,
    }

    impl TestCtx {
        pub fn new() -> Self {
            Self {
                functions: HashMap::new(),
                variables: RefCell::new(HashMap::new()),
            }
        }
    }

    impl Context for TestCtx {
        fn get_variable(&self, id: &str) -> Result<Value, ExecutionError> {
            if let Some(v) = self.variables.borrow().get(id) {
                Ok(v.clone())
            } else {
                Err(ExecutionError::new(
                    ExecutionErrorVariant::VariableDoesNotExist,
                ))
            }
        }

        fn set_variable(&self, id: &str, value: Value) -> Result<(), ExecutionError> {
            if let Some(v) = self.variables.borrow_mut().get_mut(id) {
                *v = value;
                Ok(())
            } else {
                Err(ExecutionError::new(
                    ExecutionErrorVariant::VariableDoesNotExist,
                ))
            }
        }

        fn new_variable(&self, id: &str, value: Value) -> Result<(), ExecutionError> {
            if !self.variables.borrow().contains_key(id) {
                self.variables.borrow_mut().insert(id.to_owned(), value);
                Ok(())
            } else {
                Err(ExecutionError::new(
                    ExecutionErrorVariant::VariableAlreadyExists,
                ))
            }
        }

        fn call_function(&self, id: &str, args: Vec<Value>) -> Result<Value, ExecutionError> {
            if let Some(func) = self.functions.get(id) {
                func.call(self, args)
            } else {
                Err(ExecutionError::new(
                    ExecutionErrorVariant::FunctionDoesNotExist,
                ))
            }
        }
    }
}
