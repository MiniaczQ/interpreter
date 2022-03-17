use std::io::BufRead;

use crate::matchers::{identifier::match_identifier, whitespace::*};
use crate::position::*;
use crate::token::*;
use crate::{char_reader::*, matchers::numerical::match_numerical};

pub struct Lexer {
    pub char_reader: CharReader,
    pub position: Position,
    pub newline: Option<NewLine>,
    pub character: char,
}

impl Lexer {
    pub fn new(source: impl BufRead + 'static) -> Self {
        let mut lexer = Self {
            char_reader: CharReader::new(source),
            position: Position::default(),
            newline: None,
            character: '\0',
        };
        lexer.next_char();
        lexer
    }

    pub fn next_char(&mut self) {
        self.position.next_char();
        match self.char_reader.next() {
            Some(c) => {
                self.character = c;
            }
            None => {
                self.character = '\x03';
            }
        }
    }

    pub fn new_token(&self, token: TokenType) -> Option<Token> {
        Some(Token {
            token_type: token,
            byte: self.char_reader.get_byte(),
            position: self.position,
        })
    }

    pub fn next_line(&mut self, newline: NewLine) -> Option<Token> {
        if self.newline.is_none() {
            self.newline = Some(newline);
            self.position.next_line();
            None
        } else {
            if self.newline == Some(newline) {
                self.position.next_line();
                None
            } else {
                self.new_token(TokenType::Error(format!(
                    "{:?} line ending used in file with {:?} line endings",
                    newline, self.newline,
                )))
            }
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = match_whitespaces(self) {
            return Some(token);
        };
        if let Some(_token) = match_etx(self) {
            return None;
        }
        if let Some(token) = match_numerical(self) {
            return Some(token);
        }
        if let Some(token) = match_identifier(self) {
            return Some(token);
        }
        let invalid_char = self.character;
        self.next_char();
        self.new_token(TokenType::Error(format!(
            "Invalid character '{}'",
            invalid_char,
        )))
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
