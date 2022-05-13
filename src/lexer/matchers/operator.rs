use crate::{
    char_match,
    lexer::lexem::{Lexem, LexemBuilder, LexemType},
    lexer::operators::Operator,
    scannable::Scannable,
};

type Op = Operator;

/// Matches an operator
pub fn match_operator(lb: &mut LexemBuilder) -> Option<Lexem> {
    match lb.curr() {
        '+' => char_match!(lb, Op::Plus),
        '-' => char_match!(lb, Op::Minus, '>', Op::Arrow),
        '*' => char_match!(lb, Op::Asterisk),
        '=' => char_match!(lb, Op::Equal, '=', Op::DoubleEqual),
        '<' => char_match!(lb, Op::Lesser, '=', Op::LesserEqual),
        '>' => char_match!(lb, Op::Greater, '=', Op::GreaterEqual),
        '(' => char_match!(lb, Op::OpenRoundBracket),
        ')' => char_match!(lb, Op::CloseRoundBracket),
        '{' => char_match!(lb, Op::OpenCurlyBracket),
        '}' => char_match!(lb, Op::CloseCurlyBracket),
        '[' => char_match!(lb, Op::OpenSquareBracket),
        ']' => char_match!(lb, Op::CloseSquareBracket),
        ':' => char_match!(lb, Op::Colon, ':', Op::DoubleColon),
        '&' => char_match!(lb, Op::And),
        '|' => char_match!(lb, Op::Or),
        ';' => char_match!(lb, Op::Semicolon),
        ',' => char_match!(lb, Op::Split),
        '!' => char_match!(lb, Op::ExclamationMark, '=', Op::Unequal),
        '%' => char_match!(lb, Op::Modulo),
        _ => None,
    }
    .map(|operator| lb.bake_raw(LexemType::Operator(operator)))
}

#[cfg(test)]
mod tests {
    use crate::lexer::{
        lexem::{Lexem, LexemType},
        matchers::test_utils::{lexem_with, matcher_with},
        operators::Operator,
    };

    use super::match_operator;

    fn matcher(string: &'static str) -> Option<Lexem> {
        let r = matcher_with(match_operator, string);
        assert!(r.1.is_empty());
        r.0
    }

    fn lexem(operator: Operator, start: (usize, usize), stop: (usize, usize)) -> Option<Lexem> {
        lexem_with(LexemType::Operator(operator), start, stop)
    }

    #[test]
    fn all() {
        assert_eq!(matcher("+"), lexem(Operator::Plus, (1, 1), (1, 2)));
        assert_eq!(matcher("-"), lexem(Operator::Minus, (1, 1), (1, 2)));
        assert_eq!(matcher("*"), lexem(Operator::Asterisk, (1, 1), (1, 2)));
        assert_eq!(matcher("%"), lexem(Operator::Modulo, (1, 1), (1, 2)));
        assert_eq!(
            matcher("!"),
            lexem(Operator::ExclamationMark, (1, 1), (1, 2))
        );
        assert_eq!(matcher("="), lexem(Operator::Equal, (1, 1), (1, 2)));
        assert_eq!(matcher("=="), lexem(Operator::DoubleEqual, (1, 1), (1, 3)));
        assert_eq!(matcher(">"), lexem(Operator::Greater, (1, 1), (1, 2)));
        assert_eq!(matcher(">="), lexem(Operator::GreaterEqual, (1, 1), (1, 3)));
        assert_eq!(matcher("<"), lexem(Operator::Lesser, (1, 1), (1, 2)));
        assert_eq!(matcher("<="), lexem(Operator::LesserEqual, (1, 1), (1, 3)));
        assert_eq!(
            matcher("("),
            lexem(Operator::OpenRoundBracket, (1, 1), (1, 2))
        );
        assert_eq!(
            matcher(")"),
            lexem(Operator::CloseRoundBracket, (1, 1), (1, 2))
        );
        assert_eq!(
            matcher("["),
            lexem(Operator::OpenSquareBracket, (1, 1), (1, 2))
        );
        assert_eq!(
            matcher("]"),
            lexem(Operator::CloseSquareBracket, (1, 1), (1, 2))
        );
        assert_eq!(
            matcher("{"),
            lexem(Operator::OpenCurlyBracket, (1, 1), (1, 2))
        );
        assert_eq!(
            matcher("}"),
            lexem(Operator::CloseCurlyBracket, (1, 1), (1, 2))
        );
        assert_eq!(matcher(":"), lexem(Operator::Colon, (1, 1), (1, 2)));
        assert_eq!(matcher("::"), lexem(Operator::DoubleColon, (1, 1), (1, 3)));
        assert_eq!(matcher(";"), lexem(Operator::Semicolon, (1, 1), (1, 2)));
        assert_eq!(matcher(","), lexem(Operator::Split, (1, 1), (1, 2)));
        assert_eq!(matcher("&"), lexem(Operator::And, (1, 1), (1, 2)));
        assert_eq!(matcher("|"), lexem(Operator::Or, (1, 1), (1, 2)));
        assert_eq!(matcher("->"), lexem(Operator::Arrow, (1, 1), (1, 3)));
        assert_eq!(matcher("!="), lexem(Operator::Unequal, (1, 1), (1, 3)));
    }

    #[test]
    fn prepended() {
        assert_eq!(matcher("abcd +"), None);
    }

    #[test]
    fn postpended() {
        assert_eq!(matcher("+ abcd"), lexem(Operator::Plus, (1, 1), (1, 2)));
    }

    #[test]
    fn empty() {
        assert_eq!(matcher(""), None);
    }
}
