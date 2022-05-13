use std::io::BufReader;

use crate::lexer::{
    char_scanner::CharScanner,
    lexem::{Lexem, LexemBuilder, LexemType},
};

#[allow(dead_code)]
pub fn matcher_with(
    matcher: fn(&mut LexemBuilder) -> Option<Lexem>,
    string: &'static str,
) -> Option<Lexem> {
    let scanner = &mut CharScanner::new(BufReader::new(string.as_bytes()));
    let lb = &mut LexemBuilder::new(scanner);
    matcher(lb)
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
