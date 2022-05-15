use crate::parser::{ErrorHandler, ExtScannable, Parser, ParserError, ParserErrorVariant};

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
    loop {
        if let Err(ParserError {
            err: ParserErrorVariant::OutOfTokens,
            pos: _,
        }) = p.token()
        {
            break;
        }
        match parse_function_def(p) {
            Ok(Some(function)) => {
                functions.push(function);
            }
            _ => return p.error(ParserErrorVariant::ExpectedFunctionDefinition),
        }
    }
    Ok(Program { functions })
}
