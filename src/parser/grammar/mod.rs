use self::expressions::Expression;

use super::ParserError;

pub mod code_block;
pub mod conditional;
pub mod expressions;
pub mod function;
pub mod literals;
pub mod loops;
pub mod program;
pub mod types;

pub trait Node {}

/// A value
#[derive(Clone, Debug)]
pub enum Value {
    List(Vec<Expression>),
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),
}

type ParseResult<T> = Result<Option<T>, ParserError>;

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
