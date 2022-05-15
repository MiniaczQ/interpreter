use super::{
    function::{parse_function_def, FunctionDef},
    utility::*,
};

/// Main program
#[derive(Debug)]
pub struct Program {
    functions: Vec<FunctionDef>,
}

/// function_definitions
///     = {function_definition}
///     ;
pub fn parse_program(p: &mut Parser) -> Res<Program> {
    let mut functions = vec![];
    while p.has_tokens() {
        if let Some(function) = parse_function_def(p)? {
            functions.push(function);
        } else {
            return p.error(ErroVar::ExpectedFunctionDefinition);
        }
    }
    Ok(Program { functions })
}
