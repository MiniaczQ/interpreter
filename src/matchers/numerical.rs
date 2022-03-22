use crate::token::{Token, TokenBuilder, TokenType};

pub fn match_numerical(b: &mut TokenBuilder) -> Option<Token> {
    if b.curr().is_ascii_digit() {
        let mut integer_part: i64 = b.curr() as i64 - '0' as i64;
        if b.curr() != '0' {
            b.next();
            loop {
                if b.curr().is_ascii_digit() {
                    integer_part = integer_part.checked_mul(10).expect("Int too big D:");
                    integer_part += b.curr() as i64 - '0' as i64;
                    b.next();
                } else if b.curr() == '_' {
                    b.next();
                } else {
                    break;
                }
            }
        } else {
            b.next();
        }
        if let Some(token) = match_float(b, integer_part) {
            Some(token)
        } else {
            Some(b.bake(TokenType::Int(integer_part)))
        }
    } else {
        None
    }
}

pub fn match_float(b: &mut TokenBuilder, integer_part: i64) -> Option<Token> {
    if b.curr() == '.' {
        b.next();
        if b.curr().is_ascii_digit() {
            let mut digits = 1;
            let mut decimal_part: i64 = b.curr() as i64 - '0' as i64;
            b.next();
            loop {
                if b.curr().is_ascii_digit() {
                    decimal_part = decimal_part.checked_mul(10).expect("Too many numbers");
                    digits += 1;
                    decimal_part += b.curr() as i64 - '0' as i64;
                    b.next();
                } else if b.curr() == '_' {
                    b.next();
                } else {
                    break;
                }
            }
            Some(b.bake(TokenType::Float(
                integer_part as f64 + decimal_part as f64 / 10i64.pow(digits) as f64,
            )))
        } else {
            None
        }
    } else {
        None
    }
}
