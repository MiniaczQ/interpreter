use super::{
    expressions::{parse_expression, Expression},
    utility::*,
    Value,
};

/// A literal value
#[derive(Clone, Debug)]
pub struct Literal(Value);

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
        return Ok(Some(Literal(Value::Integer(v))));
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
