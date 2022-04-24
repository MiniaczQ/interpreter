use crate::{lexer::{
    lexem::{Lexem, LexemBuilder, LexemType},
    operators::Operator,
}, scannable::Scannable};

pub fn match_comment_or_division(tb: &mut LexemBuilder) -> Option<Lexem> {
    if tb.peek() == '/' {
        tb.pop();
        match tb.peek() {
            '*' => return Some(complete_comment(tb)),
            _ => return Some(tb.bake(LexemType::Operator(Operator::Slash))),
        }
    }
    None
}

fn complete_comment(tb: &mut LexemBuilder) -> Lexem {
    let mut content: Vec<char> = vec![];
    tb.pop();
    loop {
        match tb.peek() {
            '*' => {
                tb.pop();
                match tb.peek() {
                    '/' => return tb.bake(LexemType::Comment(content.into_iter().collect())),
                    c => {
                        content.push('*');
                        content.push(c);
                    }
                }
            }
            c => {
                content.push(c);
            }
        }
        tb.pop();
    }
}
