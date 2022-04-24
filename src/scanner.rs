use std::io::BufRead;

use utf8_chars::BufReadCharsExt;

use crate::position::Position;

/// Lexer pre-processor and single `char` buffer
pub struct Scanner {
    source: Box<dyn BufRead>,
    position: Position,
    prev_position: Position,
    current: char,
    next: char,
}

impl Scanner {
    /// Creates a new tracking scanner from buffered reader
    pub fn new(source: impl BufRead + 'static) -> Self {
        let mut scanner = Self {
            source: Box::new(source),
            position: Position::default(),
            prev_position: Position::default(),
            current: '\0',
            next: '\0',
        };
        scanner.next = scanner.next_char();
        scanner.forward();
        scanner
    }

    /// Returns next character, EOF is marked as end of text ASCII character
    fn next_char(&mut self) -> char {
        if let Some(c) = self.source.read_char().expect("Unexpected error while reading input.") {
            c
        } else {
            '\x03'
        }
    }

    /// Replaces newlines with `'\n'`
    fn normalize_newlines(&mut self) {
        match (self.current, self.next) {
            ('\n', '\r') | ('\r', '\n') => {
                self.forward();
                self.position.next_line();
                self.current = '\n';
            }
            ('\n', _) | ('\r', _) | ('\x1e', _) => {
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
    pub fn pop(&mut self) {
        self.prev_position = self.position;
        self.forward();
        self.normalize_newlines();
    }

    // Returns the current character
    pub fn peek(&self) -> char {
        self.current
    }

    // Returns the current position
    pub fn last_pos(&self) -> Position {
        self.prev_position
    }
}

pub trait Scan {
    fn peek(&self) -> char;
    fn pop(&mut self);
    // Return the start position
    fn last_pos(&self) -> Position;
}
