use crate::parser::Parser;

use super::{
    function::{parse_function_def, FunctionDef},
    ParseResult,
};

pub struct Program {
    functions: Vec<FunctionDef>,
}

/// function_definitions
///     = {function_definition}
///     ;
pub fn parse_program(p: &mut Parser) -> ParseResult<Program> {
    let mut functions = vec![];
    while let Some(function) = parse_function_def(p)? {
        functions.push(function);
    }
    Ok(Some(Program { functions }))
}
