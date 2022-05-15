use std::fmt::Display;

use ron::ser::PrettyConfig;

use super::{
    function::{parse_function_def, FunctionDef},
    utility::*,
};

/// Main program
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Program {
    pub functions: Vec<FunctionDef>,
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let config = PrettyConfig::new();
        f.write_str(&ron::ser::to_string_pretty(self, config).unwrap())
    }
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

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
