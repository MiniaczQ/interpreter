use crate::{
    lexer::{
        keywords::Keyword as LexerKeyword, operators::Operator as LexerOperator,
        position::Position as LexemPosition,
    },
    parser::{
        keywords::Keyword, operators::Operator, position::Position, token::Token, token::TokenType,
    },
};

impl From<LexerKeyword> for Keyword {
    fn from(kw: LexerKeyword) -> Self {
        match kw {
            LexerKeyword::Int => Keyword::Int,
            LexerKeyword::Float => Keyword::Float,
            LexerKeyword::Bool => Keyword::Bool,
            LexerKeyword::String => Keyword::String,
            LexerKeyword::Let => Keyword::Let,
            LexerKeyword::Fn => Keyword::Fn,
            LexerKeyword::Return => Keyword::Return,
            LexerKeyword::While => Keyword::While,
            LexerKeyword::For => Keyword::For,
            LexerKeyword::In => Keyword::In,
            LexerKeyword::If => Keyword::If,
            LexerKeyword::Else => Keyword::Else,
            LexerKeyword::True => Keyword::True,
            LexerKeyword::False => Keyword::False,
        }
    }
}

impl From<LexerOperator> for Operator {
    fn from(op: LexerOperator) -> Self {
        match op {
            LexerOperator::Plus => Operator::Plus,
            LexerOperator::Minus => Operator::Minus,
            LexerOperator::Asterisk => Operator::Asterisk,
            LexerOperator::Slash => Operator::Slash,
            LexerOperator::Modulo => Operator::Modulo,
            LexerOperator::ExclamationMark => Operator::ExclamationMark,
            LexerOperator::And => Operator::And,
            LexerOperator::Or => Operator::Or,
            LexerOperator::Unequal => Operator::Unequal,
            LexerOperator::DoubleEqual => Operator::DoubleEqual,
            LexerOperator::Greater => Operator::Greater,
            LexerOperator::GreaterEqual => Operator::GreaterEqual,
            LexerOperator::Lesser => Operator::Lesser,
            LexerOperator::LesserEqual => Operator::LesserEqual,
            LexerOperator::OpenRoundBracket => Operator::OpenRoundBracket,
            LexerOperator::CloseRoundBracket => Operator::CloseRoundBracket,
            LexerOperator::OpenSquareBracket => Operator::OpenSquareBracket,
            LexerOperator::CloseSquareBracket => Operator::CloseSquareBracket,
            LexerOperator::OpenCurlyBracket => Operator::OpenCurlyBracket,
            LexerOperator::CloseCurlyBracket => Operator::CloseCurlyBracket,
            LexerOperator::Colon => Operator::Colon,
            LexerOperator::DoubleColon => Operator::DoubleColon,
            LexerOperator::Equal => Operator::Equal,
            LexerOperator::Arrow => Operator::Arrow,
            LexerOperator::Semicolon => Operator::Semicolon,
            LexerOperator::Split => Operator::Split,
        }
    }
}

impl From<LexemPosition> for Position {
    fn from(lp: LexemPosition) -> Self {
        Self {
            row: lp.row,
            col: lp.col,
        }
    }
}

pub fn from_lexem(start: LexemPosition, stop: LexemPosition, t: TokenType) -> Token {
    Token {
        token_type: t,
        start: start.into(),
        stop: stop.into(),
    }
}
