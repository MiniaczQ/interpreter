use crate::{
    scanner::Scanner,
    token::{Token, TokenType},
};

pub fn match_numerical(scanner: &mut Scanner) -> Option<Token> {
    if scanner.curr().is_ascii_digit() {
        let mut integer_part: i64 = scanner.curr() as i64 - '0' as i64;
        if scanner.curr() != '0' {
            scanner.next();
            loop {
                if scanner.curr().is_ascii_digit() {
                    integer_part = integer_part.checked_mul(10).expect("Int too big D:");
                    integer_part += scanner.curr() as i64 - '0' as i64;
                    scanner.next();
                } else if scanner.curr() == '_' {
                    scanner.next();
                } else {
                    break;
                }
            }
        } else {
            scanner.next();
        }
        if let Some(token) = match_float(scanner, integer_part) {
            Some(token)
        } else {
            Some(Token::new(
                TokenType::Int(integer_part),
                scanner.pos(),
                scanner.pos(),
            ))
        }
    } else {
        None
    }
}

pub fn match_float(scanner: &mut Scanner, integer_part: i64) -> Option<Token> {
    if scanner.curr() == '.' {
        scanner.next();
        if scanner.curr().is_ascii_digit() {
            let mut digits = 1;
            let mut decimal_part: i64 = scanner.curr() as i64 - '0' as i64;
            scanner.next();
            loop {
                if scanner.curr().is_ascii_digit() {
                    decimal_part = decimal_part.checked_mul(10).expect("Int too big D:");
                    digits += 1;
                    decimal_part += scanner.curr() as i64 - '0' as i64;
                    scanner.next();
                } else if scanner.curr() == '_' {
                    scanner.next();
                } else {
                    break;
                }
            }
            Some(Token::new(
                TokenType::Float(
                    integer_part as f64 + decimal_part as f64 / 10i64.pow(digits) as f64,
                ),
                scanner.pos(),
                scanner.pos(),
            ))
        } else {
            None
        }
    } else {
        None
    }
}
