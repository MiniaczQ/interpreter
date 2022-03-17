use crate::{
    lexer::Lexer,
    token::{Token, TokenType},
};

pub fn match_numerical(lexer: &mut Lexer) -> Option<Token> {
    if lexer.character.is_ascii_digit() {
        let mut integer_part: i64 = lexer.character as i64 - '0' as i64;
        if lexer.character != '0' {
            lexer.next_char();
            loop {
                if lexer.character.is_ascii_digit() {
                    integer_part = integer_part.checked_mul(10).expect("Int too big D:");
                    integer_part += lexer.character as i64 - '0' as i64;
                    lexer.next_char();
                } else if lexer.character == '_' {
                    lexer.next_char();
                } else {
                    break;
                }
            }
        } else {
            lexer.next_char();
        }
        if let Some(token) = match_float(lexer, integer_part) {
            Some(token)
        } else {
            lexer.new_token(TokenType::Int(integer_part))
        }
    } else {
        None
    }
}

pub fn match_float(lexer: &mut Lexer, integer_part: i64) -> Option<Token> {
    if lexer.character == '.' {
        lexer.next_char();
        if lexer.character.is_ascii_digit() {
            let mut digits = 1;
            let mut decimal_part: i64 = lexer.character as i64 - '0' as i64;
            lexer.next_char();
            loop {
                if lexer.character.is_ascii_digit() {
                    decimal_part = decimal_part.checked_mul(10).expect("Int too big D:");
                    digits += 1;
                    decimal_part += lexer.character as i64 - '0' as i64;
                    lexer.next_char();
                } else if lexer.character == '_' {
                    lexer.next_char();
                } else {
                    break;
                }
            }
            lexer.new_token(crate::token::TokenType::Float(
                integer_part as f64 + decimal_part as f64 / 10i64.pow(digits) as f64,
            ))
        } else {
            None
        }
    } else {
        None
    }
}
