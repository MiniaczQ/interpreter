use crate::{lexer::lexem::{Lexem, LexemBuilder, LexemType}, scannable::Scannable};

pub fn match_numerical(b: &mut LexemBuilder) -> Option<Lexem> {
    if b.peek().is_ascii_digit() {
        let mut integer_part: i64 = b.peek() as i64 - '0' as i64;
        if b.peek() != '0' {
            b.pop();
            loop {
                if b.peek().is_ascii_digit() {
                    integer_part = integer_part.checked_mul(10).expect("Int too big D:");
                    integer_part += b.peek() as i64 - '0' as i64;
                    b.pop();
                } else if b.peek() == '_' {
                    b.pop();
                } else {
                    break;
                }
            }
        } else {
            b.pop();
        }
        if let Some(token) = match_float(b, integer_part) {
            Some(token)
        } else {
            Some(b.bake(LexemType::Int(integer_part)))
        }
    } else {
        None
    }
}

pub fn match_float(b: &mut LexemBuilder, integer_part: i64) -> Option<Lexem> {
    if b.peek() == '.' {
        b.pop();
        if b.peek().is_ascii_digit() {
            let mut digits = 1;
            let mut decimal_part: i64 = b.peek() as i64 - '0' as i64;
            b.pop();
            loop {
                if b.peek().is_ascii_digit() {
                    decimal_part = decimal_part.checked_mul(10).expect("Too many numbers");
                    digits += 1;
                    decimal_part += b.peek() as i64 - '0' as i64;
                    b.pop();
                } else if b.peek() == '_' {
                    b.pop();
                } else {
                    break;
                }
            }
            Some(b.bake(LexemType::Float(
                integer_part as f64 + decimal_part as f64 / 10i64.pow(digits) as f64,
            )))
        } else {
            None
        }
    } else {
        None
    }
}
