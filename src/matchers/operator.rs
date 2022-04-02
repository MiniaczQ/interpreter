use crate::{
    char_match,
    token::{Token, TokenBuilder},
};

pub enum Operator {
    Plus,         // +
    Minus,        // -
    Asterisk,     // *
    Slash,        // /
    DoubleSlash,  // //
    Equal,        // =
    DoubleEqual,  // ==
    Greater,      // >
    GreaterEqual, // >=
    Lesser,       // <
    LesserEqual,  // <=

    OpenRoundBracket,   // (
    CloseRoundBracket,  // )
    OpenAngleBracket,   // <
    CloseAngleBracket,  // >
    OpenSquareBracket,  // [
    CloseSquareBracket, // ]
    OpenCurlyBracket,   // {
    CloseCurlyBracket,  // }
    OpenComment,        // /*
    CloseComment,       // */
    OpenCloneString,    // "

    Dot,             // .
    DoubleDot,       // ..
    Colon,           // :
    DoubleColor,     // ::
    Semicolon,       // ;
    QuestionMark,    // ?
    ExclamationMark, // !
    And,             // &&
    Or,              // ||
}

type Op = Operator;

pub fn match_operator(b: &mut TokenBuilder) -> Option<Token> {
    match b.curr() {
        '+' => Some(Op::Plus),
        '-' => Some(Op::Minus),
        '*' => {
            char_match!(Op::Asterisk, b, '/', Op::CloseComment)
        }
        _ => None,
    };
    None
}
