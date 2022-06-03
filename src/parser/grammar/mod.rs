use serde::{Deserialize, Serialize};

use self::expressions::Expression;

pub mod code_block;
pub mod conditional;
pub mod expressions;
pub mod function;
pub mod literals;
pub mod loops;
pub mod program;
mod test_utils;
pub mod types;
mod utility;

/// A value
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    IntegerList(Vec<i64>),
    FloatList(Vec<f64>),
    BoolList(Vec<bool>),
    String(String),
    None,
}

/// Possible data types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataType {
    Integer,
    Float,
    Bool,
    IntegerList,
    FloatList,
    BoolList,
    String,
    Any,
    None,
}
