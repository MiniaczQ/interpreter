use crate::parser::grammar::Value;

use super::{context::Context, ExecutionError};

/// A type that can be called as a function
pub trait Callable {
    /// Function call with provided arguments
    fn call(&self, ctx: &dyn Context, args: Vec<Value>) -> Result<Value, ExecutionError>;
}
