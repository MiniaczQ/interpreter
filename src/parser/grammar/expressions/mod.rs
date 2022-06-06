use core::fmt::Debug;

use crate::{
    interpreter::{context::Context, ExecutionError},
    parser::Parser,
};

use self::{
    assignment::{parse_variable_assignment_expression, AssignmentExpr},
    binary::BinaryExpr,
    code_block::{parse_code_block_expression, CodeBlockExpr},
    declaration::{parse_variable_declaration, DeclarationExpr},
    for_expr::{parse_for_expression, ForExpr},
    function_call::{parse_identifier_or_function_call_expression, FunctionCallExpr},
    identifier::IdentifierExpr,
    if_else::{parse_if_else_expression, IfElseExpr},
    list::{parse_list_expression, ListExpr},
    list_access::ListAccessExpr,
    literal::{parse_literal_expression, LiteralExpr},
    return_expr::{parse_return, ReturnExpr},
    unary::UnaryExpr,
    while_expr::{parse_while_expression, WhileExpr},
};

use super::{utility::*, Value};

pub mod assignment;
pub mod binary;
pub mod code_block;
pub mod declaration;
pub mod for_expr;
pub mod function_call;
pub mod identifier;
pub mod if_else;
pub mod list;
pub mod list_access;
pub mod literal;
pub mod return_expr;
pub mod statement;
pub mod unary;
pub mod while_expr;

pub trait Evaluable {
    fn eval(&self, ctx: &dyn Context) -> Result<Value, ExecutionError>;
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Expression {
    Assignment(AssignmentExpr),
    Binary(BinaryExpr),
    CodeBlock(CodeBlockExpr),
    Declaration(DeclarationExpr),
    For(ForExpr),
    FunctionCall(FunctionCallExpr),
    Identifier(IdentifierExpr),
    IfElse(IfElseExpr),
    ListAccess(ListAccessExpr),
    List(ListExpr),
    Literal(LiteralExpr),
    Return(ReturnExpr),
    Unary(UnaryExpr),
    While(WhileExpr),
}

impl Evaluable for Expression {
    fn eval(&self, ctx: &dyn Context) -> Result<Value, ExecutionError> {
        match self {
            Expression::Assignment(v) => v.eval(ctx),
            Expression::Binary(v) => v.eval(ctx),
            Expression::CodeBlock(v) => v.eval(ctx),
            Expression::Declaration(v) => v.eval(ctx),
            Expression::For(v) => v.eval(ctx),
            Expression::FunctionCall(v) => v.eval(ctx),
            Expression::Identifier(v) => v.eval(ctx),
            Expression::IfElse(v) => v.eval(ctx),
            Expression::ListAccess(v) => v.eval(ctx),
            Expression::List(v) => v.eval(ctx),
            Expression::Literal(v) => v.eval(ctx),
            Expression::Return(v) => v.eval(ctx),
            Expression::Unary(v) => v.eval(ctx),
            Expression::While(v) => v.eval(ctx),
        }
    }
}

/// grouped
///     = OPEN_BRACKET, expression, CLOSE_BRACKET
///     ;
fn parse_bracket_expression(p: &mut Parser) -> OptRes<Expression> {
    if !p.operator(Op::OpenRoundBracket)? {
        return Ok(None);
    }
    let expression =
        parse_expression(p)?.ok_or_else(|| p.error(ErroVar::InvalidBracketExpression))?;
    if !p.operator(Op::CloseRoundBracket)? {
        p.warn(WarnVar::MissingClosingRoundBracket)?;
    }
    Ok(Some(expression))
}

/// const_or_identifier_or_function_call_expression
///     = constant | list_expression | identifier_or_function_call | grouped
///     ;
fn parse_constant_or_identifier_or_bracket_expression(p: &mut Parser) -> OptRes<Expression> {
    parse_literal_expression(p)
        .alt(|| parse_list_expression(p))
        .alt(|| parse_bracket_expression(p))
        .alt(|| parse_identifier_or_function_call_expression(p))
}

/// control_flow_expression
///     = variable_assignment_expression
///     | for_expression
///     | while_expression
///     | if_expression
///     | code_block
///     ;
fn parse_control_flow_expression(p: &mut Parser) -> OptRes<Expression> {
    parse_variable_assignment_expression(p)
        .alt(|| parse_for_expression(p))
        .alt(|| parse_while_expression(p))
        .alt(|| parse_if_else_expression(p))
        .alt(|| parse_code_block_expression(p))
}

/// expression
///     = return
///     | variable_declaration,
///     | control_flow_expression
///     ;
pub fn parse_expression(p: &mut Parser) -> OptRes<Expression> {
    if let Some(return_expr) = parse_return(p)? {
        Ok(Some(return_expr))
    } else if let Some(declaration) = parse_variable_declaration(p)? {
        Ok(Some(declaration))
    } else {
        parse_control_flow_expression(p)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::grammar::expressions::parse_expression;

    use super::super::test_utils::tests::*;

    #[test]
    fn miss() {
        let (result, warnings) = partial_parse(
            vec![dummy_token(TokenType::Keyword(Kw::Fn))],
            parse_expression,
        );
        assert_eq!(result, Ok(None));

        assert!(warnings.is_empty());
    }

    #[test]
    fn bracket_expr() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Operator(Op::OpenRoundBracket)),
                dummy_token(TokenType::Int(5)),
                dummy_token(TokenType::Operator(Op::CloseRoundBracket)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_expression,
        );
        assert_eq!(result.unwrap().unwrap(), Value::Int(5).into());

        assert!(warnings.is_empty());
    }

    #[test]
    fn bracket_expr_missing_bracket() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Operator(Op::OpenRoundBracket)),
                dummy_token(TokenType::Int(5)),
                token(TokenType::Operator(Op::CloseCurlyBracket), (5, 6), (5, 7)),
            ],
            parse_expression,
        );
        assert_eq!(result.unwrap().unwrap(), Value::Int(5).into());

        assert_eq!(warnings.len(), 1);
        assert_eq!(
            warnings[0],
            ParserWarning {
                warning: ParserWarningVariant::MissingClosingRoundBracket,
                start: Position::new(5, 6),
                stop: Position::new(5, 7)
            }
        );
    }

    #[test]
    fn bracket_expr_missing_expression() {
        let (result, warnings) = partial_parse(
            vec![
                token(TokenType::Operator(Op::OpenRoundBracket), (2, 4), (2, 5)),
                dummy_token(TokenType::Operator(Op::CloseRoundBracket)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::InvalidBracketExpression,
                pos: Position::new(2, 5),
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn out_of_tokens() {
        let (result, warnings) = partial_parse(
            vec![token(TokenType::Keyword(Kw::Let), (5, 6), (5, 9))],
            parse_expression,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::VariableDeclarationMissingIdentifier,
                pos: Position::new(5, 9),
            }
        );

        assert!(warnings.is_empty());
    }
}
