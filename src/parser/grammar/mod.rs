use self::literals::Literal;

use super::CriticalParserError;

mod expressions;
mod function;
mod literals;
mod types;

pub trait Node {}

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

/// A value
#[derive(Clone)]
pub enum Value {
    Integer(i64),
    IntegerList(Vec<i64>),
    Float(f64),
    FloatList(Vec<f64>),
    Bool(bool),
    BoolList(Vec<bool>),
    String(String),
}

type ParseResult<T> = Result<Option<T>, CriticalParserError>;

/// Result extension for simpler parser control flow.
pub trait ExtResult<T> {
    /// In simple terms, if self if:
    /// - an error      - returns the error
    /// - is none       - returns fallback result
    /// - is a value    - returns the value
    fn alt(self, fallback: impl FnOnce() -> ParseResult<T>) -> ParseResult<T>;
}

impl<T> ExtResult<T> for ParseResult<T> {
    fn alt(self, fallback: impl FnOnce() -> ParseResult<T>) -> ParseResult<T> {
        if let Ok(opt) = self {
            if opt.is_some() {
                Ok(opt)
            } else {
                fallback()
            }
        } else {
            self
        }
    }
}

pub struct Function {}