use crate::parser::token::TokenType;

use super::{
    code_block::{parse_code_block, CodeBlock},
    conditional::{parse_if_else, IfElse},
    literals::{parse_literal, Literal},
    loops::{parse_for_loop, parse_while_loop, ForLoop, WhileLoop},
    types::parse_type,
    utility::*,
    DataType,
};

/// All possible types of expression
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    ListAccess {
        list: Box<Expression>,
        access: Box<IndexOrRange>,
    },
    FunctionCall {
        identifier: Box<Expression>,
        arguments: Vec<Expression>,
    },
    UnaryOperation {
        operator: UnaryOperator,
        expression: Box<Expression>,
    },
    BinaryOperation {
        operator: BinaryOperator,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Assignment {
        identifier: Box<Expression>,
        expression: Box<Expression>,
    },
    Return(Box<Expression>),
    Declaration {
        identifier: String,
        data_type: DataType,
        expression: Box<Expression>,
    },
    For(Box<ForLoop>),
    While(Box<WhileLoop>),
    IfElse(Box<IfElse>),
    CodeBlock(CodeBlock),
}

/// Two ways of accessing list elements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IndexOrRange {
    Index(Expression),
    Range(Expression, Expression),
}

/// Algebraic negation and logical negation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UnaryOperator {
    AlgebraicNegation,
    LogicalNegation,
}

/// Binary operators
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BinaryOperator {
    Multiplication,
    Division,
    Modulo,
    Addition,
    Subtraction,
    Equal,
    Unequal,
    Lesser,
    LesserEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
}

/// grouped
///     = OPEN_BRACKET, expression, CLOSE_BRACKET
///     ;
fn parse_bracket_expression(p: &mut Parser) -> OptRes<Expression> {
    if !p.operator(Op::OpenRoundBracket)? {
        return Ok(None);
    }
    if let Some(expression) = parse_expression(p)? {
        if !p.operator(Op::CloseRoundBracket)? {
            p.warn(WarnVar::MissingClosingRoundBracket);
        }
        Ok(Some(expression))
    } else {
        p.error(ErroVar::InvalidBracketExpression)
    }
}

/// IDENTIFIER
fn parse_identifier_expression(p: &mut Parser) -> OptRes<Expression> {
    if let Some(identifier) = p.identifier()? {
        return Ok(Some(Expression::Identifier(identifier)));
    }
    Ok(None)
}

/// constant
fn parse_literal_expression(p: &mut Parser) -> OptRes<Expression> {
    parse_literal(p).map(|v| v.map(Expression::Literal))
}

/// const_or_identifier_expression
///     = constant | IDENTIFIER | grouped
///     ;
fn parse_constant_or_identifier_or_bracket_expression(p: &mut Parser) -> OptRes<Expression> {
    parse_literal_expression(p)
        .alt(|| parse_bracket_expression(p))
        .alt(|| parse_identifier_expression(p))
}

/// index_or_range_access
///     = expression, [RANGE, expression]
///     ;
fn parse_index_or_range_access(p: &mut Parser) -> OptRes<IndexOrRange> {
    if let Some(left_index) = parse_expression(p)? {
        return if p.operator(Op::DoubleColon)? {
            if let Some(right_index) = parse_expression(p)? {
                Ok(Some(IndexOrRange::Range(left_index, right_index)))
            } else {
                p.error(ErroVar::ListRangeAccessIncomplete)
            }
        } else {
            Ok(Some(IndexOrRange::Index(left_index)))
        };
    }
    Ok(None)
}

/// list_access
///     = OPEN_LIST, index_or_range_access, CLOSE_LIST
///     ;
fn parse_list_access(p: &mut Parser) -> OptRes<IndexOrRange> {
    if !p.operator(Op::OpenSquareBracket)? {
        return Ok(None);
    }
    if let Some(index_or_range) = parse_index_or_range_access(p)? {
        if !p.operator(Op::CloseSquareBracket)? {
            p.warn(WarnVar::MissingClosingSquareBracket);
        }
        Ok(Some(index_or_range))
    } else {
        p.error(ErroVar::ListAccessEmpty)
    }
}

/// function_arguments
///     = [expression, {SPLIT, expression}]
///     ;
fn parse_function_arguments(p: &mut Parser) -> Res<Vec<Expression>> {
    let mut arguments = vec![];
    if let Some(argument) = parse_expression(p)? {
        arguments.push(argument);
        while p.operator(Op::Split)? {
            if let Some(argument) = parse_expression(p)? {
                arguments.push(argument);
            } else {
                p.warn(WarnVar::TrailingComma);
            }
        }
    }
    Ok(arguments)
}

/// function_call
///     = OPEN_BRACKET, function_arguments, CLOSE_BRACKET
///     ;
fn parse_function_call(p: &mut Parser) -> OptRes<Vec<Expression>> {
    if !p.operator(Op::OpenRoundBracket)? {
        return Ok(None);
    }
    let args = parse_function_arguments(p)?;
    if !p.operator(Op::CloseRoundBracket)? {
        p.warn(WarnVar::MissingClosingRoundBracket);
    }
    Ok(Some(args))
}

/// function_call_or_list_access_expression
///     = const_or_identifier_expression, (function_call | list_access)
///     ;
fn parse_function_call_or_list_access_expression(p: &mut Parser) -> OptRes<Expression> {
    if let Some(expression) = parse_constant_or_identifier_or_bracket_expression(p)? {
        return if let Some(arguments) = parse_function_call(p)? {
            Ok(Some(Expression::FunctionCall {
                identifier: Box::new(expression),
                arguments,
            }))
        } else if let Some(access) = parse_list_access(p)? {
            Ok(Some(Expression::ListAccess {
                list: Box::new(expression),
                access: Box::new(access),
            }))
        } else {
            Ok(Some(expression))
        };
    }
    Ok(None)
}

/// unary_operators
///     = OP_NEGATE | OP_MINUS
///     ;
fn parse_unary_operators(p: &mut Parser) -> OptRes<UnaryOperator> {
    match p.token()?.token_type {
        TokenType::Operator(Op::Minus) => {
            p.pop();
            Ok(Some(UnaryOperator::AlgebraicNegation))
        }
        TokenType::Operator(Op::ExclamationMark) => {
            p.pop();
            Ok(Some(UnaryOperator::LogicalNegation))
        }
        _ => Ok(None),
    }
}

/// unary_operator_expression
///     = unary_operators, unary_operator_expression
///     | function_call_or_list_access_expression
///     ;
fn parse_unary_operator_expression(p: &mut Parser) -> OptRes<Expression> {
    if let Some(operator) = parse_unary_operators(p)? {
        if let Some(expression) = parse_unary_operator_expression(p)? {
            Ok(Some(Expression::UnaryOperation {
                operator,
                expression: Box::new(expression),
            }))
        } else {
            p.error(ErroVar::UnaryOperatorMissingExpression)
        }
    } else if let Some(expression) = parse_function_call_or_list_access_expression(p)? {
        Ok(Some(expression))
    } else {
        Ok(None)
    }
}

/// mul_div_operators
///     = OP_MULTIPLICATION | OP_DIVISION | OP_REMAINDER
///     ;
fn parse_mul_div_operators(p: &mut Parser) -> OptRes<BinaryOperator> {
    match p.token()?.token_type {
        TokenType::Operator(Op::Asterisk) => {
            p.pop();
            Ok(Some(BinaryOperator::Multiplication))
        }
        TokenType::Operator(Op::Slash) => {
            p.pop();
            Ok(Some(BinaryOperator::Division))
        }
        TokenType::Operator(Op::Modulo) => {
            p.pop();
            Ok(Some(BinaryOperator::Modulo))
        }
        _ => Ok(None),
    }
}

/// mul_div_expression
///     = unary_operator_expression, {mul_div_operators, unary_operator_expression}
///     ;
fn parse_mul_div_expression(p: &mut Parser) -> OptRes<Expression> {
    if let Some(mut lhs) = parse_unary_operator_expression(p)? {
        while let Some(operator) = parse_mul_div_operators(p)? {
            if let Some(rhs) = parse_unary_operator_expression(p)? {
                lhs = Expression::BinaryOperation {
                    operator,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                };
            } else {
                return p.error(ErroVar::BinaryOperatorMissingRHS);
            }
        }
        Ok(Some(lhs))
    } else {
        Ok(None)
    }
}

/// add_sub_operators
///     = OP_PLUS | OP_MINUS
///     ;
fn parse_add_sub_operators(p: &mut Parser) -> OptRes<BinaryOperator> {
    match p.token()?.token_type {
        TokenType::Operator(Op::Plus) => {
            p.pop();
            Ok(Some(BinaryOperator::Addition))
        }
        TokenType::Operator(Op::Minus) => {
            p.pop();
            Ok(Some(BinaryOperator::Subtraction))
        }
        _ => Ok(None),
    }
}

/// add_sub_expression
///     = mul_div_expression, {add_sub_operators, mul_div_expression}
///     ;
fn parse_add_sub_expression(p: &mut Parser) -> OptRes<Expression> {
    if let Some(mut lhs) = parse_mul_div_expression(p)? {
        while let Some(operator) = parse_add_sub_operators(p)? {
            if let Some(rhs) = parse_mul_div_expression(p)? {
                lhs = Expression::BinaryOperation {
                    operator,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                };
            } else {
                return p.error(ErroVar::BinaryOperatorMissingRHS);
            }
        }
        Ok(Some(lhs))
    } else {
        Ok(None)
    }
}

/// comparison_operators
///     = OP_EQUAL | OP_UNEQUAL | OP_LESSER | OP_LESSER_EQUAL | OP_GREATER | OP_GREATER_EQUAL
///     ;
fn parse_comparison_operators(p: &mut Parser) -> OptRes<BinaryOperator> {
    match p.token()?.token_type {
        TokenType::Operator(Op::DoubleEqual) => {
            p.pop();
            Ok(Some(BinaryOperator::Equal))
        }
        TokenType::Operator(Op::Unequal) => {
            p.pop();
            Ok(Some(BinaryOperator::Unequal))
        }
        TokenType::Operator(Op::Lesser) => {
            p.pop();
            Ok(Some(BinaryOperator::Lesser))
        }
        TokenType::Operator(Op::LesserEqual) => {
            p.pop();
            Ok(Some(BinaryOperator::LesserEqual))
        }
        TokenType::Operator(Op::Greater) => {
            p.pop();
            Ok(Some(BinaryOperator::Greater))
        }
        TokenType::Operator(Op::GreaterEqual) => {
            p.pop();
            Ok(Some(BinaryOperator::GreaterEqual))
        }
        _ => Ok(None),
    }
}

/// comparison_expression
///     = add_sub_expression, {comparison_operators, add_sub_expression}
///     ;
fn parse_comparison_expression(p: &mut Parser) -> OptRes<Expression> {
    if let Some(mut lhs) = parse_add_sub_expression(p)? {
        while let Some(operator) = parse_comparison_operators(p)? {
            if let Some(rhs) = parse_add_sub_expression(p)? {
                lhs = Expression::BinaryOperation {
                    operator,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                };
            } else {
                return p.error(ErroVar::BinaryOperatorMissingRHS);
            }
        }
        Ok(Some(lhs))
    } else {
        Ok(None)
    }
}

/// logical_conjunction_expression
///     = comparison_expression, {OP_AND, comparison_expression}
///     ;
fn parse_logical_conjunction_expression(p: &mut Parser) -> OptRes<Expression> {
    if let Some(mut lhs) = parse_comparison_expression(p)? {
        while p.operator(Op::And)? {
            if let Some(rhs) = parse_comparison_expression(p)? {
                lhs = Expression::BinaryOperation {
                    operator: BinaryOperator::And,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                };
            } else {
                return p.error(ErroVar::BinaryOperatorMissingRHS);
            }
        }
        Ok(Some(lhs))
    } else {
        Ok(None)
    }
}

/// logical_alternative_expression
///     = logical_conjunction_expression, {OP_OR, logical_conjunction_expression}
///     ;
fn parse_logical_alternative_expression(p: &mut Parser) -> OptRes<Expression> {
    if let Some(mut lhs) = parse_logical_conjunction_expression(p)? {
        while p.operator(Op::Or)? {
            if let Some(rhs) = parse_logical_conjunction_expression(p)? {
                lhs = Expression::BinaryOperation {
                    operator: BinaryOperator::Or,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                };
            } else {
                return p.error(ErroVar::BinaryOperatorMissingRHS);
            }
        }
        Ok(Some(lhs))
    } else {
        Ok(None)
    }
}

/// variable_assignment_expression
///     = logical_alternative_expression, [ASSIGN, expression]
///     ;
fn parse_variable_assignment_expression(p: &mut Parser) -> OptRes<Expression> {
    if let Some(mut lhs) = parse_logical_alternative_expression(p)? {
        if p.operator(Op::Equal)? {
            if let Some(rhs) = parse_logical_alternative_expression(p)? {
                lhs = Expression::Assignment {
                    identifier: Box::new(lhs),
                    expression: Box::new(rhs),
                };
            } else {
                return p.error(ErroVar::AssignmentMissingExpression);
            }
        }
        Ok(Some(lhs))
    } else {
        Ok(None)
    }
}

/// Declaration of a variable
struct VariableDeclaration {
    identifier: String,
    data_type: DataType,
}

/// variable_declaration
///     = KW_LET, IDENTIFIER, TYPE_SIGNATURE, type, ASSIGN
///     ;
fn parse_variable_declaration(p: &mut Parser) -> OptRes<VariableDeclaration> {
    if !p.keyword(Kw::Let)? {
        return Ok(None);
    }
    if let Some(identifier) = p.identifier()? {
        if !p.operator(Op::Colon)? {
            p.warn(WarnVar::VariableDeclarationMissingTypeSeparator);
        }
        if let Some(data_type) = parse_type(p)? {
            if !p.operator(Op::Equal)? {
                p.warn(WarnVar::VariableDeclarationMissingEqualsSign);
            }
            Ok(Some(VariableDeclaration {
                identifier,
                data_type,
            }))
        } else {
            p.error(ErroVar::VariableDeclarationMissingType)
        }
    } else {
        p.error(ErroVar::VariableDeclarationMissingIdentifier)
    }
}

/// for_expression
fn parse_for_expression(p: &mut Parser) -> OptRes<Expression> {
    parse_for_loop(p).map(|v| v.map(|v| Expression::For(Box::new(v))))
}

/// while_expression
fn parse_while_expression(p: &mut Parser) -> OptRes<Expression> {
    parse_while_loop(p).map(|v| v.map(|v| Expression::While(Box::new(v))))
}

/// if_expression
fn parse_if_else_expression(p: &mut Parser) -> OptRes<Expression> {
    parse_if_else(p).map(|v| v.map(|v| Expression::IfElse(Box::new(v))))
}

/// for_expression
fn parse_code_block_expression(p: &mut Parser) -> OptRes<Expression> {
    parse_code_block(p).map(|v| v.map(Expression::CodeBlock))
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
///     = [KW_RETURN | variable_declaration], control_flow_expression
///     ;
pub fn parse_expression(p: &mut Parser) -> OptRes<Expression> {
    if p.keyword(Kw::Return)? {
        if let Some(expression) = parse_control_flow_expression(p)? {
            Ok(Some(Expression::Return(Box::new(expression))))
        } else {
            p.error(ErroVar::ReturnMissingExpression)
        }
    } else if let Some(variable_declaration) = parse_variable_declaration(p)? {
        if let Some(expression) = parse_control_flow_expression(p)? {
            Ok(Some(Expression::Declaration {
                identifier: variable_declaration.identifier,
                data_type: variable_declaration.data_type,
                expression: Box::new(expression),
            }))
        } else {
            p.error(ErroVar::VariableDeclarationMissingExpression)
        }
    } else {
        parse_control_flow_expression(p)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::grammar::{
        code_block::{CodeBlock, Statement},
        conditional::IfElse,
        expressions::{parse_expression, IndexOrRange},
        loops::{ForLoop, WhileLoop},
    };

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
        assert_eq!(
            result.unwrap().unwrap(),
            Expression::Literal(Literal(Value::Integer(5)))
        );

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
        assert_eq!(
            result.unwrap().unwrap(),
            Expression::Literal(Literal(Value::Integer(5)))
        );

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
    fn literal() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Int(7)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            Expression::Literal(Literal(Value::Integer(7)))
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn identifier() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            Expression::Identifier("a".to_owned())
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn list_access_index() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenSquareBracket)),
                dummy_token(TokenType::Int(1)),
                dummy_token(TokenType::Operator(Op::CloseSquareBracket)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            Expression::ListAccess {
                list: Box::new(Expression::Identifier("a".to_owned())),
                access: Box::new(IndexOrRange::Index(Expression::Literal(Literal(
                    Value::Integer(1)
                ))))
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn list_access_empty() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Identifier("a".to_owned())),
                token(TokenType::Operator(Op::OpenSquareBracket), (3, 4), (3, 5)),
                dummy_token(TokenType::Operator(Op::CloseSquareBracket)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::ListAccessEmpty,
                pos: Position::new(3, 5),
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn list_access_range() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenSquareBracket)),
                dummy_token(TokenType::Int(1)),
                dummy_token(TokenType::Operator(Op::DoubleColon)),
                dummy_token(TokenType::Int(5)),
                dummy_token(TokenType::Operator(Op::CloseSquareBracket)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            Expression::ListAccess {
                list: Box::new(Expression::Identifier("a".to_owned())),
                access: Box::new(IndexOrRange::Range(
                    Expression::Literal(Literal(Value::Integer(1))),
                    Expression::Literal(Literal(Value::Integer(5)))
                ))
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn list_access_missing_closing_bracket() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenSquareBracket)),
                dummy_token(TokenType::Int(1)),
                dummy_token(TokenType::Operator(Op::DoubleColon)),
                dummy_token(TokenType::Int(5)),
                token(TokenType::Operator(Op::Semicolon), (7, 8), (7, 9)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            Expression::ListAccess {
                list: Box::new(Expression::Identifier("a".to_owned())),
                access: Box::new(IndexOrRange::Range(
                    Expression::Literal(Literal(Value::Integer(1))),
                    Expression::Literal(Literal(Value::Integer(5)))
                ))
            }
        );

        assert_eq!(warnings.len(), 1);
        assert_eq!(
            warnings[0],
            ParserWarning {
                warning: ParserWarningVariant::MissingClosingSquareBracket,
                start: Position::new(7, 8),
                stop: Position::new(7, 9)
            }
        );
    }

    #[test]
    fn list_access_range_incomplete() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenSquareBracket)),
                dummy_token(TokenType::Int(1)),
                token(TokenType::Operator(Op::DoubleColon), (3, 8), (3, 10)),
                dummy_token(TokenType::Operator(Op::CloseSquareBracket)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::ListRangeAccessIncomplete,
                pos: Position::new(3, 10)
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn function_call() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenRoundBracket)),
                dummy_token(TokenType::Int(30)),
                dummy_token(TokenType::Operator(Op::Split)),
                dummy_token(TokenType::String("ccc".to_owned())),
                dummy_token(TokenType::Operator(Op::CloseRoundBracket)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            Expression::FunctionCall {
                identifier: Box::new(Expression::Identifier("a".to_owned())),
                arguments: vec![
                    Expression::Literal(Literal(Value::Integer(30))),
                    Expression::Literal(Literal(Value::String("ccc".to_owned())))
                ],
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn function_call_empty() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenRoundBracket)),
                dummy_token(TokenType::Operator(Op::CloseRoundBracket)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            Expression::FunctionCall {
                identifier: Box::new(Expression::Identifier("a".to_owned())),
                arguments: vec![],
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn function_call_trailing_comma() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenRoundBracket)),
                dummy_token(TokenType::Int(30)),
                dummy_token(TokenType::Operator(Op::Split)),
                dummy_token(TokenType::String("ccc".to_owned())),
                dummy_token(TokenType::Operator(Op::Split)),
                token(TokenType::Operator(Op::CloseRoundBracket), (6, 15), (6, 16)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            Expression::FunctionCall {
                identifier: Box::new(Expression::Identifier("a".to_owned())),
                arguments: vec![
                    Expression::Literal(Literal(Value::Integer(30))),
                    Expression::Literal(Literal(Value::String("ccc".to_owned())))
                ],
            }
        );

        assert_eq!(warnings.len(), 1);
        assert_eq!(
            warnings[0],
            ParserWarning {
                warning: ParserWarningVariant::TrailingComma,
                start: Position::new(6, 15),
                stop: Position::new(6, 16)
            }
        );
    }

    #[test]
    fn function_call_missing_closing_bracket() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenRoundBracket)),
                dummy_token(TokenType::Int(30)),
                dummy_token(TokenType::Operator(Op::Split)),
                dummy_token(TokenType::String("ccc".to_owned())),
                token(TokenType::Operator(Op::Semicolon), (13, 20), (13, 21)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            Expression::FunctionCall {
                identifier: Box::new(Expression::Identifier("a".to_owned())),
                arguments: vec![
                    Expression::Literal(Literal(Value::Integer(30))),
                    Expression::Literal(Literal(Value::String("ccc".to_owned())))
                ],
            }
        );

        assert_eq!(warnings.len(), 1);
        assert_eq!(
            warnings[0],
            ParserWarning {
                warning: ParserWarningVariant::MissingClosingRoundBracket,
                start: Position::new(13, 20),
                stop: Position::new(13, 21)
            }
        );
    }

    #[test]
    fn unary_algebraic_negation() {}

    #[test]
    fn unary_logical_negation() {}

    #[test]
    fn unary_operation_missing_expression() {}

    #[test]
    fn binary_multiplication() {}

    #[test]
    fn binary_division() {}

    #[test]
    fn binary_modulo() {}

    #[test]
    fn binary_addition() {}

    #[test]
    fn binary_subtraction() {}

    #[test]
    fn binary_equal() {}

    #[test]
    fn binary_unequal() {}

    #[test]
    fn binary_lesser() {}

    #[test]
    fn binary_lesser_equal() {}

    #[test]
    fn binary_greater() {}

    #[test]
    fn binary_greater_equal() {}

    #[test]
    fn binary_and() {}

    #[test]
    fn binary_or() {}

    #[test]
    fn binary_operation_missing_rhs() {}

    #[test]
    fn assignment() {}

    #[test]
    fn assignment_missing_expression() {}

    #[test]
    fn declaration() {}

    #[test]
    fn declaration_missing_type_separator() {}

    #[test]
    fn declaration_missing_equals_sign() {}

    #[test]
    fn declaration_missing_type() {}

    #[test]
    fn declaration_missing_identifier() {}

    #[test]
    fn declaration_missing_expression() {}

    #[test]
    fn for_loop() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::For)),
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Keyword(Kw::In)),
                dummy_token(TokenType::Identifier("b".to_owned())),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            Expression::For(Box::new(ForLoop {
                variable: "a".to_owned(),
                provider: Expression::Identifier("b".to_owned()),
                body: CodeBlock { statements: vec![] }
            }))
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn while_loop() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::While)),
                dummy_token(TokenType::Keyword(Kw::True)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            Expression::While(Box::new(WhileLoop {
                condition: Expression::Literal(Literal(Value::Bool(true))),
                body: CodeBlock { statements: vec![] }
            }))
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn if_else() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::If)),
                dummy_token(TokenType::Keyword(Kw::True)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
                dummy_token(TokenType::Keyword(Kw::Else)),
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            Expression::IfElse(Box::new(IfElse {
                condition: Expression::Literal(Literal(Value::Bool(true))),
                true_case: CodeBlock { statements: vec![] },
                false_case: Some(CodeBlock { statements: vec![] }),
            }))
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn code_block() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Operator(Op::OpenCurlyBracket)),
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Operator(Op::Semicolon)),
                dummy_token(TokenType::Int(5)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
                dummy_token(TokenType::Operator(Op::CloseCurlyBracket)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            Expression::CodeBlock(CodeBlock {
                statements: vec![
                    Statement::Expression(Expression::Identifier("a".to_owned())),
                    Statement::Semicolon,
                    Statement::Expression(Expression::Literal(Literal(Value::Integer(5)))),
                    Statement::Semicolon,
                ]
            })
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn return_expr() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::Return)),
                dummy_token(TokenType::Int(0)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            Expression::Return(Box::new(Expression::Literal(Literal(Value::Integer(0)))))
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn return_expr_missing_expression() {
        let (result, warnings) = partial_parse(
            vec![
                token(TokenType::Keyword(Kw::Return), (4, 2), (4, 8)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::ReturnMissingExpression,
                pos: Position::new(4, 8),
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
                error: ParserErrorVariant::OutOfTokens,
                pos: Position::new(5, 9),
            }
        );

        assert!(warnings.is_empty());
    }
}
