use crate::{
    lexer::{
        lexem::{Lexem, LexemBuilder, LexemType, LexemWarningVariant},
        operators::Operator,
    },
    scannable::Scannable,
};

/// Matches:
///  - `/`              - division
///  - `//`             - single line comment
///  - `/* [...] */`    - multi-line comment
pub fn match_comment_or_division(lb: &mut LexemBuilder, max: usize) -> Option<Lexem> {
    if lb.curr() == '/' {
        lb.pop();
        match lb.curr() {
            '*' => return Some(complete_multi_line_comment(lb, max)),
            '/' => return Some(complete_single_line_comment(lb, max)),
            _ => return lb.bake(LexemType::Operator(Operator::Slash)),
        }
    }
    None
}

/// Completes a multi-line comment
fn complete_multi_line_comment(lb: &mut LexemBuilder, max: usize) -> Lexem {
    let mut content: Vec<char> = vec![];
    lb.pop();
    loop {
        match lb.curr() {
            '*' => {
                lb.pop();
                match lb.curr() {
                    '/' => {
                        lb.pop();
                        break lb.bake_raw(LexemType::Comment(content.into_iter().collect()));
                    }
                    c => {
                        content.push('*');
                        content.push(c);
                    }
                }
            }
            '\x03' => {
                lb.error(LexemWarningVariant::CommentNeverEnds);
                let t = lb.bake_raw(LexemType::Comment(content.into_iter().collect()));
                lb.pop();
                break t;
            }
            c => {
                content.push(c);
            }
        }
        if content.len() > max {
            content.pop();
            lb.error(LexemWarningVariant::CommentTooLong);
            break lb.bake_raw(LexemType::Comment(content.into_iter().collect()));
        }
        lb.pop();
    }
}

/// Completes a single-line comment
fn complete_single_line_comment(lb: &mut LexemBuilder, max: usize) -> Lexem {
    let mut content: Vec<char> = vec![];
    lb.pop();
    loop {
        match lb.curr() {
            '\n' | '\x03' => break lb.bake_raw(LexemType::Comment(content.into_iter().collect())),
            c => {
                content.push(c);
            }
        }
        if content.len() > max {
            content.pop();
            lb.error(LexemWarningVariant::CommentTooLong);
            break lb.bake_raw(LexemType::Comment(content.into_iter().collect()));
        }
        lb.pop();
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::{
        lexem::{Lexem, LexemType, LexemWarning, LexemWarningVariant},
        matchers::test_utils::{lexem_with, matcher_with},
        operators::Operator,
    };

    use super::match_comment_or_division;

    fn matcher(string: &'static str) -> Option<Lexem> {
        let r = matcher_with(|lb| match_comment_or_division(lb, 32), string);
        assert!(r.1.is_empty());
        r.0
    }

    fn err_matcher(string: &'static str) -> (Option<Lexem>, Vec<LexemWarning>) {
        matcher_with(|lb| match_comment_or_division(lb, 32), string)
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
    fn com_single_max_long() {
        assert_eq!(
            matcher("//___a___b___a___c___a___b___a___d"),
            comment_lexem("___a___b___a___c___a___b___a___d", (1, 1), (1, 35))
        );
    }

    #[test]
    fn com_single_too_long() {
        let (result, errors) = err_matcher("//___a___b___a___c___a___b___a___d_");
        assert_eq!(
            result,
            comment_lexem("___a___b___a___c___a___b___a___d", (1, 1), (1, 35))
        );
        assert!(errors[0].variant == LexemWarningVariant::CommentTooLong);
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
    fn com_multi_max_long() {
        assert_eq!(
            matcher("/*___a___b___a___c___a___b___a___d*/"),
            comment_lexem("___a___b___a___c___a___b___a___d", (1, 1), (1, 37))
        );
    }

    #[test]
    fn com_multi_too_long() {
        let (result, errors) = err_matcher("/*___a___b___a___c___a___b___a___d_*/");
        assert_eq!(
            result,
            comment_lexem("___a___b___a___c___a___b___a___d", (1, 1), (1, 35))
        );
        assert!(errors[0].variant == LexemWarningVariant::CommentTooLong);
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
        let (result, errors) = err_matcher("/*a\n");
        assert_eq!(result, comment_lexem("a\n", (1, 1), (2, 1)));
        assert!(errors[0].variant == LexemWarningVariant::CommentNeverEnds);
    }

    #[test]
    fn empty() {
        assert_eq!(matcher(""), None);
    }
}
