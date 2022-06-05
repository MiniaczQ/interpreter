#[allow(dead_code)]
pub mod tests {
    use crate::{parser::grammar::program::ProgramCtx, scannable::Scannable};

    use super::super::{
        position::Position,
        token::{Token, TokenType},
        Parser, ParserError, ParserWarning,
    };

    pub struct DummyScanner {
        tokens: Vec<Token>,
        curr: Token,
    }

    impl DummyScanner {
        pub fn new(mut tokens: Vec<Token>) -> Self {
            tokens.reverse();
            let mut scanner = Self {
                tokens,
                curr: Token::empty(),
            };
            scanner.pop();
            scanner
        }
    }

    impl Scannable<Token> for DummyScanner {
        fn curr(&self) -> Token {
            self.curr.clone()
        }

        fn pop(&mut self) -> bool {
            self.curr = if let Some(t) = self.tokens.pop() {
                t
            } else {
                Token {
                    token_type: TokenType::EndOfTokens,
                    start: self.curr.stop,
                    ..self.curr
                }
            };
            self.curr.token_type != TokenType::EndOfTokens
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

    pub fn parse(tokens: Vec<Token>) -> (Result<ProgramCtx, ParserError>, Vec<ParserWarning>) {
        let scanner = DummyScanner::new(tokens);
        let mut parser = Parser::new_with_defaults(scanner);
        (parser.parse(), parser.get_warnings())
    }
}
