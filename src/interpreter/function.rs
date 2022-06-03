use crate::parser::grammar::Value;

use super::ExecutionError;

pub trait Callable {
    fn call(args: Vec<Value>) -> Result<Value, ExecutionError>;
}
