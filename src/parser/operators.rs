/// Possible operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Plus,               // +
    Minus,              // -
    Asterisk,           // *
    Slash,              // /
    Modulo,             // %
    ExclamationMark,    // !
    And,                // &
    Or,                 // |
    Unequal,            // !=
    DoubleEqual,        // ==
    Greater,            // >
    GreaterEqual,       // >=
    Lesser,             // <
    LesserEqual,        // <=
    OpenRoundBracket,   // (
    CloseRoundBracket,  // )
    OpenSquareBracket,  // [
    CloseSquareBracket, // ]
    OpenCurlyBracket,   // {
    CloseCurlyBracket,  // }
    Colon,              // :
    DoubleColon,        // ::
    Equal,              // =
    Arrow,              // ->
    Semicolon,          // ;
    Split,              // ,
}
