use crate::token::{Token, TokenBuilder, TokenType};

mod keywords {
    const KW_IF: &str = "if";
    const KW_FOREACH: &str = "foreach";
    const KW_THE: &str = "the";
    const KW_IN: &str = "in";
}

mod identifiers {

}

mod operands {
    const OP_PLUS: &str = "+";
}

pub fn match_identifier(b: &mut TokenBuilder) -> Option<Token> {
    let mut name: Vec<char> = Vec::new();
    if b.curr().is_ascii_alphabetic() | (b.curr() == '_') {
        name.push(b.curr());
        b.next();
        while b.curr().is_ascii_alphabetic() | (b.curr() == '_') | b.curr().is_ascii_digit() {
            name.push(b.curr());
            b.next();
        }
        Some(b.bake(TokenType::Identifier(name.into_iter().collect())))
    } else {
        None
    }
}
