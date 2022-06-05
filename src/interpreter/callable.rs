use crate::parser::grammar::Value;

use super::{context::Context, ExecutionError};

pub trait Callable {
    fn call(&self, ctx: &dyn Context, args: Vec<Value>) -> Result<Value, ExecutionError>;
}
