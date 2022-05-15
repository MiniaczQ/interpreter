use super::{utility::*, DataType};

//type
//    = primitive_type, [OPEN_LIST, CLOSE_LIST]
//    | TYPE_STRING
//    ;
//
//primitive_type
//    = TYPE_INT
//    | TYPE_FLOAT
//    | TYPE_BOOL
//    ;
pub fn parse_type(p: &mut Parser) -> OptRes<DataType> {
    if p.keyword(Kw::Int)? {
        return parse_list_variant(p, DataType::Integer, DataType::IntegerList);
    }
    if p.keyword(Kw::Float)? {
        return parse_list_variant(p, DataType::Float, DataType::FloatList);
    }
    if p.keyword(Kw::Bool)? {
        return parse_list_variant(p, DataType::Bool, DataType::BoolList);
    }
    if p.keyword(Kw::String)? {
        return Ok(Some(DataType::String));
    }
    Ok(None)
}

fn parse_list_variant(p: &mut Parser, non_list: DataType, list: DataType) -> OptRes<DataType> {
    if !p.operator(Op::OpenSquareBracket)? {
        return Ok(Some(non_list));
    }
    if !p.operator(Op::CloseSquareBracket)? {
        p.warn(WarnVar::MissingClosingSquareBracket);
    }
    Ok(Some(list))
}

#[cfg(test)]
mod tests {
    use super::super::test_utils::tests::*;

    #[test]
    fn miss() {}

    #[test]
    fn int() {}

    #[test]
    fn int_list() {}

    #[test]
    fn float_list() {}

    #[test]
    fn float() {}

    #[test]
    fn bool_list() {}

    #[test]
    fn bool() {}

    #[test]
    fn list_missing_bracket() {}

    #[test]
    fn out_of_tokens() {}
}
