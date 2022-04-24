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

#[cfg(test)]
mod tests {
    use crate::lexer::{
        lexem::{Lexem, LexemType},
        matchers::test_utils::{lexem_with, matcher_with}, operators::Operator,
    };

    use super::match_comment_or_division;

    fn matcher(string: &'static str) -> Option<Lexem> {
        matcher_with(match_operator, string)
    }

    fn comment_lexem(operator: Operator, start: (usize, usize), stop: (usize, usize)) -> Option<Lexem> {
        lexem_with(LexemType::Operator(operator), start, stop)
    }

    fn division_lexem(start: (usize, usize), stop: (usize, usize)) -> Option<Lexem> {
        lexem_with(LexemType::Operator(operator), start, stop)
    }

    #[test]
    fn all() {
        assert_eq!(matcher("+"), lexem(Operator::Plus, (1, 1), (1, 2)));
        todo!();
    }
}