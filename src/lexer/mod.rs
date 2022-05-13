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
    // TODO bufor na błędy
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

    use crate::lexer::{keywords::Keyword, operators::Operator, Lexer};

    use super::lexem::{Lexem, LexemType};

    fn correct_output() -> Vec<Lexem> {
        vec![
            Lexem::new(
                LexemType::Comment(" do nothing".to_owned()),
                (1, 1),
                (1, 14),
            ),
            Lexem::new(LexemType::Keyword(Keyword::Fn), (2, 1), (2, 3)),
            Lexem::new(LexemType::Identifier("main".to_owned()), (2, 4), (2, 8)),
            Lexem::new(
                LexemType::Operator(Operator::OpenRoundBracket),
                (2, 8),
                (2, 9),
            ),
            Lexem::new(
                LexemType::Operator(Operator::CloseRoundBracket),
                (2, 9),
                (2, 10),
            ),
            Lexem::new(
                LexemType::Operator(Operator::OpenCurlyBracket),
                (2, 11),
                (2, 12),
            ),
            Lexem::new(LexemType::Keyword(Keyword::Let), (3, 5), (3, 8)),
            Lexem::new(LexemType::Identifier("a".to_owned()), (3, 9), (3, 10)),
            Lexem::new(LexemType::Operator(Operator::Equal), (3, 11), (3, 12)),
            Lexem::new(LexemType::Int(5), (3, 13), (3, 14)),
            Lexem::new(LexemType::Operator(Operator::Semicolon), (3, 14), (3, 15)),
            Lexem::new(
                LexemType::Operator(Operator::CloseCurlyBracket),
                (4, 1),
                (4, 2),
            ),
        ]
    }

    #[test]
    fn test_file() {
        let file = OpenOptions::new()
            .read(true)
            .open("snippets/short.txt")
            .unwrap();
        let parser = Lexer::new(BufReader::new(file));
        let output = parser.into_iter().collect::<Vec<Lexem>>();
        assert_eq!(output, correct_output());
    }

    #[test]
    fn test_string() {
        let string = "// do nothing\nfn main() {\n    let a = 5;\n}";
        let parser = Lexer::new(BufReader::new(string.as_bytes()));
        let output = parser.into_iter().collect::<Vec<Lexem>>();
        assert_eq!(output, correct_output());
    }

    #[test]
    fn invalid_sequence() {
        let string = "invalid $@#@$#@$#$@ sequence breaks$stuff 0#.323";
        let parser = Lexer::new(BufReader::new(string.as_bytes()));
        let correct_output = vec![
            Lexem::new(LexemType::Identifier("invalid".to_owned()), (1, 1), (1, 8)),
            Lexem::new(
                LexemType::Identifier("sequence".to_owned()),
                (1, 21),
                (1, 29),
            ),
            Lexem::new(LexemType::Identifier("breaks".to_owned()), (1, 30), (1, 36)),
            Lexem::new(LexemType::Identifier("stuff".to_owned()), (1, 37), (1, 42)),
            Lexem::new(LexemType::Int(0), (1, 43), (1, 44)),
            Lexem::new(LexemType::Int(323), (1, 46), (1, 49)),
        ];
        let output = parser.into_iter().collect::<Vec<Lexem>>();
        assert_eq!(output, correct_output);
    }

    #[test]
    fn incomplete_string() {
        let string = "// do nothing\nfn main() \"{\n    let a = 5;\n}\n";
        let parser = Lexer::new(BufReader::new(string.as_bytes()));
        let correct_output = vec![
            Lexem::new(
                LexemType::Comment(" do nothing".to_owned()),
                (1, 1),
                (1, 14),
            ),
            Lexem::new(LexemType::Keyword(Keyword::Fn), (2, 1), (2, 3)),
            Lexem::new(LexemType::Identifier("main".to_owned()), (2, 4), (2, 8)),
            Lexem::new(
                LexemType::Operator(Operator::OpenRoundBracket),
                (2, 8),
                (2, 9),
            ),
            Lexem::new(
                LexemType::Operator(Operator::CloseRoundBracket),
                (2, 9),
                (2, 10),
            ),
            Lexem::new(
                LexemType::String("{\n    let a = 5;\n}\n".to_owned()),
                (2, 11),
                (5, 1),
            ),
        ];
        let output = parser.into_iter().collect::<Vec<Lexem>>();
        assert_eq!(output, correct_output);
    }

    #[test]
    fn incomplete_comment() {
        let string = "// do nothing\nfn main() /*{\n    let a = 5;\n}\n";
        let parser = Lexer::new(BufReader::new(string.as_bytes()));
        let correct_output = vec![
            Lexem::new(
                LexemType::Comment(" do nothing".to_owned()),
                (1, 1),
                (1, 14),
            ),
            Lexem::new(LexemType::Keyword(Keyword::Fn), (2, 1), (2, 3)),
            Lexem::new(LexemType::Identifier("main".to_owned()), (2, 4), (2, 8)),
            Lexem::new(
                LexemType::Operator(Operator::OpenRoundBracket),
                (2, 8),
                (2, 9),
            ),
            Lexem::new(
                LexemType::Operator(Operator::CloseRoundBracket),
                (2, 9),
                (2, 10),
            ),
            Lexem::new(
                LexemType::Comment("{\n    let a = 5;\n}\n".to_owned()),
                (2, 11),
                (5, 1),
            ),
        ];
        let output = parser.into_iter().collect::<Vec<Lexem>>();
        assert_eq!(output, correct_output);
    }
}
