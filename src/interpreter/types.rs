use crate::parser::grammar::{DataType, Value};

use super::{ExecutionError, ExecutionErrorVariant};

pub fn validate_type(t: DataType, v: &Value) -> Result<(), ExecutionError> {
    match (t, v) {
        (DataType::Integer, Value::Int(_)) => Ok(()),
        (DataType::Float, Value::Float(_)) => Ok(()),
        (DataType::Bool, Value::Bool(_)) => Ok(()),
        (DataType::String, Value::String(_)) => Ok(()),
        (DataType::List, Value::List(_)) => Ok(()),
        (DataType::None, Value::None) => Ok(()),
        _ => Err(ExecutionError::new(ExecutionErrorVariant::InvalidType)),
    }
}

pub fn validate_types(l: &Value, r: &Value) -> Result<(), ExecutionError> {
    match (r, l) {
        (Value::Int(_), Value::Int(_)) => Ok(()),
        (Value::Float(_), Value::Float(_)) => Ok(()),
        (Value::Bool(_), Value::Bool(_)) => Ok(()),
        (Value::String(_), Value::String(_)) => Ok(()),
        (Value::List(_), Value::List(_)) => Ok(()),
        _ => Err(ExecutionError::new(ExecutionErrorVariant::InvalidType)),
    }
}
