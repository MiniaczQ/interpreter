#[allow(dead_code)]
pub mod tests {
    use crate::{parser::grammar::program::Program, scannable::Scannable};

    use super::super::{
        position::Position,
        token::{Token, TokenType},
        Parser, ParserError, ParserWarning,
    };

    pub struct DummyScanner {
        tokens: Vec<Token>,
        curr: Option<Token>,
    }

    impl DummyScanner {
        fn new(mut tokens: Vec<Token>) -> Self {
            tokens.reverse();
            let mut scanner = Self { tokens, curr: None };
            scanner.pop();
            scanner
        }
    }

    impl Scannable<Option<Token>> for DummyScanner {
        fn curr(&self) -> Option<Token> {
            self.curr.clone()
        }

        fn pop(&mut self) -> bool {
            self.curr = self.tokens.pop();
            self.curr.is_some()
        }
    }

    pub fn dummy_token(token_type: TokenType) -> Token {
        Token {
            token_type,
            start: Position::new(0, 0),
            stop: Position::new(0, 0),
        }
    }

    pub fn token(
        token_type: TokenType,
        (r1, c1): (usize, usize),
        (r2, c2): (usize, usize),
    ) -> Token {
        Token {
            token_type,
            start: Position::new(r1, c1),
            stop: Position::new(r2, c2),
        }
    }

    pub fn parse(tokens: Vec<Token>) -> (Result<Program, ParserError>, Vec<ParserWarning>) {
        let scanner = DummyScanner::new(tokens);
        let mut parser = Parser::new(scanner);
        (parser.parse(), parser.get_warnings())
    }

    pub fn partial_parse<T>(
        tokens: Vec<Token>,
        parse_func: fn(p: &mut Parser) -> Result<Option<T>, ParserError>,
    ) -> (Result<Option<T>, ParserError>, Vec<ParserWarning>) {
        let scanner = DummyScanner::new(tokens);
        let mut parser = Parser::new(scanner);
        (parse_func(&mut parser), parser.get_warnings())
    }
}
