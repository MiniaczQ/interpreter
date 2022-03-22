use crate::{
    scanner::Scanner,
    token::{Token, TokenType},
};

pub fn match_whitespaces(scanner: &mut Scanner) -> Option<Token> {
    while scanner.curr().is_whitespace() {
        scanner.next();
    }
    None
}

pub fn match_etx(scanner: &mut Scanner) -> Option<Token> {
    if scanner.curr() == '\x03' {
        Some(Token::new(
            TokenType::EndOfText,
            scanner.pos(),
            scanner.pos(),
        ))
    } else {
        None
    }
}
