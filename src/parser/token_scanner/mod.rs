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
    curr: Option<Token>,
}

impl<'a> TokenScanner<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let mut scanner = Self { lexer, curr: None };
        scanner.pop();
        scanner
    }
}

impl<'a> Scannable<Option<Token>> for TokenScanner<'a> {
    fn curr(&self) -> Option<Token> {
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
                        break Some(from_lexem(lx.start, lx.stop, TokenType::Operator(v.into())))
                    }
                    LexemType::Keyword(v) => {
                        break Some(from_lexem(lx.start, lx.stop, TokenType::Keyword(v.into())))
                    }
                    LexemType::Identifier(v) => {
                        break Some(from_lexem(lx.start, lx.stop, TokenType::Identifier(v)))
                    }
                    LexemType::String(v) => {
                        break Some(from_lexem(lx.start, lx.stop, TokenType::String(v)))
                    }
                    LexemType::Float(v) => {
                        break Some(from_lexem(lx.start, lx.stop, TokenType::Float(v)))
                    }
                    LexemType::Int(v) => {
                        break Some(from_lexem(lx.start, lx.stop, TokenType::Int(v)))
                    }
                }
            } else {
                break None;
            }
        };
        self.curr.is_some()
    }
}
