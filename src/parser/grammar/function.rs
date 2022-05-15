use crate::{
    parser::{
        keywords::Keyword, operators::Operator, token::TokenType, ErrorHandler, ExtScannable,
        Parser, ParserError, ParserErrorVariant, ParserWarningVariant,
    },
    scannable::Scannable,
};

use super::{
    code_block::{parse_code_block, CodeBlock},
    types::{parse_type, DataType},
    ParseResult,
};

/// A single function parameter
struct Parameter {
    name: String,
    data_type: DataType,
}

/// parameter
///     = IDENTIFIER, TYPE_SIGNATURE, type
///     ;
fn parse_parameter(p: &mut Parser) -> ParseResult<Parameter> {
    if let TokenType::Identifier(name) = p.token()?.token_type {
        p.pop();
        if let TokenType::Operator(Operator::Colon) = p.token()?.token_type {
            p.pop();
        } else {
            p.warn(ParserWarningVariant::MissingColon);
        }
        if let Some(data_type) = parse_type(p)? {
            Ok(Some(Parameter { name, data_type }))
        } else {
            Err(p.error(ParserErrorVariant::FunctionParameterMissingType))
        }
    } else {
        Ok(None)
    }
}

/// parameters
///     = [parameter, {SPLIT, parameter}]
///     ;
fn parse_parameters(p: &mut Parser) -> Result<Vec<Parameter>, ParserError> {
    let mut params = vec![];
    while let Some(param) = parse_parameter(p)? {
        params.push(param);
    }
    Ok(params)
}

/// Definition of a function
pub struct FunctionDef {
    params: Vec<Parameter>,
    code_block: CodeBlock,
    result: DataType,
}

/// function_definition
///     = KW_FN, OPEN_BRACKET, parameters, CLOSE_BRACKET, [RETURN_SIGNATURE, type], code_block
///     ;
pub fn parse_function_def(p: &mut Parser) -> ParseResult<FunctionDef> {
    if let TokenType::Keyword(Keyword::Fn) = p.token()?.token_type {
        p.pop();
        if let TokenType::Identifier(name) = p.token()?.token_type {
            p.pop();
            if let TokenType::Operator(Operator::OpenRoundBracket) = p.token()?.token_type {
                p.pop();
            } else {
                p.warn(ParserWarningVariant::MissingOpeningRoundBracket);
            }
            let params = parse_parameters(p)?;
            if let TokenType::Operator(Operator::CloseRoundBracket) = p.token()?.token_type {
                p.pop();
            } else {
                p.warn(ParserWarningVariant::MissingClosingRoundBracket);
            }
            let result = if let TokenType::Operator(Operator::Arrow) = p.token()?.token_type {
                p.pop();
                if let Some(data_type) = parse_type(p)? {
                    data_type
                } else {
                    return Err(p.error(ParserErrorVariant::FunctionMissingReturnType));
                }
            } else {
                DataType::None
            };
            if let Some(code_block) = parse_code_block(p)? {
                Ok(Some(FunctionDef {
                    params,
                    code_block,
                    result,
                }))
            } else {
                Err(p.error(ParserErrorVariant::FunctionMissingBody))
            }
        } else {
            Err(p.error(ParserErrorVariant::FunctionMissingIdentifier))
        }
    } else {
        Ok(None)
    }
}
