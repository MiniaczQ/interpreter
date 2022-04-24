use crate::{
    char_match,
    lexer::lexem::{Lexem, LexemBuilder, LexemType},
    lexer::operators::Operator, scannable::Scannable,
};

type Op = Operator;

#[allow(clippy::manual_map)]
pub fn match_operator(t_b: &mut LexemBuilder) -> Option<Lexem> {
    if let Some(operator) = match t_b.peek() {
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
        ':' => char_match!(t_b, Op::Colon, ':', Op::DoubleColor),
        '&' => char_match!(t_b, Op::And),
        '|' => char_match!(t_b, Op::Or),
        ';' => char_match!(t_b, Op::Semicolon),
        ',' => char_match!(t_b, Op::Split),
        '!' => char_match!(t_b, Op::ExclamationMark),
        '%' => char_match!(t_b, Op::Modulo),
        '.' => char_match!(t_b, Op::Dot),
        _ => None,
    } {
        Some(t_b.bake(LexemType::Operator(operator)))
    } else {
        None
    }
}
