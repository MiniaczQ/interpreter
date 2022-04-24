/// Possible operators
#[derive(Debug, Clone)]
pub enum Operator {
    Plus,            // +
    Minus,           // -
    Asterisk,        // *
    Slash,           // /
    Modulo,          // %
    ExclamationMark, // !

    Equal,        // =
    DoubleEqual,  // ==
    Greater,      // >
    GreaterEqual, // >=
    Lesser,       // <
    LesserEqual,  // <=

    OpenRoundBracket,   // (
    CloseRoundBracket,  // )
    OpenSquareBracket,  // [
    CloseSquareBracket, // ]
    OpenCurlyBracket,   // {
    CloseCurlyBracket,  // }

    Colon,       // :
    DoubleColor, // ::
    Semicolon,   // ;
    Split,       // ,
    Dot,         // .
    And,         // &
    Or,          // |
}
