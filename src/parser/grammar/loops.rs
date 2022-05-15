use super::{
    code_block::{parse_code_block, CodeBlock},
    expressions::{parse_expression, Expression},
    utility::*,
};

/// While loop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhileLoop {
    condition: Expression,
    body: CodeBlock,
}

/// For loop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForLoop {
    variable: String,
    provider: Expression,
    body: CodeBlock,
}

/// while_expression
///     = KW_WHILE, expression, code_block
///     ;
pub fn parse_while_loop(p: &mut Parser) -> OptRes<WhileLoop> {
    if !p.keyword(Kw::While)? {
        return Ok(None);
    }
    if let Some(condition) = parse_expression(p)? {
        if let Some(body) = parse_code_block(p)? {
            Ok(Some(WhileLoop { condition, body }))
        } else {
            p.error(ErroVar::WhileLoopMissingBody)
        }
    } else {
        p.error(ErroVar::WhileLoopMissingCondition)
    }
}

/// for_expression
///     = KW_FOR, IDENTIFIER, KW_IN, expression, code_block
///     ;
pub fn parse_for_loop(p: &mut Parser) -> OptRes<ForLoop> {
    if !p.keyword(Kw::For)? {
        return Ok(None);
    }
    if let Some(variable) = p.identifier()? {
        if !p.keyword(Kw::In)? {
            p.warn(WarnVar::ForLoopMissingInKeyword);
        }
        if let Some(provider) = parse_expression(p)? {
            if let Some(body) = parse_code_block(p)? {
                Ok(Some(ForLoop {
                    variable,
                    provider,
                    body,
                }))
            } else {
                p.error(ErroVar::ForLoopMissingBody)
            }
        } else {
            p.error(ErroVar::ForLoopMissingProvider)
        }
    } else {
        p.error(ErroVar::ForLoopMissingVariable)
    }
}
