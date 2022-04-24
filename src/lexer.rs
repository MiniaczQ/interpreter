use std::io::BufRead;

use crate::{
    first_match,
    matchers::{
        comment::match_comment_or_division, identifier_or_keyword::match_identifier_or_keyword,
        string::match_string,
    },
};
use crate::{matchers::numerical::match_numerical, scanner::*};
use crate::{matchers::operator::match_operator, token::*};

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
        while self.scanner.peek().is_whitespace() {
            self.scanner.pop();
        }
        if self.scanner.peek() == '\x03' {
            return None;
        }
        let b = &mut TokenBuilder::new(&mut self.scanner);
        if let Some(token) = first_match!(
            b,
            match_numerical,
            match_identifier_or_keyword,
            match_operator,
            match_string,
            match_comment_or_division
        ) {
            Some(token)
        } else {
            let invalid_char = b.peek();
            b.pop();
            Some(b.bake(TokenType::Error(format!(
                "Invalid character '{}'",
                invalid_char,
            ))))
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
