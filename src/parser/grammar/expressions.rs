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
/// W dokumentacji jest AST visit, visitor
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Expression {
    Literal(Literal),
    List(Vec<Expression>),
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
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum UnaryOperator {
    AlgebraicNegation,
    LogicalNegation,
}

/// Binary operators
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
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

/// IDENTIFIER
fn parse_identifier_expression(p: &mut Parser) -> OptRes<Expression> {
    if let Some(identifier) = p.identifier()? {
        return Ok(Some(Expression::Identifier(identifier)));
    }
    Ok(None)
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
                p.warn(WarnVar::ExpectedExpression)?;
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
        p.warn(WarnVar::MissingClosingRoundBracket)?;
    }
    Ok(Some(args))
}

/// identifier_or_function_call
///     = IDENTIFIER, [function_call]
///     ;
fn parse_identifier_or_function_call_expression(p: &mut Parser) -> OptRes<Expression> {
    if let Some(mut expression) = parse_identifier_expression(p)? {
        if let Some(arguments) = parse_function_call(p)? {
            expression = Expression::FunctionCall {
                identifier: Box::new(expression),
                arguments,
            };
        }
        return Ok(Some(expression));
    }
    Ok(None)
}

/// constant
fn parse_literal_expression(p: &mut Parser) -> OptRes<Expression> {
    parse_literal(p).map(|v| v.map(Expression::Literal))
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

/// list_expression
///     = OPEN_LIST, [expression, {SPLIT, expression}], CLOSE_LIST
///     ;
fn parse_list_expression(p: &mut Parser) -> OptRes<Expression> {
    let mut list: Vec<Expression> = vec![];
    if !p.operator(Op::OpenSquareBracket)? {
        return Ok(None);
    }
    if let Some(expression) = parse_expression(p)? {
        list.push(expression);
        while p.operator(Op::Split)? {
            if let Some(expression) = parse_expression(p)? {
                list.push(expression);
            } else {
                p.warn(WarnVar::ExpectedExpression)?;
            }
        }
    }
    if !p.operator(Op::CloseSquareBracket)? {
        p.warn(WarnVar::MissingClosingSquareBracket)?;
    }
    Ok(Some(Expression::List(list)))
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

/// index_or_range_access
///     = expression, [RANGE, expression]
///     ;
fn parse_index_or_range_access(p: &mut Parser) -> Res<IndexOrRange> {
    let left_index = parse_expression(p)?.ok_or_else(|| p.error(ErroVar::ListAccessEmpty))?;
    if !p.operator(Op::DoubleColon)? {
        return Ok(IndexOrRange::Index(left_index));
    }
    let right_index =
        parse_expression(p)?.ok_or_else(|| p.error(ErroVar::ListRangeAccessIncomplete))?;
    Ok(IndexOrRange::Range(left_index, right_index))
}

/// list_access
///     = OPEN_LIST, index_or_range_access, CLOSE_LIST
///     ;
fn parse_list_access(p: &mut Parser) -> OptRes<IndexOrRange> {
    if !p.operator(Op::OpenSquareBracket)? {
        return Ok(None);
    }
    let index_or_range = parse_index_or_range_access(p)?;
    if !p.operator(Op::CloseSquareBracket)? {
        p.warn(WarnVar::MissingClosingSquareBracket)?;
    }
    Ok(Some(index_or_range))
}

/// list_access_expression
///     = const_or_identifier_or_function_call_expression, [list_access]
///     ;
fn parse_list_access_expression(p: &mut Parser) -> OptRes<Expression> {
    if let Some(mut expression) = parse_constant_or_identifier_or_bracket_expression(p)? {
        if let Some(access) = parse_list_access(p)? {
            expression = Expression::ListAccess {
                list: Box::new(expression),
                access: Box::new(access),
            };
        }
        return Ok(Some(expression));
    }
    Ok(None)
}

/// unary_operators
///     = OP_NEGATE | OP_MINUS
///     ;
fn parse_unary_operators(p: &mut Parser) -> OptRes<UnaryOperator> {
    match p.curr().token_type {
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
///     = {unary_operators}, list_access_expression
///     ;
fn parse_unary_operator_expression(p: &mut Parser) -> OptRes<Expression> {
    if let Some(operator) = parse_unary_operators(p)? {
        let expression = parse_unary_operator_expression(p)?
            .ok_or_else(|| p.error(ErroVar::UnaryOperatorMissingExpression))?;
        Ok(Some(Expression::UnaryOperation {
            operator,
            expression: Box::new(expression),
        }))
    } else if let Some(expression) = parse_list_access_expression(p)? {
        Ok(Some(expression))
    } else {
        Ok(None)
    }
}

/// mul_div_operators
///     = OP_MULTIPLICATION | OP_DIVISION | OP_REMAINDER
///     ;
fn parse_mul_div_operators(p: &mut Parser) -> OptRes<BinaryOperator> {
    match p.curr().token_type {
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
            let rhs = parse_unary_operator_expression(p)?
                .ok_or_else(|| p.error(ErroVar::BinaryOperatorMissingRHS))?;
            lhs = Expression::BinaryOperation {
                operator,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
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
    match p.curr().token_type {
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
            let rhs = parse_mul_div_expression(p)?
                .ok_or_else(|| p.error(ErroVar::BinaryOperatorMissingRHS))?;
            lhs = Expression::BinaryOperation {
                operator,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
        }
        return Ok(Some(lhs));
    }
    Ok(None)
}

/// comparison_operators
///     = OP_EQUAL | OP_UNEQUAL | OP_LESSER | OP_LESSER_EQUAL | OP_GREATER | OP_GREATER_EQUAL
///     ;
fn parse_comparison_operators(p: &mut Parser) -> OptRes<BinaryOperator> {
    match p.curr().token_type {
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
            let rhs = parse_add_sub_expression(p)?
                .ok_or_else(|| p.error(ErroVar::BinaryOperatorMissingRHS))?;
            lhs = Expression::BinaryOperation {
                operator,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
        }
        return Ok(Some(lhs));
    }
    Ok(None)
}

/// logical_conjunction_expression
///     = comparison_expression, {OP_AND, comparison_expression}
///     ;
fn parse_logical_conjunction_expression(p: &mut Parser) -> OptRes<Expression> {
    if let Some(mut lhs) = parse_comparison_expression(p)? {
        while p.operator(Op::And)? {
            let rhs = parse_comparison_expression(p)?
                .ok_or_else(|| p.error(ErroVar::BinaryOperatorMissingRHS))?;
            lhs = Expression::BinaryOperation {
                operator: BinaryOperator::And,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
        }
        return Ok(Some(lhs));
    }
    Ok(None)
}

/// logical_alternative_expression
///     = logical_conjunction_expression, {OP_OR, logical_conjunction_expression}
///     ;
fn parse_logical_alternative_expression(p: &mut Parser) -> OptRes<Expression> {
    if let Some(mut lhs) = parse_logical_conjunction_expression(p)? {
        while p.operator(Op::Or)? {
            let rhs = parse_logical_conjunction_expression(p)?
                .ok_or_else(|| p.error(ErroVar::BinaryOperatorMissingRHS))?;
            lhs = Expression::BinaryOperation {
                operator: BinaryOperator::Or,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
        }
        return Ok(Some(lhs));
    }
    Ok(None)
}

/// variable_assignment_expression
///     = logical_alternative_expression, {ASSIGN, expression}
///     ;
fn parse_variable_assignment_expression(p: &mut Parser) -> OptRes<Expression> {
    if let Some(mut lhs) = parse_logical_alternative_expression(p)? {
        while p.operator(Op::Equal)? {
            let rhs = parse_logical_alternative_expression(p)?
                .ok_or_else(|| p.error(ErroVar::AssignmentMissingExpression))?;
            lhs = Expression::Assignment {
                identifier: Box::new(lhs),
                expression: Box::new(rhs),
            };
        }
        return Ok(Some(lhs));
    }
    Ok(None)
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

/// Declaration of a variable
struct VariableDeclaration {
    identifier: String,
    data_type: DataType,
}

/// variable_declaration
///     = KW_LET, IDENTIFIER, COLON, TYPE_SIGNATURE, type, ASSIGN
///     ;
fn parse_variable_declaration(p: &mut Parser) -> OptRes<VariableDeclaration> {
    if !p.keyword(Kw::Let)? {
        return Ok(None);
    }
    let identifier = p
        .identifier()?
        .ok_or_else(|| p.error(ErroVar::VariableDeclarationMissingIdentifier))?;
    if !p.operator(Op::Colon)? {
        p.warn(WarnVar::VariableDeclarationMissingTypeSeparator)?;
    }
    let data_type =
        parse_type(p)?.ok_or_else(|| p.error(ErroVar::VariableDeclarationMissingType))?;
    if !p.operator(Op::Equal)? {
        p.warn(WarnVar::VariableDeclarationMissingEqualsSign)?;
    }
    Ok(Some(VariableDeclaration {
        identifier,
        data_type,
    }))
}

/// KW_RETURN
pub fn parse_return(p: &mut Parser) -> OptRes<()> {
    if !p.keyword(Kw::Return)? {
        return Ok(None);
    }
    Ok(Some(()))
}

/// expression
///     = [KW_RETURN | variable_declaration], control_flow_expression
///     ;
pub fn parse_expression(p: &mut Parser) -> OptRes<Expression> {
    if parse_return(p)?.is_some() {
        let expression = parse_control_flow_expression(p)?
            .ok_or_else(|| p.error(ErroVar::ReturnMissingExpression))?;
        Ok(Some(Expression::Return(Box::new(expression))))
    } else if let Some(variable_declaration) = parse_variable_declaration(p)? {
        let expression = parse_control_flow_expression(p)?
            .ok_or_else(|| p.error(ErroVar::VariableDeclarationMissingExpression))?;
        Ok(Some(Expression::Declaration {
            identifier: variable_declaration.identifier,
            data_type: variable_declaration.data_type,
            expression: Box::new(expression),
        }))
    } else {
        parse_control_flow_expression(p)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::grammar::{
        code_block::{CodeBlock, Statement},
        conditional::IfElse,
        expressions::{parse_expression, parse_list_expression, IndexOrRange},
        loops::{ForLoop, WhileLoop},
    };

    use super::{super::test_utils::tests::*, BinaryOperator, UnaryOperator};

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
                    Expression::Literal(Literal(Value::Int(30))),
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
                    Expression::Literal(Literal(Value::Int(30))),
                    Expression::Literal(Literal(Value::String("ccc".to_owned())))
                ],
            }
        );

        assert_eq!(warnings.len(), 1);
        assert_eq!(
            warnings[0],
            ParserWarning {
                warning: ParserWarningVariant::ExpectedExpression,
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
                    Expression::Literal(Literal(Value::Int(30))),
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
            Expression::Literal(Literal(Value::Int(5)))
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
            Expression::Literal(Literal(Value::Int(5)))
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
            Expression::Literal(Literal(Value::Int(7)))
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
    fn list() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Operator(Op::OpenSquareBracket)),
                dummy_token(TokenType::Int(5)),
                dummy_token(TokenType::Operator(Op::Split)),
                dummy_token(TokenType::Int(6)),
                dummy_token(TokenType::Operator(Op::CloseSquareBracket)),
            ],
            parse_list_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            Expression::List(vec![
                Expression::Literal(Literal(Value::Int(5))),
                Expression::Literal(Literal(Value::Int(6)))
            ])
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn list_empty() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Operator(Op::OpenSquareBracket)),
                dummy_token(TokenType::Operator(Op::CloseSquareBracket)),
                token(TokenType::Int(5), (2, 3), (2, 4)),
            ],
            parse_list_expression,
        );
        assert_eq!(result.unwrap().unwrap(), Expression::List(vec![]));

        assert!(warnings.is_empty());
    }

    #[test]
    fn list_trailing_comma() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Operator(Op::OpenSquareBracket)),
                dummy_token(TokenType::Int(5)),
                dummy_token(TokenType::Operator(Op::Split)),
                dummy_token(TokenType::Int(6)),
                dummy_token(TokenType::Operator(Op::Split)),
                token(TokenType::Operator(Op::CloseSquareBracket), (5, 6), (5, 7)),
            ],
            parse_list_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            Expression::List(vec![
                Expression::Literal(Literal(Value::Int(5))),
                Expression::Literal(Literal(Value::Int(6)))
            ])
        );

        assert_eq!(warnings.len(), 1);
        assert_eq!(
            warnings[0],
            ParserWarning {
                warning: ParserWarningVariant::ExpectedExpression,
                start: Position::new(5, 6),
                stop: Position::new(5, 7)
            }
        );
    }

    #[test]
    fn list_missing_bracket() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Operator(Op::OpenSquareBracket)),
                dummy_token(TokenType::Int(5)),
                dummy_token(TokenType::Operator(Op::Split)),
                dummy_token(TokenType::Int(6)),
                token(TokenType::Keyword(Kw::Let), (7, 3), (7, 6)),
            ],
            parse_list_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            Expression::List(vec![
                Expression::Literal(Literal(Value::Int(5))),
                Expression::Literal(Literal(Value::Int(6)))
            ])
        );

        assert_eq!(warnings.len(), 1);
        assert_eq!(
            warnings[0],
            ParserWarning {
                warning: ParserWarningVariant::MissingClosingSquareBracket,
                start: Position::new(7, 3),
                stop: Position::new(7, 6)
            }
        );
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
                    Value::Int(1)
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
                    Expression::Literal(Literal(Value::Int(1))),
                    Expression::Literal(Literal(Value::Int(5)))
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
                    Expression::Literal(Literal(Value::Int(1))),
                    Expression::Literal(Literal(Value::Int(5)))
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

    fn unary_op_helper(
        t_operator: TokenType,
        t_literal: TokenType,
        operator: UnaryOperator,
        literal: Literal,
    ) {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(t_operator.clone()),
                dummy_token(t_operator),
                dummy_token(t_literal),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            Expression::UnaryOperation {
                operator,
                expression: Box::new(Expression::UnaryOperation {
                    operator,
                    expression: Box::new(Expression::Literal(literal))
                })
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn unary_algebraic_negation() {
        unary_op_helper(
            TokenType::Operator(Op::Minus),
            TokenType::Float(5.37),
            UnaryOperator::AlgebraicNegation,
            Literal(Value::Float(5.37)),
        )
    }

    #[test]
    fn unary_logical_negation() {
        unary_op_helper(
            TokenType::Operator(Op::ExclamationMark),
            TokenType::Keyword(Kw::True),
            UnaryOperator::LogicalNegation,
            Literal(Value::Bool(true)),
        )
    }

    #[test]
    fn unary_operation_missing_expression() {
        let (result, warnings) = partial_parse(
            vec![
                token(TokenType::Operator(Op::ExclamationMark), (5, 7), (5, 8)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::UnaryOperatorMissingExpression,
                pos: Position::new(5, 8),
            }
        );

        assert!(warnings.is_empty());
    }

    fn binary_op_helper(
        t_operator: TokenType,
        t_literal: TokenType,
        operator: BinaryOperator,
        literal: Literal,
    ) {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(t_literal.clone()),
                dummy_token(t_operator.clone()),
                dummy_token(t_literal.clone()),
                dummy_token(t_operator),
                dummy_token(t_literal),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            Expression::BinaryOperation {
                operator,
                lhs: Box::new(Expression::BinaryOperation {
                    operator,
                    lhs: Box::new(Expression::Literal(literal.clone())),
                    rhs: Box::new(Expression::Literal(literal.clone()))
                }),
                rhs: Box::new(Expression::Literal(literal))
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn binary_multiplication() {
        binary_op_helper(
            TokenType::Operator(Op::Asterisk),
            TokenType::Float(2.71),
            BinaryOperator::Multiplication,
            Literal(Value::Float(2.71)),
        )
    }

    #[test]
    fn binary_division() {
        binary_op_helper(
            TokenType::Operator(Op::Slash),
            TokenType::Float(2.71),
            BinaryOperator::Division,
            Literal(Value::Float(2.71)),
        )
    }

    #[test]
    fn binary_modulo() {
        binary_op_helper(
            TokenType::Operator(Op::Modulo),
            TokenType::Int(5),
            BinaryOperator::Modulo,
            Literal(Value::Int(5)),
        )
    }

    #[test]
    fn binary_addition() {
        binary_op_helper(
            TokenType::Operator(Op::Plus),
            TokenType::Float(2.71),
            BinaryOperator::Addition,
            Literal(Value::Float(2.71)),
        )
    }

    #[test]
    fn binary_subtraction() {
        binary_op_helper(
            TokenType::Operator(Op::Minus),
            TokenType::Float(2.71),
            BinaryOperator::Subtraction,
            Literal(Value::Float(2.71)),
        )
    }

    #[test]
    fn binary_equal() {
        binary_op_helper(
            TokenType::Operator(Op::DoubleEqual),
            TokenType::String("a".to_owned()),
            BinaryOperator::Equal,
            Literal(Value::String("a".to_owned())),
        )
    }

    #[test]
    fn binary_unequal() {
        binary_op_helper(
            TokenType::Operator(Op::Unequal),
            TokenType::Float(2.71),
            BinaryOperator::Unequal,
            Literal(Value::Float(2.71)),
        )
    }

    #[test]
    fn binary_lesser() {
        binary_op_helper(
            TokenType::Operator(Op::Lesser),
            TokenType::Float(2.71),
            BinaryOperator::Lesser,
            Literal(Value::Float(2.71)),
        )
    }

    #[test]
    fn binary_lesser_equal() {
        binary_op_helper(
            TokenType::Operator(Op::LesserEqual),
            TokenType::Float(2.71),
            BinaryOperator::LesserEqual,
            Literal(Value::Float(2.71)),
        )
    }

    #[test]
    fn binary_greater() {
        binary_op_helper(
            TokenType::Operator(Op::Greater),
            TokenType::Float(2.71),
            BinaryOperator::Greater,
            Literal(Value::Float(2.71)),
        )
    }

    #[test]
    fn binary_greater_equal() {
        binary_op_helper(
            TokenType::Operator(Op::GreaterEqual),
            TokenType::Float(2.71),
            BinaryOperator::GreaterEqual,
            Literal(Value::Float(2.71)),
        )
    }

    #[test]
    fn binary_and() {
        binary_op_helper(
            TokenType::Operator(Op::And),
            TokenType::Keyword(Kw::True),
            BinaryOperator::And,
            Literal(Value::Bool(true)),
        )
    }

    #[test]
    fn binary_or() {
        binary_op_helper(
            TokenType::Operator(Op::Or),
            TokenType::Keyword(Kw::False),
            BinaryOperator::Or,
            Literal(Value::Bool(false)),
        )
    }

    #[test]
    fn binary_operation_missing_rhs() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Int(45)),
                token(TokenType::Operator(Op::Plus), (5, 9), (5, 10)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::BinaryOperatorMissingRHS,
                pos: Position::new(5, 10),
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn assignment() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Operator(Op::Equal)),
                dummy_token(TokenType::Int(69)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            Expression::Assignment {
                identifier: Box::new(Expression::Identifier("a".to_owned())),
                expression: Box::new(Expression::Literal(Literal(Value::Int(69))))
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn assignment_missing_expression() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Identifier("a".to_owned())),
                token(TokenType::Operator(Op::Equal), (2, 6), (2, 7)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::AssignmentMissingExpression,
                pos: Position::new(2, 7),
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn declaration() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::Let)),
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Operator(Op::Colon)),
                dummy_token(TokenType::Keyword(Kw::Int)),
                dummy_token(TokenType::Operator(Op::Equal)),
                dummy_token(TokenType::Int(1337)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            Expression::Declaration {
                identifier: "a".to_owned(),
                data_type: grammar::DataType::Integer,
                expression: Box::new(Expression::Literal(Literal(Value::Int(1337))))
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn declaration_missing_type_separator() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::Let)),
                dummy_token(TokenType::Identifier("a".to_owned())),
                token(TokenType::Keyword(Kw::Int), (2, 2), (2, 5)),
                dummy_token(TokenType::Operator(Op::Equal)),
                dummy_token(TokenType::Int(42)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            Expression::Declaration {
                identifier: "a".to_owned(),
                data_type: grammar::DataType::Integer,
                expression: Box::new(Expression::Literal(Literal(Value::Int(42))))
            }
        );

        assert_eq!(warnings.len(), 1);
        assert_eq!(
            warnings[0],
            ParserWarning {
                warning: ParserWarningVariant::VariableDeclarationMissingTypeSeparator,
                start: Position::new(2, 2),
                stop: Position::new(2, 5)
            }
        );
    }

    #[test]
    fn declaration_missing_equals_sign() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::Let)),
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Operator(Op::Colon)),
                dummy_token(TokenType::Keyword(Kw::Int)),
                token(TokenType::Int(2137), (4, 13), (4, 17)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            Expression::Declaration {
                identifier: "a".to_owned(),
                data_type: grammar::DataType::Integer,
                expression: Box::new(Expression::Literal(Literal(Value::Int(2137))))
            }
        );

        assert_eq!(warnings.len(), 1);
        assert_eq!(
            warnings[0],
            ParserWarning {
                warning: ParserWarningVariant::VariableDeclarationMissingEqualsSign,
                start: Position::new(4, 13),
                stop: Position::new(4, 17)
            }
        );
    }

    #[test]
    fn declaration_missing_type() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::Let)),
                dummy_token(TokenType::Identifier("a".to_owned())),
                token(TokenType::Operator(Op::Colon), (5, 7), (5, 8)),
                dummy_token(TokenType::Operator(Op::Equal)),
                dummy_token(TokenType::Int(1337)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::VariableDeclarationMissingType,
                pos: Position::new(5, 8),
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn declaration_missing_identifier() {
        let (result, warnings) = partial_parse(
            vec![
                token(TokenType::Keyword(Kw::Let), (2, 2), (2, 5)),
                dummy_token(TokenType::Operator(Op::Colon)),
                dummy_token(TokenType::Keyword(Kw::Int)),
                dummy_token(TokenType::Operator(Op::Equal)),
                dummy_token(TokenType::Int(1337)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::VariableDeclarationMissingIdentifier,
                pos: Position::new(2, 5),
            }
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn declaration_missing_expression() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Keyword(Kw::Let)),
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Operator(Op::Colon)),
                dummy_token(TokenType::Keyword(Kw::Int)),
                token(TokenType::Operator(Op::Equal), (5, 17), (5, 18)),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap_err(),
            ParserError {
                error: ParserErrorVariant::VariableDeclarationMissingExpression,
                pos: Position::new(5, 18),
            }
        );

        assert!(warnings.is_empty());
    }

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
                    Statement::Expression(Expression::Literal(Literal(Value::Int(5)))),
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
            Expression::Return(Box::new(Expression::Literal(Literal(Value::Int(0)))))
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
                error: ParserErrorVariant::VariableDeclarationMissingIdentifier,
                pos: Position::new(5, 9),
            }
        );

        assert!(warnings.is_empty());
    }
}
