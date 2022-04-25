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

#[cfg(test)]
mod tests {
    use crate::lexer::{
        lexem::{Lexem, LexemType},
        matchers::test_utils::{lexem_with, matcher_with},
    };

    use super::match_string;

    fn matcher(string: &'static str) -> Option<Lexem> {
        matcher_with(match_string, string)
    }

    fn lexem(string: &'static str, start: (usize, usize), stop: (usize, usize)) -> Option<Lexem> {
        lexem_with(LexemType::String(string.to_owned()), start, stop)
    }

    #[test]
    fn simple() {
        assert_eq!(matcher("\"abcd\""), lexem("abcd", (1, 1), (1, 7)));
    }

    #[test]
    fn multiline() {
        assert_eq!(matcher("\"ab\ncd\""), lexem("ab\ncd", (1, 1), (2, 4)));
    }

    #[test]
    fn prepended() {
        assert_eq!(matcher("asd \"abcd\""), None);
    }

    #[test]
    fn postpended() {
        assert_eq!(matcher("\"abcd\" abc"), lexem("abcd", (1, 1), (1, 7)));
    }

    #[test]
    fn no_end() {
        assert_eq!(matcher("\"abcd"), lexem("abcd", (1, 1), (1, 6)));
    }

    #[test]
    fn empty_str() {
        assert_eq!(matcher("\"\""), lexem("", (1, 1), (1, 3)));
    }

    #[test]
    fn empty_no_end() {
        assert_eq!(matcher("\""), lexem("", (1, 1), (1, 2)));
    }

    #[test]
    fn prepended_whitespace() {
        assert_eq!(matcher(" \"abcd\""), None);
    }

    #[test]
    fn escape() {
        assert_eq!(matcher("\"ab\\\"cd\""), lexem("ab\"cd", (1, 1), (1, 9)));
    }

    #[test]
    fn escape_the_escape() {
        assert_eq!(matcher("\"ab\\\\\""), lexem("ab\\", (1, 1), (1, 7)));
    }

    #[test]
    fn unknown_escape() {
        assert_eq!(matcher("\"abc\\d\""), lexem("abc", (1, 1), (1, 8)));
    }

    #[test]
    fn empty() {
        assert_eq!(matcher(""), None);
    }
}
