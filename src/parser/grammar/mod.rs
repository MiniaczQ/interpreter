use std::fmt::Display;

use serde::{Deserialize, Serialize};

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
    String(String),
    List(Vec<Value>),
    None,
}

fn display_list(list: &[Value], f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut list = list.iter();
    f.write_str("[")?;
    if let Some(v) = list.next() {
        f.write_fmt(format_args!("{v}"))?;
        for v in list {
            f.write_fmt(format_args!(", {v}"))?;
        }
    }
    f.write_str("]")
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(v) => f.write_fmt(format_args!("{v}")),
            Value::Float(v) => f.write_fmt(format_args!("{v}")),
            Value::Bool(v) => f.write_fmt(format_args!("{v}")),
            Value::String(v) => f.write_fmt(format_args!("{v}")),
            Value::List(v) => display_list(v, f),
            Value::None => f.write_str("None"),
        }
    }
}

/// Possible data types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DataType {
    Integer,
    Float,
    Bool,
    String,
    List,
    None,
}
