use std::io::BufRead;

use crate::token::*;
use crate::{
    first_match,
    matchers::{identifier::match_identifier, whitespace::*},
};
use crate::{matchers::numerical::match_numerical, scanner::*};

pub struct Lexer {
    pub scanner: Scanner,
}

impl Lexer {
    pub fn new(source: impl BufRead + 'static) -> Self {
        Self {
            scanner: Scanner::new(source),
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = first_match!(
            &mut self.scanner,
            match_whitespaces,
            match_etx,
            match_numerical,
            match_identifier
        ) {
            match token.token_type {
                TokenType::EndOfText => None,
                _ => Some(token),
            }
        } else {
            let invalid_char = self.scanner.curr();
            self.scanner.next();
            Some(Token::new(TokenType::Error(format!(
                "Invalid character '{}'",
                invalid_char,
            )), self.scanner.pos(), self.scanner.pos()))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::OpenOptions, io::BufReader};

    use crate::lexer::Lexer;

    #[test]
    fn test_file() {
        let file = OpenOptions::new().read(true).open("test.txt").unwrap();
        let parser = Lexer::new(BufReader::new(file));
        for token in parser {
            println!("{:?}", token);
        }
    }

    #[test]
    fn test_string() {
        let file = b"token1\n\rtoken2\n\rtoken3" as &[u8];
        let parser = Lexer::new(BufReader::new(file));
        for token in parser {
            println!("{:?}", token);
        }
    }
}
