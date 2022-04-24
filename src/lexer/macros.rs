#[macro_export]
/// Yields the first `Some(token)` value or `None` if none matched
macro_rules! first_match {
    ($lexer:expr, $single:expr) => {
        $single($lexer)
    };
    ($lexer:expr, $first:expr, $($other:expr), +) => {
        if let Some(t) = $first($lexer) {
            Some(t)
        } else {
            first_match!($lexer, $($other), +)
        }
    };
}

#[macro_export]
/// Shorthand for branching character matching
/// First two arguments are `TokenBuilder` and explression to be returned in default case
/// Each pair of arguments after that are a character constant and expression to be returned in that case
macro_rules! char_match {
    ($token_builder: expr, $default: expr) => { {
        $token_builder.pop();
        Some($default)
    } };
    ($token_builder: expr, $default: expr, $($pattern: literal, $operator: expr), +) => { {
        $token_builder.pop();
        match $token_builder.peek() {
            $($pattern => {
                $token_builder.pop();
                Some($operator)
            },)*
            _ => Some($default),
        }
    } };
}
