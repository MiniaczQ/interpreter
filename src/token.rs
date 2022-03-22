use crate::{position::Position, scanner::Scanner};

#[derive(Debug, Clone)]
pub enum TokenType {
    //OpPlus,
    //OpMinus,

    //Comment,
    Identifier(String),
    //String(String),
    Float(f64),
    Int(i64),

    Error(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub start: Position,
    pub stop: Position,
}

pub struct TokenBuilder<'a> {
    scanner: &'a mut Scanner,
    start: Position,
}

impl<'a> TokenBuilder<'a> {
    pub fn new(scanner: &'a mut Scanner) -> Self {
        let start = (&*scanner).prev_pos();
        Self { scanner, start }
    }

    pub fn next(&mut self) {
        self.scanner.next()
    }

    pub fn curr(&self) -> char {
        self.scanner.curr()
    }

    pub fn bake(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            start: self.start,
            stop: self.scanner.prev_pos(),
        }
    }
}
