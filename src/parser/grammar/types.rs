use super::{utility::*, DataType};

//primitive_type
//    = TYPE_INT
//    | TYPE_FLOAT
//    | TYPE_BOOL
//    | OPEN_LIST, CLOSE_LIST
//    ;
pub fn parse_type(p: &mut Parser) -> OptRes<DataType> {
    if p.keyword(Kw::Int)? {
        return Ok(Some(DataType::Integer));
    }
    if p.keyword(Kw::Float)? {
        return Ok(Some(DataType::Float));
    }
    if p.keyword(Kw::Bool)? {
        return Ok(Some(DataType::Bool));
    }
    if p.keyword(Kw::String)? {
        return Ok(Some(DataType::String));
    }
    parse_list_type(p)
}

fn parse_list_type(p: &mut Parser) -> OptRes<DataType> {
    if !p.operator(Op::OpenSquareBracket)? {
        return Ok(None);
    }
    if !p.operator(Op::CloseSquareBracket)? {
        p.warn(WarnVar::MissingClosingSquareBracket)?;
    }
    Ok(Some(DataType::List))
}

#[cfg(test)]
mod tests {
    use crate::parser::grammar::{types::parse_type, DataType};

    use super::super::test_utils::tests::*;

    #[test]
    fn miss() {
        let (result, warnings) =
            partial_parse(vec![dummy_token(TokenType::Keyword(Kw::Let))], parse_type);
        assert_eq!(result, Ok(None));

        assert!(warnings.is_empty());
    }

    #[test]
    fn int() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::Int)),
                dummy_token(TokenType::Keyword(Kw::Let)),
            ],
            parse_type,
        );
        assert_eq!(result.unwrap().unwrap(), DataType::Integer);

        assert!(warnings.is_empty());
    }

    #[test]
    fn float() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::Float)),
                dummy_token(TokenType::Keyword(Kw::Let)),
            ],
            parse_type,
        );
        assert_eq!(result.unwrap().unwrap(), DataType::Float);

        assert!(warnings.is_empty());
    }

    #[test]
    fn bool() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::Bool)),
                dummy_token(TokenType::Keyword(Kw::Let)),
            ],
            parse_type,
        );
        assert_eq!(result.unwrap().unwrap(), DataType::Bool);

        assert!(warnings.is_empty());
    }

    #[test]
    fn bool_list() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Operator(Op::OpenSquareBracket)),
                dummy_token(TokenType::Operator(Op::CloseSquareBracket)),
                dummy_token(TokenType::Keyword(Kw::Let)),
            ],
            parse_type,
        );
        assert_eq!(result.unwrap().unwrap(), DataType::List);

        assert!(warnings.is_empty());
    }

    #[test]
    fn list_missing_bracket() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Operator(Op::OpenSquareBracket)),
                token(TokenType::Keyword(Kw::Let), (7, 8), (7, 11)),
            ],
            parse_type,
        );
        assert_eq!(result.unwrap().unwrap(), DataType::List);

        assert_eq!(warnings.len(), 1);
        assert_eq!(
            warnings[0],
            ParserWarning {
                warning: ParserWarningVariant::MissingClosingSquareBracket,
                start: Position::new(7, 8),
                stop: Position::new(7, 11)
            }
        );
    }

    #[test]
    fn string() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::String)),
                dummy_token(TokenType::Keyword(Kw::Let)),
            ],
            parse_type,
        );
        assert_eq!(result.unwrap().unwrap(), DataType::String);

        assert!(warnings.is_empty());
    }

    #[test]
    fn string_no_list() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::String)),
                dummy_token(TokenType::Operator(Op::OpenSquareBracket)),
                dummy_token(TokenType::Operator(Op::CloseSquareBracket)),
                dummy_token(TokenType::Keyword(Kw::Let)),
            ],
            parse_type,
        );
        assert_eq!(result.unwrap().unwrap(), DataType::String);

        assert!(warnings.is_empty());
    }

    #[test]
    fn out_of_tokens() {
        let (result, warnings) = partial_parse(
            vec![token(TokenType::Keyword(Kw::Int), (2, 4), (2, 6))],
            parse_type,
        );
        assert_eq!(result.unwrap().unwrap(), DataType::Integer);

        assert!(warnings.is_empty());
    }
}
