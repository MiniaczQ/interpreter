use crate::parser::{Parser, ParserError};

use super::function::{parse_function_def, FunctionDef};

#[derive(Debug)]
pub struct Program {
    functions: Vec<FunctionDef>,
}

/// function_definitions
///     = {function_definition}
///     ;
pub fn parse_program(p: &mut Parser) -> Result<Program, ParserError> {
    let mut functions = vec![];
    while let Some(function) = parse_function_def(p)? {
        functions.push(function);
    }
    Ok(Program { functions })
}
