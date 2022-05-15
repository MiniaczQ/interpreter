#[allow(dead_code)]
pub mod tests {
    use crate::parser::token::Token;

    pub use super::super::super::test_utils::tests::*;

    pub use super::super::utility::*;

    pub use super::super::super::position::Position;

    pub use super::super::super::grammar;

    pub use super::super::super::{
        token::TokenType, ParserError, ParserErrorVariant, ParserWarning, ParserWarningVariant,
    };

    pub use grammar::{expressions::Expression, literals::Literal, Value};

    pub fn partial_parse<T>(
        tokens: Vec<Token>,
        parse_func: fn(p: &mut Parser) -> Result<Option<T>, ParserError>,
    ) -> (Result<Option<T>, ParserError>, Vec<ParserWarning>) {
        let scanner = DummyScanner::new(tokens);
        let mut parser = Parser::new(scanner);
        (parse_func(&mut parser), parser.get_warnings())
    }

    pub fn partial_parse_non_opt<T>(
        tokens: Vec<Token>,
        parse_func: fn(p: &mut Parser) -> Result<T, ParserError>,
    ) -> (Result<T, ParserError>, Vec<ParserWarning>) {
        let scanner = DummyScanner::new(tokens);
        let mut parser = Parser::new(scanner);
        (parse_func(&mut parser), parser.get_warnings())
    }
}
