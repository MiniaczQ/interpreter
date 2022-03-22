use crate::{
    lexer::Lexer,
    token::{Token, TokenType},
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NewLine {
    Cr,
    Lf,
    Crlf,
    Lfcr,
}

pub fn match_newline(lexer: &mut Lexer) -> Option<NewLine> {
    match lexer.character {
        '\n' => {
            lexer.next_char();
            match lexer.character {
                '\r' => {
                    lexer.next_char();
                    Some(NewLine::Lfcr)
                }
                _ => Some(NewLine::Lf),
            }
        }
        '\r' => {
            lexer.next_char();
            match lexer.character {
                '\n' => {
                    lexer.next_char();
                    Some(NewLine::Crlf)
                }
                _ => Some(NewLine::Cr),
            }
        }
        _ => None,
    }
}

pub fn match_whitespaces(lexer: &mut Lexer) -> Option<Token> {
    loop {
        if let Some(newline) = match_newline(lexer) {
            if let Some(token) = lexer.next_line(newline) {
                return Some(token);
            }
        } else if lexer.character.is_whitespace() {
            lexer.next_char();
        } else {
            return None;
        }
    }
}

pub fn match_etx(lexer: &mut Lexer) -> Option<Token> {
    if lexer.character == '\x03' {
        lexer.new_token(TokenType::EndOfText)
    } else {
        None
    }
}
