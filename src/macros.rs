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
