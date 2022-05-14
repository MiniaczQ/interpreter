use crate::{
    parser::{keywords::Keyword, token::TokenType, ExtScannable, Parser, ParserErrorVariant, ErrorHandler},
    scannable::Scannable,
};

use super::{
    code_block::{parse_code_block, CodeBlock},
    expressions::{parse_expression, Expression},
    ParseResult,
};

pub struct WhileLoop {
    condition: Expression,
    body: CodeBlock,
}

/// while_expression
///     = KW_WHILE, expression, code_block
///     ;
pub fn parse_while_loop(p: &mut Parser) -> ParseResult<WhileLoop> {
    if let TokenType::Keyword(Keyword::While) = p.token()?.token_type {
        p.pop();
        if let Some(condition) = parse_expression(p)? {
            if let Some(body) = parse_code_block(p)? {
                Ok(Some(WhileLoop { condition, body }))
            } else {
                Err(p.error(ParserErrorVariant::MissingWhileLoopBody))
            }
        } else {
            Err(p.error(ParserErrorVariant::MissingWhileLoopCondition))
        }
    } else {
        Ok(None)
    }
}

pub struct ForLoop {
    variable: String,
    provider: Expression,
    body: CodeBlock,
}

/// for_expression
///     = KW_FOR, IDENTIFIER, KW_IN, expression, code_block
///     ;
pub fn parse_for_loop(p: &mut Parser) -> ParseResult<ForLoop> {
    if let TokenType::Keyword(Keyword::For) = p.token()?.token_type {
        p.pop();
        if let TokenType::Identifier(variable) = p.token()?.token_type {
            p.pop();
            if let Some(provider) = parse_expression(p)? {
                if let Some(body) = parse_code_block(p)? {
                    Ok(Some(ForLoop { variable, provider, body }))
                } else {
                    Err(p.error(ParserErrorVariant::MissingForLoopBody))
                }
            } else {
                Err(p.error(ParserErrorVariant::MissingForLoopProvider))
            }
        } else {
            Err(p.error(ParserErrorVariant::MissingForLoopVariable))
        }
    } else {
        Ok(None)
    }
}
