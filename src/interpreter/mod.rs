mod context;
mod function;
mod standard;
mod types;
mod expression;

use std::collections::HashMap;

use crate::parser::grammar::{function::FunctionDef, program::Program, Value};

use self::context::ProgramCtx;

pub fn run(p: Program) {
    let ctx: ProgramCtx = p.into();
}

pub struct ExecutionError {}
