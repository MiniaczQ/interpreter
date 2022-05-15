use crate::{
    lexer::lexem::{Lexem, LexemBuilder, LexemType, LexemWarningVariant},
    scannable::Scannable,
};

/// Matches a string constant
pub fn match_string(lb: &mut LexemBuilder, max: usize) -> Option<Lexem> {
    if lb.curr() == '"' {
        lb.pop();
        Some(complete_string(lb, max))
    } else {
        None
    }
}

/// Handles different kinds of escape characters
fn escape_characters(lb: &mut LexemBuilder, content: &mut Vec<char>) {
    match lb.curr() {
        '0' => content.push('\0'),
        'b' => content.push('\x08'),
        'f' => content.push('\x0c'),
        'n' => content.push('\n'),
        'r' => content.push('\r'),
        't' => content.push('\t'),
        '"' => content.push('"'),
        '\\' => content.push('\\'),
        c => {
            lb.error(LexemWarningVariant::InvalidEscapeCharacter(c));
        }
    }
}

/// Completes a string constant
fn complete_string(lb: &mut LexemBuilder, max: usize) -> Lexem {
    let mut content: Vec<char> = vec![];
    loop {
        let c = lb.curr();
        match lb.curr() {
            '\\' => {
                lb.pop();
                escape_characters(lb, &mut content);
            }
            '\x03' => {
                lb.error(LexemWarningVariant::StringNeverEnds);
                break lb.bake_raw(LexemType::String(content.into_iter().collect()));
            }
            '"' => {
                lb.pop();
                break lb.bake_raw(LexemType::String(content.into_iter().collect()));
            }
            _ => content.push(c),
        }
        if content.len() > max {
            content.pop();
            lb.error(LexemWarningVariant::StringTooLong);
            break lb.bake_raw(LexemType::String(content.into_iter().collect()));
        }
        lb.pop();
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::{
        lexem::{Lexem, LexemType, LexemWarning, LexemWarningVariant},
        matchers::test_utils::{lexem_with, matcher_with},
    };

    use super::match_string;

    fn matcher(string: &'static str) -> Option<Lexem> {
        let r = matcher_with(|lb| match_string(lb, 32), string);
        assert!(r.1.is_empty());
        r.0
    }

    fn err_matcher(string: &'static str) -> (Option<Lexem>, Vec<LexemWarning>) {
        matcher_with(|lb| match_string(lb, 32), string)
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
        let (result, errors) = err_matcher("\"abcd");
        assert_eq!(result, lexem("abcd", (1, 1), (1, 6)));
        assert!(errors[0].variant == LexemWarningVariant::StringNeverEnds);
    }

    #[test]
    fn empty_str() {
        assert_eq!(matcher("\"\""), lexem("", (1, 1), (1, 3)));
    }

    #[test]
    fn empty_no_end() {
        let (result, errors) = err_matcher("\"");
        assert_eq!(result, lexem("", (1, 1), (1, 2)));
        assert!(errors[0].variant == LexemWarningVariant::StringNeverEnds);
    }

    #[test]
    fn prepended_whitespace() {
        assert_eq!(matcher(" \"abcd\""), None);
    }

    #[test]
    fn escape() {
        assert_eq!(matcher("\"ab\\\"cd\""), lexem("ab\"cd", (1, 1), (1, 9)));
        assert_eq!(matcher("\"\\0\""), lexem("\0", (1, 1), (1, 5)));
        assert_eq!(matcher("\"\\b\""), lexem("\x08", (1, 1), (1, 5)));
        assert_eq!(matcher("\"\\f\""), lexem("\x0c", (1, 1), (1, 5)));
        assert_eq!(matcher("\"\\n\""), lexem("\n", (1, 1), (1, 5)));
        assert_eq!(matcher("\"\\r\""), lexem("\r", (1, 1), (1, 5)));
        assert_eq!(matcher("\"\\t\""), lexem("\t", (1, 1), (1, 5)));
    }

    #[test]
    fn escape_the_escape() {
        assert_eq!(matcher("\"ab\\\\\""), lexem("ab\\", (1, 1), (1, 7)));
    }

    #[test]
    fn unknown_escape() {
        let (result, errors) = err_matcher("\"abc\\j\"");
        assert_eq!(result, lexem("abc", (1, 1), (1, 8)));
        assert!(errors[0].variant == LexemWarningVariant::InvalidEscapeCharacter('j'));
    }

    #[test]
    fn max_long() {
        assert_eq!(
            matcher("\"___a___b___a___c___a___b___a___d\""),
            lexem("___a___b___a___c___a___b___a___d", (1, 1), (1, 35))
        );
    }

    #[test]
    fn too_long() {
        let (result, errors) = err_matcher("\"___a___b___a___c___a___b___a___d_\"");
        assert_eq!(
            result,
            lexem("___a___b___a___c___a___b___a___d", (1, 1), (1, 34))
        );
        assert!(errors[0].variant == LexemWarningVariant::StringTooLong);
    }

    #[test]
    fn empty() {
        assert_eq!(matcher(""), None);
    }
}
