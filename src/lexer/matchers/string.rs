use crate::{lexer::lexem::{Lexem, LexemBuilder, LexemType}, scannable::Scannable};

pub fn match_string(tb: &mut LexemBuilder) -> Option<Lexem> {
    if tb.peek() == '"' {
        let mut content: Vec<char> = vec![];
        tb.pop();
        loop {
            let c = tb.peek();
            match tb.peek() {
                '\\' => {
                    tb.pop();
                    match tb.peek() {
                        '\\' => content.push('\\'),
                        '"' => content.push('"'),
                        _ => todo!("unknown escape character"),
                    }
                }
                '\x03' => todo!("string never closed"),
                '"' => {
                    tb.pop();
                    return Some(tb.bake(LexemType::String(content.into_iter().collect())));
                }
                _ => content.push(c),
            }
            tb.pop();
        }
    }
    None
}
