use std::io::BufReader;

use crate::lexer::{
    char_scanner::CharScanner,
    lexem::{Lexem, LexemBuilder, LexemType, LexemWarning},
};

#[allow(dead_code)]
pub fn matcher_with(
    matcher: fn(&mut LexemBuilder) -> Option<Lexem>,
    string: &'static str,
) -> (Option<Lexem>, Vec<LexemWarning>) {
    let mut scanner = CharScanner::new(BufReader::new(string.as_bytes()));
    let mut errors: Vec<LexemWarning> = vec![];
    let lb = &mut LexemBuilder::new(&mut scanner, &mut errors);
    (matcher(lb), errors)
}

#[allow(dead_code)]
pub fn lexem_with(
    lexem_type: LexemType,
    start: (usize, usize),
    stop: (usize, usize),
) -> Option<Lexem> {
    Some(Lexem {
        lexem_type,
        start: start.into(),
        stop: stop.into(),
    })
}
