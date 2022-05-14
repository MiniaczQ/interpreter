use crate::{
    parser::{keywords::Keyword, token::TokenType, ExtScannable, Parser, ParserErrorVariant, ErrorHandler},
    scannable::Scannable,
};

use super::{
    code_block::{parse_code_block, CodeBlock},
    expressions::{parse_expression, Expression},
    ParseResult,
};

/// If expression.
/// The else block is optional.
pub struct IfExpression {
    condition: Expression,
    true_case: CodeBlock,
    false_case: Option<CodeBlock>,
}

/// if_expression
///     = KW_IF, expression, code_block, [KW_ELSE, code_block]
///     ;
pub fn parse_if_expression(p: &mut Parser) -> ParseResult<IfExpression> {
    if let TokenType::Keyword(Keyword::If) = p.token()?.token_type {
        p.pop();
        if let Some(condition) = parse_expression(p)? {
            if let Some(true_case) = parse_code_block(p)? {
                let false_case = if let TokenType::Keyword(Keyword::Else) = p.token()?.token_type {
                    p.pop();
                    if let Some(false_case) = parse_code_block(p)? {
                        Some(false_case)
                    } else {
                        return Err(p.error(ParserErrorVariant::MissingIfFalseBranch));
                    }
                } else {
                    None
                };
                Ok(Some(IfExpression {
                    condition,
                    true_case,
                    false_case,
                }))
            } else {
                Err(p.error(ParserErrorVariant::MissingIfTrueBranch))
            }
        } else {
            Err(p.error(ParserErrorVariant::MissingIfCondition))
        }
    } else {
        Ok(None)
    }
}
