use crate::{token::{Token, TokenBuilder, TokenType}, lexer::keywords::Keyword};

#[inline]
fn can_begin(c: char) -> bool {
    c.is_alphabetic() | (c == '_')
}

#[inline]
fn can_continue(c: char) -> bool {
    c.is_alphabetic() | (c == '_') | c.is_ascii_digit()
}

pub fn match_identifier_or_keyword(tb: &mut TokenBuilder) -> Option<Token> {
    if can_begin(tb.peek()) {
        let mut name = vec![tb.peek()];
        tb.pop();
        while can_continue(tb.peek()) {
            name.push(tb.peek());
            tb.pop();
        }
        let name: String = name.into_iter().collect();
        if let Some(token) = match_keyword(tb, &name) {
            Some(token)
        } else {
            Some(tb.bake(TokenType::Identifier(name)))
        }
    } else {
        None
    }
}

fn match_keyword(tb: &mut TokenBuilder, name: &str) -> Option<Token> {
    match name {
        "int" => Some(tb.bake(TokenType::Keyword(Keyword::Int))),
        "float" => Some(tb.bake(TokenType::Keyword(Keyword::Float))),
        "bool" => Some(tb.bake(TokenType::Keyword(Keyword::Bool))),
        "string" => Some(tb.bake(TokenType::Keyword(Keyword::String))),

        "let" => Some(tb.bake(TokenType::Keyword(Keyword::Let))),
        "fn" => Some(tb.bake(TokenType::Keyword(Keyword::Fn))),
        "return" => Some(tb.bake(TokenType::Keyword(Keyword::Return))),
        "while" => Some(tb.bake(TokenType::Keyword(Keyword::While))),
        "for" => Some(tb.bake(TokenType::Keyword(Keyword::For))),
        "in" => Some(tb.bake(TokenType::Keyword(Keyword::In))),
        "if" => Some(tb.bake(TokenType::Keyword(Keyword::If))),
        "else" => Some(tb.bake(TokenType::Keyword(Keyword::Else))),
        
        _ => None,
    }
}