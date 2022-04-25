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
        "true" => tb.bake(LexemType::Keyword(Keyword::True)),
        "false" => tb.bake(LexemType::Keyword(Keyword::False)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::{
        keywords::Keyword,
        lexem::{Lexem, LexemType},
        matchers::test_utils::{lexem_with, matcher_with},
    };

    use super::match_identifier_or_keyword;

    fn matcher(string: &'static str) -> Option<Lexem> {
        matcher_with(match_identifier_or_keyword, string)
    }

    fn id_lexem(
        string: &'static str,
        start: (usize, usize),
        stop: (usize, usize),
    ) -> Option<Lexem> {
        lexem_with(LexemType::Identifier(string.to_owned()), start, stop)
    }

    fn kw_lexem(keyword: Keyword, start: (usize, usize), stop: (usize, usize)) -> Option<Lexem> {
        lexem_with(LexemType::Keyword(keyword), start, stop)
    }

    #[test]
    fn kw_all() {
        assert_eq!(matcher("int"), kw_lexem(Keyword::Int, (1, 1), (1, 4)));
        assert_eq!(matcher("float"), kw_lexem(Keyword::Float, (1, 1), (1, 6)));
        assert_eq!(matcher("bool"), kw_lexem(Keyword::Bool, (1, 1), (1, 5)));
        assert_eq!(matcher("string"), kw_lexem(Keyword::String, (1, 1), (1, 7)));
        assert_eq!(matcher("let"), kw_lexem(Keyword::Let, (1, 1), (1, 4)));
        assert_eq!(matcher("fn"), kw_lexem(Keyword::Fn, (1, 1), (1, 3)));
        assert_eq!(matcher("return"), kw_lexem(Keyword::Return, (1, 1), (1, 7)));
        assert_eq!(matcher("while"), kw_lexem(Keyword::While, (1, 1), (1, 6)));
        assert_eq!(matcher("for"), kw_lexem(Keyword::For, (1, 1), (1, 4)));
        assert_eq!(matcher("in"), kw_lexem(Keyword::In, (1, 1), (1, 3)));
        assert_eq!(matcher("if"), kw_lexem(Keyword::If, (1, 1), (1, 3)));
        assert_eq!(matcher("else"), kw_lexem(Keyword::Else, (1, 1), (1, 5)));
        assert_eq!(matcher("true"), kw_lexem(Keyword::True, (1, 1), (1, 5)));
        assert_eq!(matcher("false"), kw_lexem(Keyword::False, (1, 1), (1, 6)));
    }

    #[test]
    fn kw_like() {
        assert_eq!(matcher("foreach"), id_lexem("foreach", (1, 1), (1, 8)));
        assert_eq!(matcher("inside"), id_lexem("inside", (1, 1), (1, 7)));
        assert_eq!(matcher("_if"), id_lexem("_if", (1, 1), (1, 4)));
        assert_eq!(matcher("If"), id_lexem("If", (1, 1), (1, 3)));
        assert_eq!(matcher("LET"), id_lexem("LET", (1, 1), (1, 4)));
        assert_eq!(matcher("whilee"), id_lexem("whilee", (1, 1), (1, 7)));
    }

    #[test]
    fn kw_prepended() {
        assert_eq!(matcher("432 for"), None);
    }

    #[test]
    fn kw_postpended() {
        assert_eq!(matcher("for asd"), kw_lexem(Keyword::For, (1, 1), (1, 4)));
    }

    #[test]
    fn id_simple() {
        assert_eq!(matcher("abcd"), id_lexem("abcd", (1, 1), (1, 5)));
        assert_eq!(matcher("_abcd"), id_lexem("_abcd", (1, 1), (1, 6)));
        assert_eq!(matcher("__abcd__"), id_lexem("__abcd__", (1, 1), (1, 9)));
        assert_eq!(matcher("a4dasd"), id_lexem("a4dasd", (1, 1), (1, 7)));
        assert_eq!(matcher("a___bc_d"), id_lexem("a___bc_d", (1, 1), (1, 9)));
        assert_eq!(matcher("_0"), id_lexem("_0", (1, 1), (1, 3)));
        assert_eq!(matcher("_"), id_lexem("_", (1, 1), (1, 2)));
    }

    #[test]
    fn id_not() {
        assert_eq!(matcher("5sdas"), None);
        assert_eq!(matcher("╕"), None);
        assert_eq!(matcher(":)"), None);
        assert_eq!(matcher("@8@#"), None);
        assert_eq!(matcher("(hello there)"), None);
    }

    #[test]
    fn id_partial() {
        assert_eq!(matcher("_asdd$@"), id_lexem("_asdd", (1, 1), (1, 6)));
        assert_eq!(matcher("_a31$sdd$@"), id_lexem("_a31", (1, 1), (1, 5)));
        assert_eq!(matcher("a31_$_ads"), id_lexem("a31_", (1, 1), (1, 5)));
        assert_eq!(matcher("a_tas23$"), id_lexem("a_tas23", (1, 1), (1, 8)));
    }

    #[test]
    fn id_prepended() {
        assert_eq!(matcher("432 abbc"), None);
    }

    #[test]
    fn id_postpended() {
        assert_eq!(matcher("dsa 231"), id_lexem("dsa", (1, 1), (1, 4)));
    }

    #[test]
    fn empty() {
        assert_eq!(matcher(""), None);
    }
}
