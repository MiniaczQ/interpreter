use super::{utility::*, Value};

/// A literal value
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Literal(pub Value);

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

/// KW_TRUE | KW_FALSE
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
///     = CONST_INT
///     | CONST_FLOAT
///     | KW_TRUE | KW_FALSE
///     | CONST_STRING
///     ;
pub fn parse_literal(p: &mut Parser) -> OptRes<Literal> {
    parse_integer(p)
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
    fn out_of_tokens() {
        let (result, warnings) = partial_parse(vec![], parse_literal);
        assert_eq!(result.unwrap(), None);

        assert!(warnings.is_empty());
    }
}
