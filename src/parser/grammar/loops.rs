use super::{
    code_block::{parse_code_block, CodeBlock},
    expressions::{parse_expression, Expression},
    utility::*,
};

/// While loop
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WhileLoop {
    pub condition: Expression,
    pub body: CodeBlock,
}

/// For loop
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ForLoop {
    pub variable: String,
    pub provider: Expression,
    pub body: CodeBlock,
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

#[cfg(test)]
mod tests {
    use crate::parser::grammar::{
        code_block::CodeBlock,
        loops::{parse_for_loop, parse_while_loop, ForLoop, WhileLoop},
    };

    use super::super::test_utils::tests::*;

    #[test]
    fn miss_for_loop() {
        let (result, warnings) = partial_parse(
            vec![dummy_token(TokenType::Keyword(Kw::Let))],
            parse_for_loop,
        );
        assert_eq!(result, Ok(None));

        assert!(warnings.is_empty());
    }

    #[test]
    fn for_loop() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::For)),
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Keyword(Kw::In)),
                dummy_token(TokenType::Identifier("b".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_for_loop,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            ForLoop {
                variable: "a".to_owned(),
                provider: Expression::Identifier("b".to_owned()),
                body: CodeBlock { statements: vec![] }
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn for_loop_missing_in_keyword() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::For)),
                dummy_token(TokenType::Identifier("a".to_owned())),
                token(TokenType::Identifier("b".to_owned()), (7, 6), (7, 7)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_for_loop,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            ForLoop {
                variable: "a".to_owned(),
                provider: Expression::Identifier("b".to_owned()),
                body: CodeBlock { statements: vec![] }
            }
        );

        assert_eq!(warnings.len(), 1);
        assert_eq!(
            warnings[0],
            ParserWarning {
                warning: ParserWarningVariant::ForLoopMissingInKeyword,
                start: Position::new(7, 6),
                stop: Position::new(7, 7)
            }
        );
    }

    #[test]
    fn for_loop_missing_body() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::For)),
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Keyword(Kw::In)),
                token(TokenType::Identifier("b".to_owned()), (7, 2), (7, 3)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_for_loop,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::ForLoopMissingBody,
                pos: Position::new(7, 3),
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn for_loop_missing_provider() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::For)),
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Keyword(Kw::In)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                token(TokenType::Operator(Op::CloseCurlyBracket), (9, 2), (9, 3)),
                dummy_token(TokenType::Keyword(Kw::Let)),
            ],
            parse_for_loop,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::ForLoopMissingBody,
                pos: Position::new(9, 3),
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn for_loop_missing_variable() {
        let (result, warnings) = partial_parse(
            vec![
                token(TokenType::Keyword(Kw::For), (5, 6), (5, 9)),
                dummy_token(TokenType::Keyword(Kw::In)),
                dummy_token(TokenType::Identifier("b".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_for_loop,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::ForLoopMissingVariable,
                pos: Position::new(5, 9),
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn for_loop_out_of_tokens() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::For)),
                dummy_token(TokenType::Identifier("a".to_owned())),
                token(TokenType::Keyword(Kw::In), (2, 3), (2, 5)),
            ],
            parse_for_loop,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::OutOfTokens,
                pos: Position::new(2, 5),
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn miss_while_loop() {
        let (result, warnings) = partial_parse(
            vec![dummy_token(TokenType::Keyword(Kw::Let))],
            parse_while_loop,
        );
        assert_eq!(result, Ok(None));

        assert!(warnings.is_empty());
    }

    #[test]
    fn while_loop() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::While)),
                dummy_token(TokenType::Keyword(Kw::True)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_while_loop,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            WhileLoop {
                condition: Expression::Literal(Literal(Value::Bool(true))),
                body: CodeBlock { statements: vec![] }
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn while_loop_missing_body() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::While)),
                token(TokenType::Keyword(Kw::True), (6, 3), (6, 7)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_while_loop,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::WhileLoopMissingBody,
                pos: Position::new(6, 7),
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn while_loop_missing_condition() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::While)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                token(TokenType::Operator(Op::CloseCurlyBracket), (9, 8), (9, 9)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_while_loop,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::WhileLoopMissingBody,
                pos: Position::new(9, 9),
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn while_loop_out_of_tokens() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::While)),
                token(TokenType::Keyword(Kw::True), (2, 3), (2, 7)),
            ],
            parse_while_loop,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::OutOfTokens,
                pos: Position::new(2, 7),
            }
        );

        assert!(warnings.is_empty());
    }
}
