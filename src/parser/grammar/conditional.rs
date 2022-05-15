use super::{
    code_block::{parse_code_block, CodeBlock},
    expressions::{parse_expression, Expression},
    utility::*,
};

/// If expression.
/// The else block is optional.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IfElse {
    pub condition: Expression,
    pub true_case: CodeBlock,
    pub false_case: Option<CodeBlock>,
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
    use crate::parser::grammar::{
        code_block::CodeBlock,
        conditional::{parse_if_else, IfElse},
        expressions::Expression,
    };

    use super::super::test_utils::tests::*;

    #[test]
    fn miss() {
        let (result, warnings) = partial_parse(
            vec![dummy_token(TokenType::Identifier("aaa".to_owned()))],
            parse_if_else,
        );
        assert_eq!(result, Ok(None));

        assert!(warnings.is_empty());
    }

    #[test]
    fn just_if() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::If)),
                dummy_token(TokenType::Keyword(Kw::True)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
                dummy_token(TokenType::Keyword(Kw::If)),
            ],
            parse_if_else,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            IfElse {
                condition: Expression::Literal(Literal(Value::Bool(true))),
                true_case: CodeBlock { statements: vec![] },
                false_case: None,
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn if_else() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::If)),
                dummy_token(TokenType::Keyword(Kw::True)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
                dummy_token(TokenType::Keyword(Kw::Else)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_if_else,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            IfElse {
                condition: Expression::Literal(Literal(Value::Bool(true))),
                true_case: CodeBlock { statements: vec![] },
                false_case: Some(CodeBlock { statements: vec![] }),
            }
        );

        assert!(warnings.is_empty());
    }

    // considers code block as the condition, so results in a different error
    #[test]
    fn missing_condition() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::If)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                token(TokenType::Operator(Op::CloseCurlyBracket), (3, 5), (3, 6)),
                dummy_token(TokenType::Keyword(Kw::Else)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_if_else,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::IfMissingTrueBranch,
                pos: Position::new(3, 6)
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn missing_true_branch() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::If)),
                token(TokenType::Keyword(Kw::True), (7, 4), (7, 8)),
                dummy_token(TokenType::Keyword(Kw::Else)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_if_else,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::IfMissingTrueBranch,
                pos: Position::new(7, 8)
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn missing_false_branch() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::If)),
                dummy_token(TokenType::Keyword(Kw::True)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
                token(TokenType::Keyword(Kw::Else), (9, 6), (9, 10)),
                dummy_token(TokenType::Keyword(Kw::If)),
            ],
            parse_if_else,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::IfMissingFalseBranch,
                pos: Position::new(9, 10)
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn out_of_tokens() {
        let (result, warnings) = partial_parse(
            vec![token(TokenType::Keyword(Kw::If), (1, 2), (1, 4))],
            parse_if_else,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::OutOfTokens,
                pos: Position::new(1, 4)
            }
        );

        assert!(warnings.is_empty());
    }
}
