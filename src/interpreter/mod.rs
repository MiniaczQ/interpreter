pub mod context;
pub mod function;
pub mod standard_library;
pub mod test_utils;
pub mod types;

use std::{error::Error, fmt::Display};

use crate::parser::grammar::program::Program;

use self::context::ProgramCtx;

pub fn run(p: Program) {
    let ctx: ProgramCtx = p.into();
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExecutionErrorVariant {
    VariableDoesNotExist,
    VariableAlreadyExists,
    FunctionDoesNotExist,

    UnsupportedBinaryOperation,
    UnsupportedUnaryOperation,
    ExpectedVariable,
    UnsupportedListAccess,
    NonIntegerIndex,
    IndexOutOfBounds,

    InvalidArgumentCount,
    InvalidType,

    CastFailed,

    DivisionByZero,
}

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
            f.write_fmt(format_args!("In `{context}` context."))?;
        }
        f.write_fmt(format_args!(
            "Encountered runtime error {:?}.",
            self.variant
        ))
    }
}

impl Error for ExecutionError {}
