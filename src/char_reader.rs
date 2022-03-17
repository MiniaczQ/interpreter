use std::io::BufRead;

use utf8_chars::BufReadCharsExt;

pub struct CharReader {
    source: Box<dyn BufRead>,
    byte: usize,
}

impl CharReader {
    pub fn new(source: impl BufRead + 'static) -> Self {
        Self {
            source: Box::new(source),
            byte: 0,
        }
    }

    pub fn get_byte(&self) -> usize {
        self.byte
    }
}

impl Iterator for CharReader {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.byte += 1;
        self.source.read_char().unwrap()
    }
}
