use crate::{
    char_match,
    lexer::lexem::{Lexem, LexemBuilder, LexemType},
    lexer::operators::Operator,
    scannable::Scannable,
};

type Op = Operator;

/// Matches an operator
pub fn match_operator(t_b: &mut LexemBuilder) -> Option<Lexem> {
    match t_b.peek() {
        '+' => char_match!(t_b, Op::Plus),
        '-' => char_match!(t_b, Op::Minus),
        '*' => char_match!(t_b, Op::Asterisk),
        '=' => char_match!(t_b, Op::Equal, '=', Op::DoubleEqual),
        '<' => char_match!(t_b, Op::Lesser, '=', Op::LesserEqual),
        '>' => char_match!(t_b, Op::Greater, '=', Op::GreaterEqual),
        '(' => char_match!(t_b, Op::OpenRoundBracket),
        ')' => char_match!(t_b, Op::CloseRoundBracket),
        '{' => char_match!(t_b, Op::OpenCurlyBracket),
        '}' => char_match!(t_b, Op::CloseCurlyBracket),
        '[' => char_match!(t_b, Op::OpenSquareBracket),
        ']' => char_match!(t_b, Op::CloseSquareBracket),
        ':' => char_match!(t_b, Op::Colon, ':', Op::DoubleColon),
        '&' => char_match!(t_b, Op::And),
        '|' => char_match!(t_b, Op::Or),
        ';' => char_match!(t_b, Op::Semicolon),
        ',' => char_match!(t_b, Op::Split),
        '!' => char_match!(t_b, Op::ExclamationMark),
        '%' => char_match!(t_b, Op::Modulo),
        '.' => char_match!(t_b, Op::Dot),
        _ => None,
    }
    .map(|operator| t_b.bake_raw(LexemType::Operator(operator)))
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
        matcher_with(match_operator, string)
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
        assert_eq!(matcher("."), lexem(Operator::Dot, (1, 1), (1, 2)));
        assert_eq!(matcher("&"), lexem(Operator::And, (1, 1), (1, 2)));
        assert_eq!(matcher("|"), lexem(Operator::Or, (1, 1), (1, 2)));
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
