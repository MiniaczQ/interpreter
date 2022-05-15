use crate::{
    parser::{
        keywords::Keyword, token::TokenType, ErrorHandler, ExtScannable, Parser,
        ParserErrorVariant, ParserWarningVariant,
    },
    scannable::Scannable,
};

use super::{
    code_block::{parse_code_block, CodeBlock},
    expressions::{parse_expression, Expression},
    ParseResult,
};

/// While loop
#[derive(Debug, Clone)]
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
                p.error(ParserErrorVariant::WhileLoopMissingBody)
            }
        } else {
            p.error(ParserErrorVariant::WhileLoopMissingCondition)
        }
    } else {
        Ok(None)
    }
}

/// For loop
#[derive(Debug, Clone)]
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
            if let TokenType::Keyword(Keyword::In) = p.token()?.token_type {
                p.pop();
            } else {
                p.warn(ParserWarningVariant::ForLoopMissingInKeyword);
            }
            if let Some(provider) = parse_expression(p)? {
                if let Some(body) = parse_code_block(p)? {
                    Ok(Some(ForLoop {
                        variable,
                        provider,
                        body,
                    }))
                } else {
                    p.error(ParserErrorVariant::ForLoopMissingBody)
                }
            } else {
                p.error(ParserErrorVariant::ForLoopMissingProvider)
            }
        } else {
            p.error(ParserErrorVariant::ForLoopMissingVariable)
        }
    } else {
        Ok(None)
    }
}
