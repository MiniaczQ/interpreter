use crate::{
    lexer::keywords::Keyword,
    lexer::lexem::{Lexem, LexemBuilder, LexemType}, scannable::Scannable,
};

#[inline]
fn can_begin(c: char) -> bool {
    c.is_alphabetic() | (c == '_')
}

#[inline]
fn can_continue(c: char) -> bool {
    c.is_alphabetic() | (c == '_') | c.is_ascii_digit()
}

pub fn match_identifier_or_keyword(tb: &mut LexemBuilder) -> Option<Lexem> {
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
            Some(tb.bake(LexemType::Identifier(name)))
        }
    } else {
        None
    }
}

fn match_keyword(tb: &mut LexemBuilder, name: &str) -> Option<Lexem> {
    match name {
        "int" => Some(tb.bake(LexemType::Keyword(Keyword::Int))),
        "float" => Some(tb.bake(LexemType::Keyword(Keyword::Float))),
        "bool" => Some(tb.bake(LexemType::Keyword(Keyword::Bool))),
        "string" => Some(tb.bake(LexemType::Keyword(Keyword::String))),

        "let" => Some(tb.bake(LexemType::Keyword(Keyword::Let))),
        "fn" => Some(tb.bake(LexemType::Keyword(Keyword::Fn))),
        "return" => Some(tb.bake(LexemType::Keyword(Keyword::Return))),
        "while" => Some(tb.bake(LexemType::Keyword(Keyword::While))),
        "for" => Some(tb.bake(LexemType::Keyword(Keyword::For))),
        "in" => Some(tb.bake(LexemType::Keyword(Keyword::In))),
        "if" => Some(tb.bake(LexemType::Keyword(Keyword::If))),
        "else" => Some(tb.bake(LexemType::Keyword(Keyword::Else))),

        _ => None,
    }
}
