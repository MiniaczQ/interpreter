mod char_scanner;
pub mod keywords;
pub mod lexem;
mod macros;
mod matchers;
pub mod operators;

use std::io::BufRead;

use crate::scannable::Scannable;

use crate::first_match;
use matchers::{
    comment::match_comment_or_division, identifier_or_keyword::match_identifier_or_keyword,
    numerical::match_numerical, operator::match_operator, string::match_string,
};

use self::{
    char_scanner::CharScanner,
    lexem::{Lexem, LexemBuilder, LexemType},
};

pub struct Lexer {
    pub scanner: CharScanner,
}

impl Lexer {
    pub fn new(source: impl BufRead + 'static) -> Self {
        Self {
            scanner: CharScanner::new(source),
        }
    }
}

impl Iterator for Lexer {
    type Item = Lexem;

    fn next(&mut self) -> Option<Self::Item> {
        while self.scanner.peek().is_whitespace() {
            self.scanner.pop();
        }
        if self.scanner.peek() == '\x03' {
            return None;
        }
        let tb = &mut LexemBuilder::new(&mut self.scanner);
        if let Some(token) = first_match!(
            tb,
            match_numerical,
            match_identifier_or_keyword,
            match_operator,
            match_string,
            match_comment_or_division
        ) {
            Some(token)
        } else {
            let invalid_char = tb.peek();
            tb.pop();
            tb.bake(LexemType::Error(format!(
                "Invalid character '{}'",
                invalid_char,
            )))
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
