use crate::token::{Token, TokenBuilder, TokenType};

pub fn match_identifier(b: &mut TokenBuilder) -> Option<Token> {
    let mut name: Vec<char> = Vec::new();
    if b.curr().is_ascii_alphabetic() | (b.curr() == '_') {
        name.push(b.curr());
        b.next();
        while b.curr().is_ascii_alphabetic() | (b.curr() == '_') | b.curr().is_ascii_digit() {
            name.push(b.curr());
            b.next();
        }
        Some(b.bake(TokenType::Identifier(name.into_iter().collect())))
    } else {
        None
    }
}
