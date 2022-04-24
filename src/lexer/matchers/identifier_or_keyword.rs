use crate::{
    lexer::keywords::Keyword,
    lexer::lexem::{Lexem, LexemBuilder, LexemType},
    scannable::Scannable,
};

/// Whether a character can start an identifier
#[inline]
fn can_begin(c: char) -> bool {
    c.is_alphabetic() | (c == '_')
}

/// Whether a character can continue an identifier
#[inline]
fn can_continue(c: char) -> bool {
    c.is_alphabetic() | (c == '_') | c.is_ascii_digit()
}

/// Matches an identifier or a keyword
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
            tb.bake(LexemType::Identifier(name))
        }
    } else {
        None
    }
}

/// Matches a keyword
fn match_keyword(tb: &mut LexemBuilder, name: &str) -> Option<Lexem> {
    match name {
        "int" => tb.bake(LexemType::Keyword(Keyword::Int)),
        "float" => tb.bake(LexemType::Keyword(Keyword::Float)),
        "bool" => tb.bake(LexemType::Keyword(Keyword::Bool)),
        "string" => tb.bake(LexemType::Keyword(Keyword::String)),
        "let" => tb.bake(LexemType::Keyword(Keyword::Let)),
        "fn" => tb.bake(LexemType::Keyword(Keyword::Fn)),
        "return" => tb.bake(LexemType::Keyword(Keyword::Return)),
        "while" => tb.bake(LexemType::Keyword(Keyword::While)),
        "for" => tb.bake(LexemType::Keyword(Keyword::For)),
        "in" => tb.bake(LexemType::Keyword(Keyword::In)),
        "if" => tb.bake(LexemType::Keyword(Keyword::If)),
        "else" => tb.bake(LexemType::Keyword(Keyword::Else)),
        _ => None,
    }
}
