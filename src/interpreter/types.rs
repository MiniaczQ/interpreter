use crate::parser::grammar::{DataType, Value};

pub fn valid_type(t: DataType, v: &Value) -> bool {
    match (t, v) {
        (DataType::Integer, Value::Int(_)) => true,
        (DataType::Float, Value::Float(_)) => true,
        (DataType::Bool, Value::Bool(_)) => true,
        (DataType::String, Value::String(_)) => true,
        (DataType::IntegerList, Value::IntegerList(_)) => true,
        (DataType::FloatList, Value::FloatList(_)) => true,
        (DataType::BoolList, Value::BoolList(_)) => true,
        (DataType::None, Value::None) => true,
        (DataType::Any, Value::None) => false,
        (DataType::Any, _) => true,
        _ => false,
    }
}
