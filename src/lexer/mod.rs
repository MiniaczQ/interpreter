mod char_scanner;
pub mod keywords;
pub mod lexem;
mod macros;
mod matchers;
pub mod operators;
pub mod position;

use std::io::BufRead;

use crate::scannable::Scannable;

use matchers::{
    comment::match_comment_or_division, identifier_or_keyword::match_identifier_or_keyword,
    numerical::match_numerical, operator::match_operator, string::match_string,
};

use self::{
    char_scanner::CharScanner,
    lexem::{Lexem, LexemBuilder, LexerWarning, LexerWarningVariant},
};

pub struct Lexer {
    pub max_identifier_length: usize,
    pub max_string_length: usize,
    pub max_comment_length: usize,
    pub scanner: CharScanner,
    pub warnings: Vec<LexerWarning>,
}

impl Lexer {
    pub fn new_with_defaults(source: impl BufRead + 'static) -> Self {
        Self {
            max_identifier_length: 256,
            max_string_length: 256,
            max_comment_length: 256,
            scanner: CharScanner::new(source),
            warnings: vec![],
        }
    }

    #[allow(dead_code)]
    pub fn new(
        source: impl BufRead + 'static,
        max_identifier_length: Option<usize>,
        max_string_length: Option<usize>,
        max_comment_length: Option<usize>,
    ) -> Self {
        Self {
            max_identifier_length: max_identifier_length.unwrap_or(256),
            max_string_length: max_string_length.unwrap_or(256),
            max_comment_length: max_comment_length.unwrap_or(256),
            scanner: CharScanner::new(source),
            warnings: vec![],
        }
    }

    /// Removes whitespace
    fn skip_whitespace(&mut self) {
        while self.scanner.curr().is_whitespace() {
            self.scanner.pop();
        }
    }

    /// Matches lexems
    fn match_lexem(&mut self) -> Option<Lexem> {
        let lb = &mut LexemBuilder::new(&mut self.scanner, &mut self.warnings);
        match_numerical(lb)
            .or_else(|| match_identifier_or_keyword(lb, self.max_identifier_length))
            .or_else(|| match_operator(lb))
            .or_else(|| match_string(lb, self.max_string_length))
            .or_else(|| match_comment_or_division(lb, self.max_comment_length))
    }

    /// Skips whitespace and matches lexems or ETX
    fn skip_and_match(&mut self) -> Option<Option<Lexem>> {
        self.skip_whitespace();
        if self.scanner.curr() == '\x03' {
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
                        self.warnings.push(LexerWarning {
                            start: sequence_start,
                            end: sequence_stop,
                            warning: LexerWarningVariant::InvalidSequence(
                                invalid_sequence.iter().collect::<String>(),
                            ),
                        });
                    }
                    break lexem;
                } else {
                    invalid_sequence.push(self.scanner.curr());
                    self.scanner.pop();
                    sequence_stop = self.scanner.last_pos();
                }
            }
        }
    }

    /// Returns lexems until it runs out
    #[allow(dead_code)]
    pub fn next(&mut self) -> Option<Lexem> {
        self.catch_invalid_sequence()
    }

    /// Returns all lexems
    #[allow(dead_code)]
    pub fn all(&mut self) -> Vec<Lexem> {
        let mut lexems = vec![];
        while let Some(l) = self.catch_invalid_sequence() {
            lexems.push(l);
        }
        lexems
    }

    /// Consumes the lexer and returns the warning buffer.
    pub fn get_warnings(self) -> Vec<LexerWarning> {
        self.warnings
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::OpenOptions, io::BufReader};

    use crate::lexer::{keywords::Keyword, lexem::LexerWarningVariant, operators::Operator, Lexer};

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
            Lexem::new(LexemType::Operator(Operator::Colon), (3, 10), (3, 11)),
            Lexem::new(LexemType::Keyword(Keyword::Int), (3, 12), (3, 15)),
            Lexem::new(LexemType::Operator(Operator::Equal), (3, 16), (3, 17)),
            Lexem::new(LexemType::Int(5), (3, 18), (3, 19)),
            Lexem::new(LexemType::Operator(Operator::Semicolon), (3, 19), (3, 20)),
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
            .open("snippets/very_short.txt")
            .unwrap();
        let mut lexer = Lexer::new_with_defaults(BufReader::new(file));
        let output = lexer.all();
        assert_eq!(output, correct_output());
        assert!(lexer.warnings.is_empty());
    }

    #[test]
    fn test_string() {
        let string = "// do nothing\nfn main() {\n    let a: int = 5;\n}";
        let mut lexer = Lexer::new_with_defaults(BufReader::new(string.as_bytes()));
        let output = lexer.all();
        assert_eq!(output, correct_output());
        assert!(lexer.warnings.is_empty());
    }

    #[test]
    fn invalid_sequence() {
        let string = "invalid $@#@$#@$#$@ sequence breaks$stuff 0#.323";
        let mut lexer = Lexer::new_with_defaults(BufReader::new(string.as_bytes()));
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
        let output = lexer.all();
        assert_eq!(output, correct_output);
        assert!(
            lexer.warnings[0].warning
                == LexerWarningVariant::InvalidSequence("$@#@$#@$#$@".to_owned())
        );
        assert!(lexer.warnings[1].warning == LexerWarningVariant::InvalidSequence("$".to_owned()));
        assert!(lexer.warnings[2].warning == LexerWarningVariant::InvalidSequence("#.".to_owned()));
    }

    #[test]
    fn incomplete_string() {
        let string = "// do nothing\nfn main() \"{\n    let a = 5;\n}\n";
        let mut lexer = Lexer::new_with_defaults(BufReader::new(string.as_bytes()));
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
        let output = lexer.all();
        assert_eq!(output, correct_output);
        assert!(lexer.warnings[0].warning == LexerWarningVariant::StringNeverEnds);
    }

    #[test]
    fn incomplete_comment() {
        let string = "// do nothing\nfn main() /*{\n    let a = 5;\n}\n";
        let mut lexer = Lexer::new_with_defaults(BufReader::new(string.as_bytes()));
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
        let output = lexer.all();
        assert_eq!(output, correct_output);
        assert!(lexer.warnings[0].warning == LexerWarningVariant::CommentNeverEnds);
    }
}
