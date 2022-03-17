use crate::{
    lexer::Lexer,
    token::{Token, TokenType},
};

pub fn match_identifier(lexer: &mut Lexer) -> Option<Token> {
    let mut name: Vec<char> = Vec::new();
    if lexer.character.is_ascii_alphabetic() | (lexer.character == '_') {
        name.push(lexer.character);
        lexer.next_char();
        while lexer.character.is_ascii_alphabetic()
            | (lexer.character == '_')
            | lexer.character.is_ascii_digit()
        {
            name.push(lexer.character);
            lexer.next_char();
        }
        Some(Token {
            token_type: TokenType::Identifier(name.into_iter().collect()),
            byte: lexer.char_reader.get_byte(),
            position: lexer.position,
        })
    } else {
        None
    }
}
