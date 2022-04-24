use crate::{
    lexer::lexem::{Lexem, LexemBuilder, LexemType},
    scannable::Scannable,
};

/// Matches a string constant
pub fn match_string(tb: &mut LexemBuilder) -> Option<Lexem> {
    if tb.peek() == '"' {
        tb.pop();
        Some(complete_string(tb))
    } else {
        None
    }
}

/// Completes a string constant
fn complete_string(tb: &mut LexemBuilder) -> Lexem {
    let mut content: Vec<char> = vec![];
    loop {
        let c = tb.peek();
        match tb.peek() {
            '\\' => {
                let pos = tb.get_here();
                tb.pop();
                match tb.peek() {
                    '\\' => content.push('\\'),
                    '"' => content.push('"'),
                    c => {
                        eprintln!(
                            "Unknown escape sequence `\\{}` inside string at {}.",
                            c, pos
                        )
                    }
                }
            }
            '\x03' => {
                eprintln!("String started at {} never ends.", tb.get_start());
                break tb.bake_raw(LexemType::String(content.into_iter().collect()));
            }
            '"' => {
                tb.pop();
                break tb.bake_raw(LexemType::String(content.into_iter().collect()));
            }
            _ => content.push(c),
        }
        tb.pop();
    }
}
