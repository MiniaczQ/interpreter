use std::{collections::HashMap, fmt::Display};

use ron::ser::PrettyConfig;

use super::{
    function::{parse_function_def, FunctionDefinition},
    utility::*,
};

/// Main program
#[derive(Debug, Serialize, PartialEq)]
pub struct Program {
    pub functions: HashMap<String, FunctionDefinition>, // HashMap-a
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
    let mut functions = HashMap::new();
    while let Some(function) = parse_function_def(p)? {
        functions.insert(function.identifier.clone(), function);
    }
    Ok(Program { functions })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::parser::grammar::{
        function::FunctionDefinition,
        program::{parse_program, Program},
    };

    use super::super::test_utils::tests::*;

    #[test]
    fn empty() {
        let (result, warnings) = partial_parse_non_opt(
            vec![
                dummy_token(TokenType::Keyword(Kw::Fn)),
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenRoundBracket)),
                dummy_token(TokenType::Operator(Op::CloseRoundBracket)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_program,
        );
        let mut functions = HashMap::new();
        functions.insert(
            "a".to_owned(),
            FunctionDefinition {
                identifier: "a".to_owned(),
                params: vec![],
                statements: vec![],
                data_type: grammar::DataType::None,
            },
        );
        assert_eq!(result.unwrap(), Program { functions });

        assert!(warnings.is_empty());
    }

    #[test]
    fn non_empty() {
        let (result, warnings) = partial_parse_non_opt(vec![], parse_program);
        let functions = HashMap::new();
        assert_eq!(result.unwrap(), Program { functions });

        assert!(warnings.is_empty());
    }

    #[test]
    fn surface_error() {
        let (result, warnings) = partial_parse_non_opt(
            vec![token(TokenType::Keyword(Kw::Fn), (3, 4), (3, 6))],
            parse_program,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::FunctionMissingIdentifier,
                pos: Position::new(3, 6)
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn buffer_warning() {
        let (result, warnings) = partial_parse_non_opt(
            vec![
                dummy_token(TokenType::Keyword(Kw::Fn)),
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenRoundBracket)),
                token(TokenType::Operator(Op::OpenCurlyBracket), (7, 3), (7, 4)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_program,
        );
        let mut functions = HashMap::new();
        functions.insert(
            "a".to_owned(),
            FunctionDefinition {
                identifier: "a".to_owned(),
                params: vec![],
                statements: vec![],
                data_type: grammar::DataType::None,
            },
        );
        assert_eq!(result.unwrap(), Program { functions });

        assert_eq!(warnings.len(), 1);
        assert_eq!(
            warnings[0],
            ParserWarning {
                warning: ParserWarningVariant::MissingClosingRoundBracket,
                start: Position::new(7, 3),
                stop: Position::new(7, 4)
            }
        );
    }
}
