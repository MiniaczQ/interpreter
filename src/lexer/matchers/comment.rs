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
                    '/' => {
                        tb.pop();
                        break tb.bake_raw(LexemType::Comment(content.into_iter().collect()));
                    }
                    c => {
                        content.push('*');
                        content.push(c);
                    }
                }
            }
            '\x03' => {
                eprintln!("Comment started at {} never ends.", tb.get_start());
                let t = tb.bake_raw(LexemType::Comment(content.into_iter().collect()));
                tb.pop();
                break t;
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
        matchers::test_utils::{lexem_with, matcher_with},
        operators::Operator,
    };

    use super::match_comment_or_division;

    fn matcher(string: &'static str) -> Option<Lexem> {
        matcher_with(match_comment_or_division, string)
    }

    fn comment_lexem(
        string: &'static str,
        start: (usize, usize),
        stop: (usize, usize),
    ) -> Option<Lexem> {
        lexem_with(LexemType::Comment(string.to_owned()), start, stop)
    }

    fn division_lexem(start: (usize, usize), stop: (usize, usize)) -> Option<Lexem> {
        lexem_with(LexemType::Operator(Operator::Slash), start, stop)
    }

    #[test]
    fn div_simple() {
        assert_eq!(matcher("/"), division_lexem((1, 1), (1, 2)));
    }

    #[test]
    fn com_single() {
        assert_eq!(matcher("//ab"), comment_lexem("ab", (1, 1), (1, 5)));
    }

    #[test]
    fn com_single_multi() {
        assert_eq!(matcher("//a\nb"), comment_lexem("a", (1, 1), (1, 4)));
    }
    #[test]
    fn empty_com_single_multi() {
        assert_eq!(matcher("//\n"), comment_lexem("", (1, 1), (1, 3)));
    }

    #[test]
    fn com_multi() {
        assert_eq!(matcher("/*ab*/"), comment_lexem("ab", (1, 1), (1, 7)));
    }

    #[test]
    fn empty_com_multi() {
        assert_eq!(matcher("/**/"), comment_lexem("", (1, 1), (1, 5)));
    }

    #[test]
    fn com_multi_multi() {
        assert_eq!(matcher("/*a\nb*/"), comment_lexem("a\nb", (1, 1), (2, 4)));
    }

    #[test]
    fn com_multi_no_end() {
        assert_eq!(matcher("/*a\n"), comment_lexem("a\n", (1, 1), (2, 1)));
    }

    #[test]
    fn empty() {
        assert_eq!(matcher(""), None);
    }
}
