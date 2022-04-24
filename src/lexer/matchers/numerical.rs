use crate::{
    lexer::lexem::{Lexem, LexemBuilder, LexemType},
    scannable::Scannable,
};

/// Matches an integer or a float constant
pub fn match_numerical(tb: &mut LexemBuilder) -> Option<Lexem> {
    if tb.peek().is_ascii_digit() {
        let mut integer_part: i64 = tb.peek() as i64 - '0' as i64;
        if tb.peek() != '0' {
            tb.pop();
            loop {
                if tb.peek().is_ascii_digit() {
                    integer_part = integer_part.checked_mul(10).expect("Int too big D:");
                    integer_part += tb.peek() as i64 - '0' as i64;
                    tb.pop();
                } else if tb.peek() == '_' {
                    tb.pop();
                } else {
                    break;
                }
            }
        } else {
            tb.pop();
        }
        if let Some(token) = match_float(tb, integer_part) {
            Some(token)
        } else {
            tb.bake(LexemType::Int(integer_part))
        }
    } else {
        None
    }
}

/// Matches a float constant
fn match_float(tb: &mut LexemBuilder, integer_part: i64) -> Option<Lexem> {
    if tb.peek() == '.' {
        tb.pop();
        if tb.peek().is_ascii_digit() {
            let mut digits = 1;
            let mut decimal_part: i64 = tb.peek() as i64 - '0' as i64;
            tb.pop();
            loop {
                if tb.peek().is_ascii_digit() {
                    decimal_part = decimal_part.checked_mul(10).expect("Too many numbers");
                    digits += 1;
                    decimal_part += tb.peek() as i64 - '0' as i64;
                    tb.pop();
                } else if tb.peek() == '_' {
                    tb.pop();
                } else {
                    break;
                }
            }
            tb.bake(LexemType::Float(
                integer_part as f64 + decimal_part as f64 / 10i64.pow(digits) as f64,
            ))
        } else {
            None
        }
    } else {
        None
    }
}
