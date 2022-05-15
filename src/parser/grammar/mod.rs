use serde::{Deserialize, Serialize};

use self::expressions::Expression;

pub mod code_block;
pub mod conditional;
pub mod expressions;
pub mod function;
pub mod literals;
pub mod loops;
pub mod program;
pub mod types;
mod utility;

/// A value
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Value {
    List(Vec<Expression>),
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),
}

/// Possible data types
#[derive(Debug, Clone, Serialize, Deserialize)]
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
