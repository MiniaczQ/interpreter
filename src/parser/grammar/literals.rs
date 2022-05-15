use super::{
    expressions::{parse_expression, Expression},
    utility::*,
    Value,
};

/// A literal value
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Literal(pub Value);

/// list_constant
///     = OPEN_LIST, [expression, {SPLIT, expression}], CLOSE_LIST
///     ;
fn parse_list(p: &mut Parser) -> OptRes<Literal> {
    let mut list: Vec<Expression> = vec![];
    if !p.operator(Op::OpenSquareBracket)? {
        return Ok(None);
    }
    if let Some(expression) = parse_expression(p)? {
        list.push(expression);
        while p.operator(Op::Split)? {
            if let Some(expression) = parse_expression(p)? {
                list.push(expression);
            } else {
                p.warn(WarnVar::TrailingComma)
            }
        }
    }
    if !p.operator(Op::CloseSquareBracket)? {
        p.warn(WarnVar::MissingClosingSquareBracket);
    }
    Ok(Some(Literal(Value::List(list))))
}

/// CONST_INT
fn parse_integer(p: &mut Parser) -> OptRes<Literal> {
    if let Some(v) = p.integer()? {
        return Ok(Some(Literal(Value::Int(v))));
    }
    Ok(None)
}

/// CONST_FLOAT
fn parse_float(p: &mut Parser) -> OptRes<Literal> {
    if let Some(v) = p.float()? {
        return Ok(Some(Literal(Value::Float(v))));
    }
    Ok(None)
}

/// Same as `parse_bool_raw` but returns a `Literal`
fn parse_bool(p: &mut Parser) -> OptRes<Literal> {
    if p.keyword(Kw::True)? {
        return Ok(Some(Literal(Value::Bool(true))));
    }
    if p.keyword(Kw::False)? {
        return Ok(Some(Literal(Value::Bool(false))));
    }
    Ok(None)
}

/// CONST_STRING
fn parse_string(p: &mut Parser) -> OptRes<Literal> {
    if let Some(v) = p.string()? {
        return Ok(Some(Literal(Value::String(v))));
    }
    Ok(None)
}

/// constant
///     = list_constant
///     | CONST_INT
///     | CONST_FLOAT
///     | CONST_BOOL
///     | CONST_STRING
///     ;
pub fn parse_literal(p: &mut Parser) -> OptRes<Literal> {
    parse_list(p)
        .alt(|| parse_integer(p))
        .alt(|| parse_float(p))
        .alt(|| parse_bool(p))
        .alt(|| parse_string(p))
}

#[cfg(test)]
mod tests {
    use crate::parser::grammar::literals::parse_literal;

    use super::super::test_utils::tests::*;

    #[test]
    fn miss() {
        let (result, warnings) = partial_parse(
            vec![dummy_token(TokenType::Keyword(Kw::Let))],
            parse_literal,
        );
        assert_eq!(result, Ok(None));

        assert!(warnings.is_empty());
    }

    #[test]
    fn int() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Int(5)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_literal,
        );
        assert_eq!(result.unwrap().unwrap(), Literal(Value::Int(5)));

        assert!(warnings.is_empty());
    }

    #[test]
    fn float() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Float(5.0)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_literal,
        );
        assert_eq!(result.unwrap().unwrap(), Literal(Value::Float(5.0)));

        assert!(warnings.is_empty());
    }

    #[test]
    fn string() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::String("ada".to_owned())),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_literal,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            Literal(Value::String("ada".to_owned()))
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn bool_true() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::True)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_literal,
        );
        assert_eq!(result.unwrap().unwrap(), Literal(Value::Bool(true)));

        assert!(warnings.is_empty());
    }

    #[test]
    fn bool_flase() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::False)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_literal,
        );
        assert_eq!(result.unwrap().unwrap(), Literal(Value::Bool(false)));

        assert!(warnings.is_empty());
    }

    #[test]
    fn list() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Operator(Op::OpenSquareBracket)),
                dummy_token(TokenType::Int(5)),
                dummy_token(TokenType::Operator(Op::Split)),
                dummy_token(TokenType::Int(6)),
                dummy_token(TokenType::Operator(Op::CloseSquareBracket)),
            ],
            parse_literal,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            Literal(Value::List(vec![
                Expression::Literal(Literal(Value::Int(5))),
                Expression::Literal(Literal(Value::Int(6)))
            ]))
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn list_empty() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Operator(Op::OpenSquareBracket)),
                dummy_token(TokenType::Operator(Op::CloseSquareBracket)),
                token(TokenType::Int(5), (2, 3), (2, 4)),
            ],
            parse_literal,
        );
        assert_eq!(result.unwrap().unwrap(), Literal(Value::List(vec![])));

        assert!(warnings.is_empty());
    }

    #[test]
    fn list_trailing_comma() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Operator(Op::OpenSquareBracket)),
                dummy_token(TokenType::Int(5)),
                dummy_token(TokenType::Operator(Op::Split)),
                dummy_token(TokenType::Int(6)),
                dummy_token(TokenType::Operator(Op::Split)),
                token(TokenType::Operator(Op::CloseSquareBracket), (5, 6), (5, 7)),
            ],
            parse_literal,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            Literal(Value::List(vec![
                Expression::Literal(Literal(Value::Int(5))),
                Expression::Literal(Literal(Value::Int(6)))
            ]))
        );

        assert_eq!(warnings.len(), 1);
        assert_eq!(
            warnings[0],
            ParserWarning {
                warning: ParserWarningVariant::TrailingComma,
                start: Position::new(5, 6),
                stop: Position::new(5, 7)
            }
        );
    }

    #[test]
    fn list_missing_bracket() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Operator(Op::OpenSquareBracket)),
                dummy_token(TokenType::Int(5)),
                dummy_token(TokenType::Operator(Op::Split)),
                dummy_token(TokenType::Int(6)),
                token(TokenType::Keyword(Kw::Let), (7, 3), (7, 6)),
            ],
            parse_literal,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            Literal(Value::List(vec![
                Expression::Literal(Literal(Value::Int(5))),
                Expression::Literal(Literal(Value::Int(6)))
            ]))
        );

        assert_eq!(warnings.len(), 1);
        assert_eq!(
            warnings[0],
            ParserWarning {
                warning: ParserWarningVariant::MissingClosingSquareBracket,
                start: Position::new(7, 3),
                stop: Position::new(7, 6)
            }
        );
    }

    #[test]
    fn out_of_tokens() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Operator(Op::OpenSquareBracket)),
                token(TokenType::Int(5), (2, 3), (2, 4)),
            ],
            parse_literal,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::OutOfTokens,
                pos: Position::new(2, 4),
            }
        );

        assert!(warnings.is_empty());
    }
}
