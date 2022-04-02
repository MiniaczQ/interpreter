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

macro_rules! char_match_branches {
    ($token_builder: expr, $pattern:expr, $operator:expr, $($patterns: expr, $operators: expr), +) => { {
        $pattern => {
            token_builder.next();
            Some($operator)
        },
        char_match_branches!($($patterns, $operators), +)
    } };
}

#[macro_export]
macro_rules! char_match {
    ($default: expr, $token_builder: expr) => { {
        $token_builder.next();
        Some($default)
    } };
    ($default: expr, $token_builder: expr, $($patterns: expr, $operators: expr), +) => { {
        $token_builder.next();
        match $token_builder.curr() {
            char_match_branches!($token_builder, $($patterns, $operators), +)
            _ => Some($default),
        }
    } };
}
