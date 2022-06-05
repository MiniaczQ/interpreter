use super::{
    expressions::statement::{parse_code_block, Statement},
    types::parse_type,
    utility::*,
    DataType,
};

/// A single function parameter
#[derive(Debug, Serialize, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub data_type: DataType,
}

/// Definition of a function
#[derive(Debug, Serialize, PartialEq)]
pub struct FunctionDefinition {
    pub identifier: String,
    pub params: Vec<Parameter>,
    pub statements: Vec<Statement>,
    pub data_type: DataType,
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
