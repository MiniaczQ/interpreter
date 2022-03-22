use std::io::BufRead;

use utf8_chars::BufReadCharsExt;

use crate::position::Position;

pub struct Scanner {
    source: Box<dyn BufRead>,
    position: Position,
    current: char,
    next: char,
}

/// Lexer pre-processor and single `char` buffer
impl Scanner {
    /// Creates a new tracking scanner from buffered reader
    pub fn new(source: impl BufRead + 'static) -> Self {
        let mut scanner = Self {
            source: Box::new(source),
            position: Position::default(),
            current: '\0',
            next: '\0',
        };
        scanner.next = scanner.next_char();
        scanner.forward();
        scanner
    }

    /// Returns next character, EOF is marked as end of text ASCII character
    fn next_char(&mut self) -> char {
        if let Some(c) = self.source.read_char().unwrap() {
            c
        } else {
            '\x03'
        }
    }

    /// Replaces current newlines with `'\n'`
    fn normalize_newlines(&mut self) {
        match (self.current, self.next) {
            ('\n', '\r') | ('\r', '\n') => {
                self.forward();
                self.position.next_line();
                self.current = '\n';
            }
            ('\n', _) | ('\r', _) => {
                self.position.next_line();
                self.current = '\n';
            }
            _ => {}
        }
    }

    /// Moves the scanner one character forward
    fn forward(&mut self) {
        self.current = self.next;
        self.next = self.next_char();
        self.position.next_bytes(self.current.len_utf8());
        self.position.next_col();
    }

    /// Moves the scanner one character forward
    /// Turns all newlines into `'\n'`
    pub fn next(&mut self) {
        self.forward();
        self.normalize_newlines();
    }

    // Returns the current character
    pub fn curr(&self) -> char {
        self.current
    }

    // Returns the current position
    pub fn pos(&self) -> Position {
        self.position
    }
}
