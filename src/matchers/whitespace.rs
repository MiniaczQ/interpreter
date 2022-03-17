use crate::{
    lexer::Lexer,
    token::{Token, TokenType},
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NewLine {
    CR,
    LF,
    CRLF,
    LFCR,
}

pub fn match_newline(lexer: &mut Lexer) -> Option<NewLine> {
    match lexer.character {
        '\n' => {
            lexer.next_char();
            match lexer.character {
                '\r' => {
                    lexer.next_char();
                    Some(NewLine::LFCR)
                }
                _ => Some(NewLine::LF),
            }
        }
        '\r' => {
            lexer.next_char();
            match lexer.character {
                '\n' => {
                    lexer.next_char();
                    Some(NewLine::CRLF)
                }
                _ => Some(NewLine::CR),
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
