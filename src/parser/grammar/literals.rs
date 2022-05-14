use crate::scannable::Scannable;

use super::{
    super::{
        keywords::Keyword, operators::Operator, token::TokenType, ElusiveParserErrorVariant,
        ExtScannable, Parser,
    },
    DataType, ExtResult, Node, ParseResult, Value,
};

/// A literal value
#[derive(Clone)]
pub struct Literal(Value);

//impl Computeable for Literal {
//    fn data_type(&self) -> DataType {
//        match self {
//            Literal(Value::Integer(_)) => DataType::Integer,
//            Literal(Value::Float(_)) => DataType::Float,
//            Literal(Value::Bool(_)) => DataType::Bool,
//            Literal(Value::IntegerList(_)) => DataType::IntegerList,
//            Literal(Value::FloatList(_)) => DataType::FloatList,
//            Literal(Value::BoolList(_)) => DataType::BoolList,
//            Literal(Value::String(_)) => DataType::String,
//        }
//    }
//
//    fn compute(&self) -> Literal {
//        self.clone()
//    }
//}

/// CONST_INT
fn parse_integer(p: &mut Parser) -> ParseResult<Literal> {
    let t = p.token()?;
    if let TokenType::Int(v) = t.token_type {
        Ok(Some(Literal(Value::Integer(v))))
    } else {
        Ok(None)
    }
}

/// int_list_constant
///     = OPEN_LIST, [CONST_INT, {SPLIT, CONST_INT}], CLOSE_LIST
///     ;
fn parse_integer_list(p: &mut Parser) -> ParseResult<Literal> {
    let mut list: Vec<i64> = vec![];
    if let TokenType::Operator(Operator::OpenSquareBracket) = p.token()?.token_type {
        p.pop();
        if let TokenType::Int(v) = p.token()?.token_type {
            p.pop();
            list.push(v);
            while let TokenType::Operator(Operator::Split) = p.token()?.token_type {
                p.pop();
                if let TokenType::Int(v) = p.token()?.token_type {
                    p.pop();
                    list.push(v);
                } else {
                    p.error(ElusiveParserErrorVariant::TrailingComma)
                }
            }
        }
        if let TokenType::Operator(Operator::CloseSquareBracket) = p.token()?.token_type {
            p.pop();
        } else {
            p.error(ElusiveParserErrorVariant::MissingClosingSquareBracket)
        }
        Ok(Some(Literal(Value::IntegerList(list))))
    } else {
        Ok(None)
    }
}

/// CONST_FLOAT
fn parse_float(p: &mut Parser) -> ParseResult<Literal> {
    if let TokenType::Float(v) = p.token()?.token_type {
        p.pop();
        Ok(Some(Literal(Value::Float(v))))
    } else {
        Ok(None)
    }
}

/// float_list_constant
///     = OPEN_LIST, [CONST_FLOAT, {SPLIT, CONST_FLOAT}], CLOSE_LIST
///     ;
fn parse_float_list(p: &mut Parser) -> ParseResult<Literal> {
    let mut list: Vec<f64> = vec![];
    if let TokenType::Operator(Operator::OpenSquareBracket) = p.token()?.token_type {
        p.pop();
        if let TokenType::Float(v) = p.token()?.token_type {
            p.pop();
            list.push(v);
            while let TokenType::Operator(Operator::Split) = p.token()?.token_type {
                p.pop();
                if let TokenType::Float(v) = p.token()?.token_type {
                    p.pop();
                    list.push(v);
                } else {
                    p.error(ElusiveParserErrorVariant::TrailingComma)
                }
            }
        }
        if let TokenType::Operator(Operator::CloseSquareBracket) = p.token()?.token_type {
            p.pop();
        } else {
            p.error(ElusiveParserErrorVariant::MissingClosingSquareBracket)
        }
        Ok(Some(Literal(Value::FloatList(list))))
    } else {
        Ok(None)
    }
}

/// CONST_BOOL
fn parse_bool_raw(p: &mut Parser) -> ParseResult<bool> {
    match p.token()?.token_type {
        TokenType::Keyword(Keyword::True) => {
            p.pop();
            Ok(Some(true))
        }
        TokenType::Keyword(Keyword::False) => {
            p.pop();
            Ok(Some(false))
        }
        _ => Ok(None),
    }
}

/// Same as `parse_bool_raw` but returns a `Literal`
fn parse_bool(p: &mut Parser) -> ParseResult<Literal> {
    parse_bool_raw(p).map(|o| o.map(|v| Literal(Value::Bool(v))))
}

/// bool_list_constant
///     = OPEN_LIST, [CONST_BOOL, {SPLIT, CONST_BOOL}], CLOSE_LIST
///     ;
fn parse_bool_list(p: &mut Parser) -> ParseResult<Literal> {
    let mut list: Vec<bool> = vec![];
    if let TokenType::Operator(Operator::OpenSquareBracket) = p.token()?.token_type {
        p.pop();
        if let Some(v) = parse_bool_raw(p)? {
            p.pop();
            list.push(v);
            while let TokenType::Operator(Operator::Split) = p.token()?.token_type {
                p.pop();
                if let Some(v) = parse_bool_raw(p)? {
                    p.pop();
                    list.push(v);
                } else {
                    p.error(ElusiveParserErrorVariant::TrailingComma)
                }
            }
        }
        if let TokenType::Operator(Operator::CloseSquareBracket) = p.token()?.token_type {
            p.pop();
        } else {
            p.error(ElusiveParserErrorVariant::MissingClosingSquareBracket)
        }
        Ok(Some(Literal(Value::BoolList(list))))
    } else {
        Ok(None)
    }
}

fn parse_string(p: &mut Parser) -> ParseResult<Literal> {
    let t = p.token()?;
    if let TokenType::String(v) = t.token_type {
        p.pop();
        Ok(Some(Literal(Value::String(v))))
    } else {
        Ok(None)
    }
}

pub fn parse_literal(p: &mut Parser) -> ParseResult<Literal> {
    parse_integer(p)
        .alt(|| parse_integer_list(p))
        .alt(|| parse_float(p))
        .alt(|| parse_float_list(p))
        .alt(|| parse_bool(p))
        .alt(|| parse_bool_list(p))
        .alt(|| parse_string(p))
}
