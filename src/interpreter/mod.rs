pub mod callable;
pub mod context;
pub mod standard_library;
pub mod test_utils;
pub mod types;

use std::{error::Error, fmt::Display};

/// Different kinds of interpretation errors
#[derive(Debug, PartialEq, Eq)]
pub enum ExecutionErrorVariant {
    VariableDoesNotExist,
    VariableAlreadyExists,
    FunctionDoesNotExist,

    UnsupportedBinaryOperation,
    UnsupportedUnaryOperation,
    UnsupportedListAccess,
    NonIntegerIndex,
    IndexOutOfBounds,

    InvalidArgumentCount,
    InvalidType,

    CastFailed,

    DivisionByZero,
    Overflow,

    MissingMainFunction,
    ExpectedIdentifier,

    ExpectedSemicolon,
}

/// Interpretation error with stack trace
#[derive(Debug, PartialEq, Eq)]
pub struct ExecutionError {
    pub contexts: Vec<String>,
    pub variant: ExecutionErrorVariant,
}

impl ExecutionError {
    pub fn new(variant: ExecutionErrorVariant) -> Self {
        Self {
            contexts: vec![],
            variant,
        }
    }
}

impl Display for ExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for context in &self.contexts {
            f.write_fmt(format_args!("In `{context}` context.\n"))?;
        }
        f.write_fmt(format_args!(
            "Encountered runtime error {:?}.\n",
            self.variant
        ))
    }
}

impl Error for ExecutionError {}
