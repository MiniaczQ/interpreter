use std::{
    collections::HashMap,
    fmt::{Debug, Display}, io::stdout,
};

use ron::ser::PrettyConfig;

use crate::interpreter::{
    callable::Callable, context::Context, standard_library::{StandardCtx, PrintOuts}, ExecutionError,
    ExecutionErrorVariant,
};

use super::{
    function::{parse_function_def, FunctionDefinition},
    utility::*,
    DataType, Value,
};

/// Main program
#[derive(Serialize)]
pub struct Program {
    #[serde(skip_serializing)]
    pub std_ctx: StandardCtx,
    functions: HashMap<String, FunctionDefinition>,
}

impl Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProgramCtx")
            .field("functions", &self.functions)
            .finish()
    }
}

impl PartialEq for Program {
    fn eq(&self, other: &Self) -> bool {
        self.functions == other.functions
    }
}

impl Program {
    pub fn new(functions: HashMap<String, FunctionDefinition>) -> Self {
        Self {
            std_ctx: StandardCtx::new(PrintOuts::Std(stdout())),
            functions,
        }
    }

    pub fn run(&self) -> Result<(), ExecutionError> {
        if let Some(main) = self.functions.get("main") {
            if main.data_type != DataType::None {
                return Err(ExecutionError::new(ExecutionErrorVariant::InvalidType));
            }
            if !main.params.is_empty() {
                return Err(ExecutionError::new(
                    ExecutionErrorVariant::InvalidArgumentCount,
                ));
            }
            main.call(self, vec![])?;
            Ok(())
        } else {
            Err(ExecutionError::new(
                ExecutionErrorVariant::MissingMainFunction,
            ))
        }
    }
}

impl Context for Program {
    fn escalate_error(&self, r: Result<Value, ExecutionError>) -> Result<Value, ExecutionError> {
        r
    }

    fn ret(&self, _value: Value) {
        unreachable!()
    }

    fn is_ret(&self) -> bool {
        unreachable!()
    }

    fn call_function(&self, id: &str, args: Vec<Value>) -> Result<Value, ExecutionError> {
        if let Some(func) = self.functions.get(id) {
            func.call(self, args)
        } else {
            self.std_ctx.call_function(id, args)
        }
    }

    fn name(&self) -> String {
        unreachable!()
    }
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
    Ok(Program::new(functions))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::parser::grammar::{
        function::FunctionDefinition,
        program::{parse_program, Program},
        DataType,
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
            FunctionDefinition::new("a".to_owned(), vec![], vec![], DataType::None),
        );
        assert_eq!(result.unwrap(), Program::new(functions));

        assert!(warnings.is_empty());
    }

    #[test]
    fn non_empty() {
        let (result, warnings) = partial_parse_non_opt(vec![], parse_program);
        let functions = HashMap::new();
        assert_eq!(result.unwrap(), Program::new(functions));

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
            FunctionDefinition::new("a".to_owned(), vec![], vec![], DataType::None),
        );
        assert_eq!(result.unwrap(), Program::new(functions));

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
