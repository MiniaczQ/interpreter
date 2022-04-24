use crate::{
    lexer::{
        lexem::{Lexem, LexemBuilder, LexemType},
        operators::Operator,
    },
    scannable::Scannable,
};

/// Matches:
///  - `/`              - division
///  - `//`             - single line comment
///  - `/* [...] */`    - multi-line comment
pub fn match_comment_or_division(tb: &mut LexemBuilder) -> Option<Lexem> {
    if tb.peek() == '/' {
        tb.pop();
        match tb.peek() {
            '*' => return Some(complete_multi_line_comment(tb)),
            '/' => return Some(complete_single_line_comment(tb)),
            _ => return tb.bake(LexemType::Operator(Operator::Slash)),
        }
    }
    None
}

/// Completes a multi-line comment
fn complete_multi_line_comment(tb: &mut LexemBuilder) -> Lexem {
    let mut content: Vec<char> = vec![];
    tb.pop();
    loop {
        match tb.peek() {
            '*' => {
                tb.pop();
                match tb.peek() {
                    '/' => break tb.bake_raw(LexemType::Comment(content.into_iter().collect())),
                    c => {
                        content.push('*');
                        content.push(c);
                    }
                }
            }
            '\x03' => {
                eprintln!("Comment started at {} never ends.", tb.get_start());
                break tb.bake_raw(LexemType::Comment(content.into_iter().collect()));
            }
            c => {
                content.push(c);
            }
        }
        tb.pop();
    }
}

/// Completes a single-line comment
fn complete_single_line_comment(tb: &mut LexemBuilder) -> Lexem {
    let mut content: Vec<char> = vec![];
    tb.pop();
    loop {
        match tb.peek() {
            '\n' | '\x03' => return tb.bake_raw(LexemType::Comment(content.into_iter().collect())),
            c => {
                content.push(c);
            }
        }
        tb.pop();
    }
}
