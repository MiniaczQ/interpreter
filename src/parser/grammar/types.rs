use crate::{
    parser::{
        keywords::Keyword, operators::Operator, token::TokenType, ElusiveParserErrorVariant,
        ExtScannable, Parser,
    },
    scannable::Scannable,
};

use super::{DataType, ParseResult};

//type
//    = primitive_type, [OPEN_LIST, CLOSE_LIST]
//    | TYPE_STRING
//    ;
//
//primitive_type
//    = TYPE_INT
//    | TYPE_FLOAT
//    | TYPE_BOOL
//    ;
pub fn parse_type(p: &mut Parser) -> ParseResult<DataType> {
    match p.token()?.token_type {
        TokenType::Keyword(Keyword::Int) => {
            p.pop();
            parse_list_variant(p, DataType::Integer, DataType::IntegerList)
        }
        TokenType::Keyword(Keyword::Float) => {
            p.pop();
            parse_list_variant(p, DataType::Float, DataType::FloatList)
        }
        TokenType::Keyword(Keyword::Bool) => {
            p.pop();
            parse_list_variant(p, DataType::Bool, DataType::BoolList)
        }
        TokenType::Keyword(Keyword::String) => {
            p.pop();
            Ok(Some(DataType::String))
        }
        _ => Ok(None),
    }
}

fn parse_list_variant(p: &mut Parser, non_list: DataType, list: DataType) -> ParseResult<DataType> {
    if let TokenType::Operator(Operator::OpenSquareBracket) = p.token()?.token_type {
        p.pop();
        if let TokenType::Operator(Operator::OpenSquareBracket) = p.token()?.token_type {
            p.pop();
        } else {
            p.error(ElusiveParserErrorVariant::MissingClosingSquareBracket);
        }
        Ok(Some(list))
    } else {
        Ok(Some(non_list))
    }
}
