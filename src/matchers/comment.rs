use crate::token::{TokenBuilder, Token, TokenType};

use super::operator::Operator;

pub fn match_comment_or_division(tb: &mut TokenBuilder) -> Option<Token> {
    if tb.peek() == '/' {
        tb.pop();
        match tb.peek() {
            '*' => return Some(complete_comment(tb)),
            _ => return Some(tb.bake(TokenType::Operator(Operator::Slash)))
        }
    }
    None
}

fn complete_comment(tb: &mut TokenBuilder) -> Token {
    let mut content: Vec<char> = vec![];
    tb.pop();
    loop {
        match tb.peek() {
            '*' => {
                tb.pop();
                match tb.peek() {
                    '/' => return tb.bake(TokenType::Comment(content.into_iter().collect())),
                    c => {
                        content.push('*');
                        content.push(c);
                    }
                }
            }
            c => {
                content.push(c);
            }
        }
        tb.pop();
    }
}