use crate::{parser::ErrorHandler, scannable::Scannable};

use super::{
    super::{
        keywords::Keyword, operators::Operator, token::TokenType, ExtScannable, Parser,
        ParserWarningVariant,
    },
    expressions::{parse_expression, Expression},
    ExtResult, ParseResult, Value,
};

/// A literal value
#[derive(Clone, Debug)]
pub struct Literal(Value);

/// list_constant
///     = OPEN_LIST, [expression, {SPLIT, expression}], CLOSE_LIST
///     ;
fn parse_list(p: &mut Parser) -> ParseResult<Literal> {
    let mut list: Vec<Expression> = vec![];
    if let TokenType::Operator(Operator::OpenSquareBracket) = p.token()?.token_type {
        p.pop();
        if let Some(expression) = parse_expression(p)? {
            list.push(expression);
            while let TokenType::Operator(Operator::Split) = p.token()?.token_type {
                p.pop();
                if let Some(expression) = parse_expression(p)? {
                    list.push(expression);
                } else {
                    p.warn(ParserWarningVariant::TrailingComma)
                }
            }
        }
        if let TokenType::Operator(Operator::CloseSquareBracket) = p.token()?.token_type {
            p.pop();
        } else {
            p.warn(ParserWarningVariant::MissingClosingSquareBracket)
        }
        Ok(Some(Literal(Value::List(list))))
    } else {
        Ok(None)
    }
}

/// CONST_INT
fn parse_integer(p: &mut Parser) -> ParseResult<Literal> {
    let t = p.token()?;
    if let TokenType::Int(v) = t.token_type {
        p.pop();
        Ok(Some(Literal(Value::Integer(v))))
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

/// Same as `parse_bool_raw` but returns a `Literal`
fn parse_bool(p: &mut Parser) -> ParseResult<Literal> {
    match p.token()?.token_type {
        TokenType::Keyword(Keyword::True) => {
            p.pop();
            Ok(Some(Literal(Value::Bool(true))))
        }
        TokenType::Keyword(Keyword::False) => {
            p.pop();
            Ok(Some(Literal(Value::Bool(false))))
        }
        _ => Ok(None),
    }
}

/// CONST_STRING
fn parse_string(p: &mut Parser) -> ParseResult<Literal> {
    let t = p.token()?;
    if let TokenType::String(v) = t.token_type {
        p.pop();
        Ok(Some(Literal(Value::String(v))))
    } else {
        Ok(None)
    }
}

/// constant
///     = list_constant
///     | CONST_INT
///     | CONST_FLOAT
///     | CONST_BOOL
///     | CONST_STRING
///     ;
pub fn parse_literal(p: &mut Parser) -> ParseResult<Literal> {
    parse_list(p)
        .alt(|| parse_integer(p))
        .alt(|| parse_float(p))
        .alt(|| parse_bool(p))
        .alt(|| parse_string(p))
}
