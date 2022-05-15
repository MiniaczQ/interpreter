use super::{
    code_block::{parse_code_block, CodeBlock},
    expressions::{parse_expression, Expression},
    utility::*,
};

/// If expression.
/// The else block is optional.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IfElse {
    condition: Expression,
    true_case: CodeBlock,
    false_case: Option<CodeBlock>,
}

/// if_expression
///     = KW_IF, expression, code_block, [KW_ELSE, code_block]
///     ;
pub fn parse_if_else(p: &mut Parser) -> OptRes<IfElse> {
    if !p.keyword(Kw::If)? {
        return Ok(None);
    }
    if let Some(condition) = parse_expression(p)? {
        if let Some(true_case) = parse_code_block(p)? {
            let false_case = if p.keyword(Kw::Else)? {
                if let Some(false_case) = parse_code_block(p)? {
                    Some(false_case)
                } else {
                    return p.error(ErroVar::IfMissingFalseBranch);
                }
            } else {
                None
            };
            Ok(Some(IfElse {
                condition,
                true_case,
                false_case,
            }))
        } else {
            p.error(ErroVar::IfMissingTrueBranch)
        }
    } else {
        p.error(ErroVar::IfMissingCondition)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
