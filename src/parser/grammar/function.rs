use super::{
    code_block::{parse_code_block, CodeBlock},
    types::parse_type,
    utility::*,
    DataType,
};

/// A single function parameter
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub data_type: DataType,
}

/// Definition of a function
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FunctionDef {
    pub identifier: String,
    pub params: Vec<Parameter>,
    pub code_block: CodeBlock,
    pub data_type: DataType,
}

/// parameter
///     = IDENTIFIER, TYPE_SIGNATURE, type
///     ;
fn parse_parameter(p: &mut Parser) -> OptRes<Parameter> {
    if let Some(name) = p.identifier()? {
        if !p.operator(Op::Colon)? {
            p.warn(WarnVar::MissingColon);
        }
        if let Some(data_type) = parse_type(p)? {
            Ok(Some(Parameter { name, data_type }))
        } else {
            p.error(ErroVar::FunctionParameterMissingType)
        }
    } else {
        Ok(None)
    }
}

/// parameters
///     = [parameter, {SPLIT, parameter}]
///     ;
fn parse_parameters(p: &mut Parser) -> Res<Vec<Parameter>> {
    let mut params = vec![];
    while let Some(param) = parse_parameter(p)? {
        params.push(param);
    }
    Ok(params)
}

/// function_definition
///     = KW_FN, OPEN_BRACKET, parameters, CLOSE_BRACKET, [RETURN_SIGNATURE, type], code_block
///     ;
pub fn parse_function_def(p: &mut Parser) -> OptRes<FunctionDef> {
    if !p.keyword(Kw::Fn)? {
        return Ok(None);
    }
    if let Some(identifier) = p.identifier()? {
        if !p.operator(Op::OpenRoundBracket)? {
            p.warn(WarnVar::MissingOpeningRoundBracket);
        }
        let params = parse_parameters(p)?;
        if !p.operator(Op::CloseRoundBracket)? {
            p.warn(WarnVar::MissingClosingRoundBracket);
        }
        let data_type = if p.operator(Op::Arrow)? {
            if let Some(data_type) = parse_type(p)? {
                data_type
            } else {
                return p.error(ErroVar::FunctionMissingReturnType);
            }
        } else {
            DataType::None
        };
        if let Some(code_block) = parse_code_block(p)? {
            Ok(Some(FunctionDef {
                identifier,
                params,
                code_block,
                data_type,
            }))
        } else {
            p.error(ErroVar::FunctionMissingBody)
        }
    } else {
        p.error(ErroVar::FunctionMissingIdentifier)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::grammar::{
        code_block::{CodeBlock, Statement},
        function::{parse_function_def, FunctionDef, Parameter},
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
            FunctionDef {
                identifier: "a".to_owned(),
                params: vec![Parameter {
                    name: "b".to_owned(),
                    data_type: grammar::DataType::Integer
                }],
                code_block: CodeBlock {
                    statements: vec![
                        Statement::Expression(Expression::FunctionCall {
                            identifier: Box::new(Expression::Identifier("c".to_owned())),
                            arguments: vec![]
                        }),
                        Statement::Semicolon
                    ]
                },
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
            FunctionDef {
                identifier: "a".to_owned(),
                params: vec![],
                code_block: CodeBlock {
                    statements: vec![
                        Statement::Expression(Expression::FunctionCall {
                            identifier: Box::new(Expression::Identifier("c".to_owned())),
                            arguments: vec![]
                        }),
                        Statement::Semicolon
                    ]
                },
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
            FunctionDef {
                identifier: "a".to_owned(),
                params: vec![Parameter {
                    name: "b".to_owned(),
                    data_type: grammar::DataType::Integer
                }],
                code_block: CodeBlock {
                    statements: vec![
                        Statement::Expression(Expression::FunctionCall {
                            identifier: Box::new(Expression::Identifier("c".to_owned())),
                            arguments: vec![]
                        }),
                        Statement::Semicolon
                    ]
                },
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
                error: ParserErrorVariant::OutOfTokens,
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
            FunctionDef {
                identifier: "a".to_owned(),
                params: vec![Parameter {
                    name: "b".to_owned(),
                    data_type: grammar::DataType::Integer
                }],
                code_block: CodeBlock {
                    statements: vec![
                        Statement::Expression(Expression::FunctionCall {
                            identifier: Box::new(Expression::Identifier("c".to_owned())),
                            arguments: vec![]
                        }),
                        Statement::Semicolon
                    ]
                },
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
            FunctionDef {
                identifier: "a".to_owned(),
                params: vec![Parameter {
                    name: "b".to_owned(),
                    data_type: grammar::DataType::Integer
                }],
                code_block: CodeBlock {
                    statements: vec![
                        Statement::Expression(Expression::FunctionCall {
                            identifier: Box::new(Expression::Identifier("c".to_owned())),
                            arguments: vec![]
                        }),
                        Statement::Semicolon
                    ]
                },
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
