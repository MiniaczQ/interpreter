use crate::{
    parser::{
        operators::Operator, token::TokenType, ErrorHandler, ExtScannable, Parser, ParserError,
        ParserErrorVariant, ParserWarningVariant,
    },
    scannable::Scannable,
};

use super::{
    literals::{parse_literal, Literal},
    ExtResult, ParseResult,
};

pub enum Expression {
    Bracketed(Box<Expression>),
    Literal(Literal),
    Identifier(String),
}

/// grouped
///     = OPEN_BRACKET, expression, CLOSE_BRACKET
///     ;
fn parse_bracket_expression(p: &mut Parser) -> ParseResult<Expression> {
    if let TokenType::Operator(Operator::OpenRoundBracket) = p.token()?.token_type {
        p.pop();
        if let Some(expression) = parse_expression(p)? {
            if let TokenType::Operator(Operator::OpenRoundBracket) = p.token()?.token_type {
                p.pop();
            } else {
                p.warn(ParserWarningVariant::MissingClosingRoundBracket);
            }
            Ok(Some(expression))
        } else {
            Err(p.error(ParserErrorVariant::InvalidBracketExpression))
        }
    } else {
        Ok(None)
    }
}

fn parse_identifier(p: &mut Parser) -> ParseResult<Expression> {
    if let TokenType::Identifier(identifier) = p.token()?.token_type {
        Ok(Some(Expression::Identifier(identifier)))
    } else {
        Ok(None)
    }
}

/// const_or_identifier_expression
///     = constant | IDENTIFIER | grouped
///     ;
fn parse_constant_or_identifier_or_bracket_expression(p: &mut Parser) -> ParseResult<Expression> {
    parse_literal(p)
        .alt(|| parse_bracket_expression(p))
        .alt(|| parse_identifier(p))
}

/// Two ways of accessing list elements
pub enum IndexOrRange {
    Index(Expression),
    Range(Expression, Expression),
}

/// index_or_range_access
///     = expression, [RANGE, expression]
///     ;
fn parse_index_or_range_access(p: &mut Parser) -> ParseResult<IndexOrRange> {
    if let Some(left_index) = parse_expression(p)? {
        if let TokenType::Operator(Operator::DoubleColon) = p.token()?.token_type {
            p.pop();
            if let Some(right_index) = parse_expression(p)? {
                Ok(Some(IndexOrRange::Range(left_index, right_index)))
            } else {
                Err(p.error(ParserErrorVariant::IncompleteRange))
            }
        } else {
            Ok(Some(IndexOrRange::Index(left_index)))
        }
    } else {
        Ok(None)
    }
}

/// list_access
///     = OPEN_LIST, index_or_range_access, CLOSE_LIST
///     ;
fn parse_list_access(p: &mut Parser) -> ParseResult<IndexOrRange> {
    if let TokenType::Operator(Operator::OpenSquareBracket) = p.token()?.token_type {
        if let Some(index_or_range) = parse_index_or_range_access(p)? {
            if let TokenType::Operator(Operator::CloseSquareBracket) = p.token()?.token_type {
                p.pop();
            } else {
                p.warn(ParserWarningVariant::MissingClosingSquareBracket);
            }
            Ok(Some(index_or_range))
        } else {
            Err(p.error(ParserErrorVariant::EmptyListAccess))
        }
    } else {
        Ok(None)
    }
}

/// function_arguments
///     = [expression, {SPLIT, expression}]
///     ;
fn parse_function_arguments(p: &mut Parser) -> Result<Vec<Expression>, ParserError> {
    let mut arguments = vec![];
    while let Some(argument) = parse_expression(p)? {
        arguments.push(argument);
    }
    Ok(arguments)
}

/// function_call
///     = OPEN_BRACKET, function_arguments, CLOSE_BRACKET
///     ;
fn parse_function_call(p: &mut Parser) -> ParseResult<Vec<Expression>> {
    if let TokenType::Operator(Operator::OpenRoundBracket) = p.token()?.token_type {
        let args = parse_function_arguments(p)?;
        if let TokenType::Operator(Operator::CloseRoundBracket) = p.token()?.token_type {
            p.pop();
        } else {
            p.warn(ParserWarningVariant::MissingClosingRoundBracket);
        }
        Ok(Some(args))
    } else {
        Ok(None)
    }
}

pub fn parse_expression(p: &mut Parser) -> ParseResult<Expression> {
    unimplemented!()
}
