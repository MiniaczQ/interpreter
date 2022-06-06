use std::{cell::RefCell, collections::HashMap};

use crate::interpreter::{
    callable::Callable,
    context::Context,
    types::{validate_type, validate_types},
    ExecutionError, ExecutionErrorVariant,
};

use super::{
    expressions::statement::{alternate_statements, parse_code_block, Statement},
    types::parse_type,
    utility::*,
    DataType, Value,
};

/// A single function parameter
#[derive(Debug, Serialize, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub data_type: DataType,
}

impl Parameter {
    #[allow(dead_code)]
    pub fn new(name: String, data_type: DataType) -> Self {
        Self { name, data_type }
    }
}

/// Definition of a function
#[derive(Debug, Serialize, PartialEq)]
pub struct FunctionDefinition {
    pub identifier: String,
    pub params: Vec<Parameter>,
    statements: Vec<Statement>,
    pub data_type: DataType,
}

impl FunctionDefinition {
    #[allow(dead_code)]
    pub fn new(
        identifier: String,
        params: Vec<Parameter>,
        statements: Vec<Statement>,
        data_type: DataType,
    ) -> Self {
        Self {
            identifier,
            params,
            statements,
            data_type,
        }
    }
}

impl Callable for FunctionDefinition {
    fn call(&self, ctx: &dyn Context, args: Vec<Value>) -> Result<Value, ExecutionError> {
        if self.params.len() != args.len() {
            return Err(ExecutionError::new(
                ExecutionErrorVariant::InvalidArgumentCount,
            ));
        }
        let mut variables = HashMap::new();
        for (parameter, argument) in self.params.iter().zip(args.into_iter()) {
            validate_type(parameter.data_type, &argument)?;
            variables.insert(parameter.name.clone(), argument);
        }
        let ctx = FunctionCtx::new(ctx, self.identifier.clone(), variables);
        let returning = alternate_statements(&self.statements, &ctx)?;
        let returning = ctx.returning.replace(None).unwrap_or(returning);
        validate_type(self.data_type, &returning)?;
        Ok(returning)
    }
}

pub struct FunctionCtx<'a> {
    name: String,
    parent: &'a dyn Context,
    returning: RefCell<Option<Value>>,
    variables: RefCell<HashMap<String, Value>>,
}

impl Context for FunctionCtx<'_> {
    fn get_variable(&self, id: &str) -> Result<Value, ExecutionError> {
        if let Some(v) = self.variables.borrow().get(id) {
            Ok(v.clone())
        } else {
            self.parent.get_variable(id)
        }
    }

    fn set_variable(&self, id: &str, value: Value) -> Result<(), ExecutionError> {
        if let Some(v) = self.variables.borrow_mut().get_mut(id) {
            validate_types(v, &value)?;
            *v = value;
            Ok(())
        } else {
            self.parent.set_variable(id, value)
        }
    }

    fn new_variable(&self, id: &str, value: Value) -> Result<(), ExecutionError> {
        if self.variables.borrow().contains_key(id) {
            return Err(ExecutionError::new(
                ExecutionErrorVariant::VariableAlreadyExists,
            ));
        }
        self.variables.borrow_mut().insert(id.to_owned(), value);
        Ok(())
    }

    fn ret(&self, value: Value) {
        *self.returning.borrow_mut() = Some(value);
    }

    fn is_ret(&self) -> bool {
        self.returning.borrow().is_some()
    }

    fn call_function(&self, id: &str, args: Vec<Value>) -> Result<Value, ExecutionError> {
        self.parent.call_function(id, args)
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl<'a> FunctionCtx<'a> {
    pub fn new(ctx: &'a dyn Context, name: String, variables: HashMap<String, Value>) -> Self {
        Self {
            name,
            parent: ctx,
            returning: RefCell::new(None),
            variables: RefCell::new(variables),
        }
    }
}

/// parameter
///     = IDENTIFIER, TYPE_SIGNATURE, type
///     ;
fn parse_parameter(p: &mut Parser) -> OptRes<Parameter> {
    if let Some(name) = p.identifier()? {
        if !p.operator(Op::Colon)? {
            p.warn(WarnVar::MissingColon)?;
        }
        let data_type =
            parse_type(p)?.ok_or_else(|| p.error(ErroVar::FunctionParameterMissingType))?;
        return Ok(Some(Parameter { name, data_type }));
    }
    Ok(None)
}

/// parameters
///     = [parameter, {SPLIT, parameter}]
///     ;
fn parse_parameters(p: &mut Parser) -> Res<Vec<Parameter>> {
    let mut params = vec![];
    if let Some(param) = parse_parameter(p)? {
        params.push(param);
        while p.operator(Op::Split)? {
            if let Some(param) = parse_parameter(p)? {
                if params.iter().any(|par: &Parameter| par.name == param.name) {
                    return Err(p.error(ErroVar::DuplicateParameter));
                }
                params.push(param);
            } else {
                p.warn(WarnVar::ExpectedParameter)?;
            }
        }
    }
    Ok(params)
}

/// function_definition
///     = KW_FN, OPEN_BRACKET, parameters, CLOSE_BRACKET, [RETURN_SIGNATURE, type], code_block
///     ;
pub fn parse_function_def(p: &mut Parser) -> OptRes<FunctionDefinition> {
    if !p.keyword(Kw::Fn)? {
        return Ok(None);
    }
    let identifier = p
        .identifier()?
        .ok_or_else(|| p.error(ErroVar::FunctionMissingIdentifier))?;
    if !p.operator(Op::OpenRoundBracket)? {
        p.warn(WarnVar::MissingOpeningRoundBracket)?;
    }
    let params = parse_parameters(p)?;
    if !p.operator(Op::CloseRoundBracket)? {
        p.warn(WarnVar::MissingClosingRoundBracket)?;
    }
    let data_type = if p.operator(Op::Arrow)? {
        parse_type(p)?.ok_or_else(|| p.error(ErroVar::FunctionMissingReturnType))?
    } else {
        DataType::None
    };
    let code_block = parse_code_block(p)?.ok_or_else(|| p.error(ErroVar::FunctionMissingBody))?;
    Ok(Some(FunctionDefinition {
        identifier,
        params,
        statements: code_block,
        data_type,
    }))
}

#[cfg(test)]
mod tests {
    use crate::parser::grammar::{
        expressions::{
            function_call::FunctionCallExpr, identifier::IdentifierExpr, statement::Statement,
        },
        function::{parse_function_def, FunctionDefinition, Parameter},
    };

    use super::super::test_utils::tests::*;

    #[test]
    fn miss() {
        let (result, warnings) = partial_parse(
            vec![dummy_token(TokenType::Keyword(Kw::Let))],
            parse_function_def,
        );
        assert_eq!(result, Ok(None));

        assert!(warnings.is_empty());
    }

    #[test]
    fn ok() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::Fn)),
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenRoundBracket)),
                dummy_token(TokenType::Identifier("b".to_owned())),
                dummy_token(TokenType::Operator(Op::Colon)),
                dummy_token(TokenType::Keyword(Kw::Int)),
                dummy_token(TokenType::Operator(Op::Split)),
                dummy_token(TokenType::Identifier("c".to_owned())),
                dummy_token(TokenType::Operator(Op::Colon)),
                dummy_token(TokenType::Keyword(Kw::Int)),
                dummy_token(TokenType::Operator(Op::CloseRoundBracket)),
                dummy_token(TokenType::Operator(Op::Arrow)),
                dummy_token(TokenType::Keyword(Kw::Int)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Identifier("d".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenRoundBracket)),
                dummy_token(TokenType::Operator(Op::CloseRoundBracket)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_function_def,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            FunctionDefinition {
                identifier: "a".to_owned(),
                params: vec![
                    Parameter {
                        name: "b".to_owned(),
                        data_type: grammar::DataType::Integer
                    },
                    Parameter {
                        name: "c".to_owned(),
                        data_type: grammar::DataType::Integer
                    }
                ],
                statements: vec![
                    FunctionCallExpr::new(IdentifierExpr::new("d".to_owned()).into(), vec![])
                        .into(),
                    Statement::Semicolon
                ],
                data_type: grammar::DataType::Integer
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn duplicate_parameters() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::Fn)),
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenRoundBracket)),
                dummy_token(TokenType::Identifier("b".to_owned())),
                dummy_token(TokenType::Operator(Op::Colon)),
                dummy_token(TokenType::Keyword(Kw::Int)),
                dummy_token(TokenType::Operator(Op::Split)),
                dummy_token(TokenType::Identifier("b".to_owned())),
                dummy_token(TokenType::Operator(Op::Colon)),
                token(TokenType::Keyword(Kw::Int), (10, 5), (10, 6)),
                dummy_token(TokenType::Operator(Op::CloseRoundBracket)),
                dummy_token(TokenType::Operator(Op::Arrow)),
                dummy_token(TokenType::Keyword(Kw::Int)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Identifier("d".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenRoundBracket)),
                dummy_token(TokenType::Operator(Op::CloseRoundBracket)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_function_def,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::DuplicateParameter,
                pos: Position::new(10, 6),
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn no_parameters() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::Fn)),
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenRoundBracket)),
                dummy_token(TokenType::Operator(Op::CloseRoundBracket)),
                dummy_token(TokenType::Operator(Op::Arrow)),
                dummy_token(TokenType::Keyword(Kw::Int)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Identifier("c".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenRoundBracket)),
                dummy_token(TokenType::Operator(Op::CloseRoundBracket)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_function_def,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            FunctionDefinition {
                identifier: "a".to_owned(),
                params: vec![],
                statements: vec![
                    FunctionCallExpr::new(IdentifierExpr::new("c".to_owned()).into(), vec![])
                        .into(),
                    Statement::Semicolon
                ],
                data_type: grammar::DataType::Integer
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn no_return_type() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::Fn)),
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenRoundBracket)),
                dummy_token(TokenType::Identifier("b".to_owned())),
                dummy_token(TokenType::Operator(Op::Colon)),
                dummy_token(TokenType::Keyword(Kw::Int)),
                dummy_token(TokenType::Operator(Op::CloseRoundBracket)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Identifier("c".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenRoundBracket)),
                dummy_token(TokenType::Operator(Op::CloseRoundBracket)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_function_def,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            FunctionDefinition {
                identifier: "a".to_owned(),
                params: vec![Parameter {
                    name: "b".to_owned(),
                    data_type: grammar::DataType::Integer
                }],
                statements: vec![
                    FunctionCallExpr::new(IdentifierExpr::new("c".to_owned()).into(), vec![])
                        .into(),
                    Statement::Semicolon
                ],
                data_type: grammar::DataType::None
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn out_of_tokens() {
        let (result, warnings) = partial_parse(
            vec![token(TokenType::Keyword(Kw::Fn), (2, 4), (2, 6))],
            parse_function_def,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::FunctionMissingIdentifier,
                pos: Position::new(2, 6),
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn missing_opening_bracket() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::Fn)),
                dummy_token(TokenType::Identifier("a".to_owned())),
                token(TokenType::Identifier("b".to_owned()), (7, 8), (7, 9)),
                dummy_token(TokenType::Operator(Op::Colon)),
                dummy_token(TokenType::Keyword(Kw::Int)),
                dummy_token(TokenType::Operator(Op::CloseRoundBracket)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Identifier("c".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenRoundBracket)),
                dummy_token(TokenType::Operator(Op::CloseRoundBracket)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_function_def,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            FunctionDefinition {
                identifier: "a".to_owned(),
                params: vec![Parameter {
                    name: "b".to_owned(),
                    data_type: grammar::DataType::Integer
                }],
                statements: vec![
                    FunctionCallExpr::new(IdentifierExpr::new("c".to_owned()).into(), vec![])
                        .into(),
                    Statement::Semicolon
                ],
                data_type: grammar::DataType::None
            }
        );

        assert_eq!(warnings.len(), 1);
        assert_eq!(
            warnings[0],
            ParserWarning {
                warning: ParserWarningVariant::MissingOpeningRoundBracket,
                start: Position::new(7, 8),
                stop: Position::new(7, 9)
            }
        );
    }

    #[test]
    fn missing_closing_bracket() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::Fn)),
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenRoundBracket)),
                dummy_token(TokenType::Identifier("b".to_owned())),
                dummy_token(TokenType::Operator(Op::Colon)),
                dummy_token(TokenType::Keyword(Kw::Int)),
                token(TokenType::Operator(Op::OpenCurlyBracket), (9, 3), (9, 4)),
                dummy_token(TokenType::Identifier("c".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenRoundBracket)),
                dummy_token(TokenType::Operator(Op::CloseRoundBracket)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_function_def,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            FunctionDefinition {
                identifier: "a".to_owned(),
                params: vec![Parameter {
                    name: "b".to_owned(),
                    data_type: grammar::DataType::Integer
                }],
                statements: vec![
                    FunctionCallExpr::new(IdentifierExpr::new("c".to_owned()).into(), vec![])
                        .into(),
                    Statement::Semicolon
                ],
                data_type: grammar::DataType::None
            }
        );

        assert_eq!(warnings.len(), 1);
        assert_eq!(
            warnings[0],
            ParserWarning {
                warning: ParserWarningVariant::MissingClosingRoundBracket,
                start: Position::new(9, 3),
                stop: Position::new(9, 4)
            }
        );
    }

    #[test]
    fn missing_return_type() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::Fn)),
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenRoundBracket)),
                dummy_token(TokenType::Identifier("b".to_owned())),
                dummy_token(TokenType::Operator(Op::Colon)),
                dummy_token(TokenType::Keyword(Kw::Int)),
                dummy_token(TokenType::Operator(Op::CloseRoundBracket)),
                token(TokenType::Operator(Op::Arrow), (3, 5), (3, 7)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Identifier("c".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenRoundBracket)),
                dummy_token(TokenType::Operator(Op::CloseRoundBracket)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_function_def,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::FunctionMissingReturnType,
                pos: Position::new(3, 7)
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn missing_body() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::Fn)),
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenRoundBracket)),
                dummy_token(TokenType::Identifier("b".to_owned())),
                dummy_token(TokenType::Operator(Op::Colon)),
                dummy_token(TokenType::Keyword(Kw::Int)),
                dummy_token(TokenType::Operator(Op::CloseRoundBracket)),
                dummy_token(TokenType::Operator(Op::Arrow)),
                token(TokenType::Keyword(Kw::Int), (6, 5), (6, 8)),
                dummy_token(TokenType::Keyword(Kw::Fn)),
            ],
            parse_function_def,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::FunctionMissingBody,
                pos: Position::new(6, 8)
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn missing_identifier() {
        let (result, warnings) = partial_parse(
            vec![
                token(TokenType::Keyword(Kw::Fn), (4, 5), (4, 7)),
                dummy_token(TokenType::Operator(Op::OpenRoundBracket)),
            ],
            parse_function_def,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::FunctionMissingIdentifier,
                pos: Position::new(4, 7)
            }
        );

        assert!(warnings.is_empty());
    }
}
