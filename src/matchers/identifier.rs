use crate::{
    scanner::Scanner,
    token::{Token, TokenType},
};

pub fn match_identifier(scanner: &mut Scanner) -> Option<Token> {
    let mut name: Vec<char> = Vec::new();
    if scanner.curr().is_ascii_alphabetic() | (scanner.curr() == '_') {
        name.push(scanner.curr());
        scanner.next();
        while scanner.curr().is_ascii_alphabetic()
            | (scanner.curr() == '_')
            | scanner.curr().is_ascii_digit()
        {
            name.push(scanner.curr());
            scanner.next();
        }
        Some(Token::new(
            TokenType::Identifier(name.into_iter().collect()),
            scanner.pos(),
            scanner.pos(),
        ))
    } else {
        None
    }
}
