use crate::parser::grammar::{DataType, Value};

use super::ExecutionError;

pub fn type_checker(
    arg_types: Vec<DataType>,
    args: Vec<Value>,
) -> Result<Vec<Value>, ExecutionError> {
    if arg_types.len() != args.len() {
        return Err(ExecutionError {});
    }

    Ok(args)
}
