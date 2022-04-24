mod char_scanner;
pub mod keywords;
pub mod lexem;
mod macros;
mod matchers;
pub mod operators;

use std::io::BufRead;

use crate::scannable::Scannable;

use matchers::{
    comment::match_comment_or_division, identifier_or_keyword::match_identifier_or_keyword,
    numerical::match_numerical, operator::match_operator, string::match_string,
};

use self::{
    char_scanner::CharScanner,
    lexem::{Lexem, LexemBuilder},
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

    /// Removes whitespace
    fn skip_whitespace(&mut self) {
        while self.scanner.peek().is_whitespace() {
            self.scanner.pop();
        }
    }

    /// Matches lexems
    fn match_lexem(&mut self) -> Option<Lexem> {
        let tb = &mut LexemBuilder::new(&mut self.scanner);
        match_numerical(tb)
            .or_else(|| match_identifier_or_keyword(tb))
            .or_else(|| match_operator(tb))
            .or_else(|| match_string(tb))
            .or_else(|| match_comment_or_division(tb))
    }

    /// Skips whitespace and matches lexems or ETX
    fn skip_and_match(&mut self) -> Option<Option<Lexem>> {
        self.skip_whitespace();
        if self.scanner.peek() == '\x03' {
            Some(None)
        } else {
            self.match_lexem().map(Some)
        }
    }

    /// Catches sequences of invalid characters
    fn catch_invalid_sequence(&mut self) -> Option<Lexem> {
        if let Some(lexem) = self.skip_and_match() {
            lexem
        } else {
            let mut invalid_sequence: Vec<char> = vec![];
            let sequence_start = self.scanner.last_pos();
            let mut sequence_stop = self.scanner.last_pos();
            loop {
                if let Some(lexem) = self.skip_and_match() {
                    if !invalid_sequence.is_empty() {
                        eprintln!(
                            "Invalid sequence of characters `{}` from {} to {}",
                            invalid_sequence.iter().collect::<String>(),
                            sequence_start,
                            sequence_stop
                        )
                    }
                    break lexem;
                } else {
                    invalid_sequence.push(self.scanner.peek());
                    self.scanner.pop();
                    sequence_stop = self.scanner.last_pos();
                }
            }
        }
    }
}

impl Iterator for Lexer {
    type Item = Lexem;

    fn next(&mut self) -> Option<Self::Item> {
        self.catch_invalid_sequence()
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
