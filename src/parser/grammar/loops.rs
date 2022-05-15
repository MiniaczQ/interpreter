use crate::{
    parser::{
        keywords::Keyword, token::TokenType, ErrorHandler, ExtScannable, Parser, ParserErrorVariant,
    },
    scannable::Scannable,
};

use super::{
    code_block::{parse_code_block, CodeBlock},
    expressions::{parse_expression, Expression},
    ParseResult,
};

/// While loop
#[derive(Debug)]
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
                Err(p.error(ParserErrorVariant::WhileLoopMissingBody))
            }
        } else {
            Err(p.error(ParserErrorVariant::WhileLoopMissingCondition))
        }
    } else {
        Ok(None)
    }
}

/// For loop
#[derive(Debug)]
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
                    Ok(Some(ForLoop {
                        variable,
                        provider,
                        body,
                    }))
                } else {
                    Err(p.error(ParserErrorVariant::ForLoopMissingBody))
                }
            } else {
                Err(p.error(ParserErrorVariant::ForLoopMissingProvider))
            }
        } else {
            Err(p.error(ParserErrorVariant::ForLoopMissingVariable))
        }
    } else {
        Ok(None)
    }
}