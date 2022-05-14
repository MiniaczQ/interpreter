use crate::{
    parser::{
        keywords::Keyword, operators::Operator, token::TokenType, ElusiveParserErrorVariant,
        ExtScannable, Parser,
    },
    scannable::Scannable,
};

use super::{DataType, ParseResult};

struct Parameter {
    name: String,
    data_type: DataType,
}

/// parameter
///     = IDENTIFIER, TYPE_SIGNATURE, type
///     ;
fn parse_parameter(p: &mut Parser) -> ParseResult<Parameter> {
    if let TokenType::Identifier(id) = p.token()?.token_type {
        p.pop();
        
    } else {
        Ok(None)
    }
}
