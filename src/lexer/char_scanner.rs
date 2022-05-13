use std::io::BufRead;

use utf8_chars::BufReadCharsExt;

use crate::{position::Position, scannable::Scannable};

/// Lexer pre-processor and single `char` buffer
pub struct CharScanner {
    source: Box<dyn BufRead>,
    position: Position,
    prev_position: Position,
    current: char,
    next: char,
}

impl CharScanner {
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
        if let Some(c) = self
            .source
            .read_char()
            .expect("Unexpected error while reading input.")
        {
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
        self.position.next_col();
    }

    // Returns the current position
    pub fn last_pos(&self) -> Position {
        self.prev_position
    }
}

impl Scannable<char> for CharScanner {
    /// Moves the scanner one character forward
    ///
    /// Turns all newlines into `'\n'`
    fn pop(&mut self) -> bool {
        self.prev_position = self.position;
        self.forward();
        self.normalize_newlines();
        self.current != '\x03'
    }

    // Returns the current character
    fn curr(&self) -> char {
        self.current
    }
}
