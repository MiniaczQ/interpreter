use crate::{
    lexer::{lexem::LexemType, Lexer},
    scannable::Scannable,
};

use self::map::from_lexem;

use super::token::{Token, TokenType};

mod map;

/// Converts `Lexem`s into `Token`s.
/// Skips comments.
pub struct TokenScanner<'a> {
    lexer: &'a mut Lexer,
    curr: Token,
}

impl<'a> TokenScanner<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let mut scanner = Self {
            lexer,
            curr: Token::empty(),
        };
        scanner.pop();
        scanner
    }
}

impl<'a> Scannable<Token> for TokenScanner<'a> {
    fn curr(&self) -> Token {
        self.curr.clone()
    }

    fn pop(&mut self) -> bool {
        let mut opt_lx = self.lexer.next();
        self.curr = loop {
            if let Some(lx) = opt_lx {
                match lx.lexem_type {
                    LexemType::Comment(_) => {
                        opt_lx = self.lexer.next();
                        continue;
                    }
                    LexemType::Operator(v) => {
                        break from_lexem(lx.start, lx.stop, TokenType::Operator(v.into()))
                    }
                    LexemType::Keyword(v) => {
                        break from_lexem(lx.start, lx.stop, TokenType::Keyword(v.into()))
                    }
                    LexemType::Identifier(v) => {
                        break from_lexem(lx.start, lx.stop, TokenType::Identifier(v))
                    }
                    LexemType::String(v) => {
                        break from_lexem(lx.start, lx.stop, TokenType::String(v))
                    }
                    LexemType::Float(v) => {
                        break from_lexem(lx.start, lx.stop, TokenType::Float(v))
                    }
                    LexemType::Int(v) => break from_lexem(lx.start, lx.stop, TokenType::Int(v)),
                }
            } else {
                break Token {
                    token_type: TokenType::EndOfTokens,
                    start: self.curr.stop,
                    ..self.curr
                };
            }
        };
        self.curr.token_type == TokenType::EndOfTokens
    }
}
