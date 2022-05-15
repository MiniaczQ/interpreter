use crate::{
    parser::{
        keywords::Keyword, operators::Operator, token::TokenType, ErrorHandler, ExtScannable,
        Parser, ParserError, ParserErrorVariant, ParserWarningVariant,
    },
    scannable::Scannable,
};

use super::{
    code_block::{parse_code_block, CodeBlock},
    conditional::{parse_if_else, IfElse},
    literals::{parse_literal, Literal},
    loops::{parse_for_loop, parse_while_loop, ForLoop, WhileLoop},
    types::{parse_type, DataType},
    ExtResult, ParseResult,
};

/// All possible types of expression
#[derive(Debug, Clone)]
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
    If(Box<IfElse>),
    CodeBlock(CodeBlock),
}

/// grouped
///     = OPEN_BRACKET, expression, CLOSE_BRACKET
///     ;
fn parse_bracket_expression(p: &mut Parser) -> ParseResult<Expression> {
    if let TokenType::Operator(Operator::OpenRoundBracket) = p.token()?.token_type {
        p.pop();
        if let Some(expression) = parse_expression(p)? {
            if let TokenType::Operator(Operator::OpenRoundBracket) = p.token()?.token_type {
                p.pop();
            } else {
                p.warn(ParserWarningVariant::MissingClosingRoundBracket);
            }
            Ok(Some(expression))
        } else {
            Err(p.error(ParserErrorVariant::InvalidBracketExpression))
        }
    } else {
        Ok(None)
    }
}

/// IDENTIFIER
fn parse_identifier_expression(p: &mut Parser) -> ParseResult<Expression> {
    if let TokenType::Identifier(identifier) = p.token()?.token_type {
        p.pop();
        Ok(Some(Expression::Identifier(identifier)))
    } else {
        Ok(None)
    }
}

/// constant
fn parse_literal_expression(p: &mut Parser) -> ParseResult<Expression> {
    parse_literal(p).map(|v| v.map(Expression::Literal))
}

/// const_or_identifier_expression
///     = constant | IDENTIFIER | grouped
///     ;
fn parse_constant_or_identifier_or_bracket_expression(p: &mut Parser) -> ParseResult<Expression> {
    parse_literal_expression(p)
        .alt(|| parse_bracket_expression(p))
        .alt(|| parse_identifier_expression(p))
}

/// Two ways of accessing list elements
#[derive(Debug, Clone)]
pub enum IndexOrRange {
    Index(Expression),
    Range(Expression, Expression),
}

/// index_or_range_access
///     = expression, [RANGE, expression]
///     ;
fn parse_index_or_range_access(p: &mut Parser) -> ParseResult<IndexOrRange> {
    if let Some(left_index) = parse_expression(p)? {
        if let TokenType::Operator(Operator::DoubleColon) = p.token()?.token_type {
            p.pop();
            if let Some(right_index) = parse_expression(p)? {
                Ok(Some(IndexOrRange::Range(left_index, right_index)))
            } else {
                Err(p.error(ParserErrorVariant::ListRangeAccessIncomplete))
            }
        } else {
            Ok(Some(IndexOrRange::Index(left_index)))
        }
    } else {
        Ok(None)
    }
}

/// list_access
///     = OPEN_LIST, index_or_range_access, CLOSE_LIST
///     ;
fn parse_list_access(p: &mut Parser) -> ParseResult<IndexOrRange> {
    if let TokenType::Operator(Operator::OpenSquareBracket) = p.token()?.token_type {
        p.pop();
        if let Some(index_or_range) = parse_index_or_range_access(p)? {
            if let TokenType::Operator(Operator::CloseSquareBracket) = p.token()?.token_type {
                p.pop();
            } else {
                p.warn(ParserWarningVariant::MissingClosingSquareBracket);
            }
            Ok(Some(index_or_range))
        } else {
            Err(p.error(ParserErrorVariant::ListAccessEmpty))
        }
    } else {
        Ok(None)
    }
}

/// function_arguments
///     = [expression, {SPLIT, expression}]
///     ;
fn parse_function_arguments(p: &mut Parser) -> Result<Vec<Expression>, ParserError> {
    let mut arguments = vec![];
    if let Some(argument) = parse_expression(p)? {
        arguments.push(argument);
        while let TokenType::Operator(Operator::Split) = p.token()?.token_type {
            p.pop();
            if let Some(argument) = parse_expression(p)? {
                arguments.push(argument);
            } else {
                p.warn(ParserWarningVariant::TrailingComma);
            }
        }
    }
    Ok(arguments)
}

/// function_call
///     = OPEN_BRACKET, function_arguments, CLOSE_BRACKET
///     ;
fn parse_function_call(p: &mut Parser) -> ParseResult<Vec<Expression>> {
    if let TokenType::Operator(Operator::OpenRoundBracket) = p.token()?.token_type {
        p.pop();
        let args = parse_function_arguments(p)?;
        if let TokenType::Operator(Operator::CloseRoundBracket) = p.token()?.token_type {
            p.pop();
        } else {
            p.warn(ParserWarningVariant::MissingClosingRoundBracket);
        }
        Ok(Some(args))
    } else {
        Ok(None)
    }
}

/// function_call_or_list_access_expression
///     = const_or_identifier_expression, (function_call | list_access)
///     ;
fn parse_function_call_or_list_access_expression(p: &mut Parser) -> ParseResult<Expression> {
    if let Some(expression) = parse_constant_or_identifier_or_bracket_expression(p)? {
        if let Some(arguments) = parse_function_call(p)? {
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
        }
    } else {
        Ok(None)
    }
}

/// Algebraic negation and logical negation
#[derive(Debug, Clone)]
pub enum UnaryOperator {
    AlgebraicNegation,
    LogicalNegation,
}

/// unary_operators
///     = OP_NEGATE | OP_MINUS
///     ;
fn parse_unary_operators(p: &mut Parser) -> ParseResult<UnaryOperator> {
    match p.token()?.token_type {
        TokenType::Operator(Operator::Minus) => {
            p.pop();
            Ok(Some(UnaryOperator::AlgebraicNegation))
        }
        TokenType::Operator(Operator::ExclamationMark) => {
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
fn parse_unary_operator_expression(p: &mut Parser) -> ParseResult<Expression> {
    if let Some(operator) = parse_unary_operators(p)? {
        if let Some(expression) = parse_unary_operator_expression(p)? {
            Ok(Some(Expression::UnaryOperation {
                operator,
                expression: Box::new(expression),
            }))
        } else {
            Err(p.error(ParserErrorVariant::UnaryOperatorMissingExpression))
        }
    } else if let Some(expression) = parse_function_call_or_list_access_expression(p)? {
        Ok(Some(expression))
    } else {
        Ok(None)
    }
}

/// Binary operators
#[derive(Debug, Clone)]
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

/// mul_div_operators
///     = OP_MULTIPLICATION | OP_DIVISION | OP_REMAINDER
///     ;
fn parse_mul_div_operators(p: &mut Parser) -> ParseResult<BinaryOperator> {
    match p.token()?.token_type {
        TokenType::Operator(Operator::Asterisk) => {
            p.pop();
            Ok(Some(BinaryOperator::Multiplication))
        }
        TokenType::Operator(Operator::Slash) => {
            p.pop();
            Ok(Some(BinaryOperator::Division))
        }
        TokenType::Operator(Operator::Modulo) => {
            p.pop();
            Ok(Some(BinaryOperator::Modulo))
        }
        _ => Ok(None),
    }
}

/// mul_div_expression
///     = unary_operator_expression, {mul_div_operators, unary_operator_expression}
///     ;
fn parse_mul_div_expression(p: &mut Parser) -> ParseResult<Expression> {
    if let Some(mut lhs) = parse_unary_operator_expression(p)? {
        while let Some(operator) = parse_mul_div_operators(p)? {
            if let Some(rhs) = parse_unary_operator_expression(p)? {
                lhs = Expression::BinaryOperation {
                    operator,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                };
            } else {
                return Err(p.error(ParserErrorVariant::BinaryOperatorMissingRHS));
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
fn parse_add_sub_operators(p: &mut Parser) -> ParseResult<BinaryOperator> {
    match p.token()?.token_type {
        TokenType::Operator(Operator::Plus) => {
            p.pop();
            Ok(Some(BinaryOperator::Addition))
        }
        TokenType::Operator(Operator::Minus) => {
            p.pop();
            Ok(Some(BinaryOperator::Subtraction))
        }
        _ => Ok(None),
    }
}

/// add_sub_expression
///     = mul_div_expression, {add_sub_operators, mul_div_expression}
///     ;
fn parse_add_sub_expression(p: &mut Parser) -> ParseResult<Expression> {
    if let Some(mut lhs) = parse_mul_div_expression(p)? {
        while let Some(operator) = parse_add_sub_operators(p)? {
            if let Some(rhs) = parse_mul_div_expression(p)? {
                lhs = Expression::BinaryOperation {
                    operator,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                };
            } else {
                return Err(p.error(ParserErrorVariant::BinaryOperatorMissingRHS));
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
fn parse_comparison_operators(p: &mut Parser) -> ParseResult<BinaryOperator> {
    match p.token()?.token_type {
        TokenType::Operator(Operator::DoubleEqual) => {
            p.pop();
            Ok(Some(BinaryOperator::Equal))
        }
        TokenType::Operator(Operator::Unequal) => {
            p.pop();
            Ok(Some(BinaryOperator::Unequal))
        }
        TokenType::Operator(Operator::Lesser) => {
            p.pop();
            Ok(Some(BinaryOperator::Lesser))
        }
        TokenType::Operator(Operator::LesserEqual) => {
            p.pop();
            Ok(Some(BinaryOperator::LesserEqual))
        }
        TokenType::Operator(Operator::Greater) => {
            p.pop();
            Ok(Some(BinaryOperator::Greater))
        }
        TokenType::Operator(Operator::GreaterEqual) => {
            p.pop();
            Ok(Some(BinaryOperator::GreaterEqual))
        }
        _ => Ok(None),
    }
}

/// comparison_expression
///     = add_sub_expression, {comparison_operators, add_sub_expression}
///     ;
fn parse_comparison_expression(p: &mut Parser) -> ParseResult<Expression> {
    if let Some(mut lhs) = parse_add_sub_expression(p)? {
        while let Some(operator) = parse_comparison_operators(p)? {
            if let Some(rhs) = parse_add_sub_expression(p)? {
                lhs = Expression::BinaryOperation {
                    operator,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                };
            } else {
                return Err(p.error(ParserErrorVariant::BinaryOperatorMissingRHS));
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
fn parse_logical_conjunction_expression(p: &mut Parser) -> ParseResult<Expression> {
    if let Some(mut lhs) = parse_comparison_expression(p)? {
        while let TokenType::Operator(Operator::And) = p.token()?.token_type {
            p.pop();
            if let Some(rhs) = parse_comparison_expression(p)? {
                lhs = Expression::BinaryOperation {
                    operator: BinaryOperator::And,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                };
            } else {
                return Err(p.error(ParserErrorVariant::BinaryOperatorMissingRHS));
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
fn parse_logical_alternative_expression(p: &mut Parser) -> ParseResult<Expression> {
    if let Some(mut lhs) = parse_logical_conjunction_expression(p)? {
        while let TokenType::Operator(Operator::Or) = p.token()?.token_type {
            p.pop();
            if let Some(rhs) = parse_logical_conjunction_expression(p)? {
                lhs = Expression::BinaryOperation {
                    operator: BinaryOperator::Or,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                };
            } else {
                return Err(p.error(ParserErrorVariant::BinaryOperatorMissingRHS));
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
fn parse_variable_assignment_expression(p: &mut Parser) -> ParseResult<Expression> {
    if let Some(mut lhs) = parse_logical_alternative_expression(p)? {
        if let TokenType::Operator(Operator::Equal) = p.token()?.token_type {
            p.pop();
            if let Some(rhs) = parse_logical_conjunction_expression(p)? {
                lhs = Expression::Assignment {
                    identifier: Box::new(lhs),
                    expression: Box::new(rhs),
                };
            } else {
                return Err(p.error(ParserErrorVariant::AssignmentMissingExpression));
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
fn parse_variable_declaration(p: &mut Parser) -> ParseResult<VariableDeclaration> {
    if let TokenType::Keyword(Keyword::Let) = p.token()?.token_type {
        p.pop();
        if let TokenType::Identifier(identifier) = p.token()?.token_type {
            p.pop();
            if let TokenType::Operator(Operator::Colon) = p.token()?.token_type {
                p.pop();
            } else {
                p.warn(ParserWarningVariant::VariableDeclarationMissingTypeSeparator);
            }
            if let Some(data_type) = parse_type(p)? {
                if let TokenType::Operator(Operator::Equal) = p.token()?.token_type {
                    p.pop();
                } else {
                    p.warn(ParserWarningVariant::VariableDeclarationMissingEqualsSign);
                }
                Ok(Some(VariableDeclaration {
                    identifier,
                    data_type,
                }))
            } else {
                Err(p.error(ParserErrorVariant::VariableDeclarationMissingType))
            }
        } else {
            Err(p.error(ParserErrorVariant::VariableDeclarationMissingIdentifier))
        }
    } else {
        Ok(None)
    }
}

/// for_expression
fn parse_for_expression(p: &mut Parser) -> ParseResult<Expression> {
    parse_for_loop(p).map(|v| v.map(|v| Expression::For(Box::new(v))))
}

/// while_expression
fn parse_while_expression(p: &mut Parser) -> ParseResult<Expression> {
    parse_while_loop(p).map(|v| v.map(|v| Expression::While(Box::new(v))))
}

/// if_expression
fn parse_if_else_expression(p: &mut Parser) -> ParseResult<Expression> {
    parse_if_else(p).map(|v| v.map(|v| Expression::If(Box::new(v))))
}

/// for_expression
fn parse_code_block_expression(p: &mut Parser) -> ParseResult<Expression> {
    parse_code_block(p).map(|v| v.map(Expression::CodeBlock))
}

/// control_flow_expression
///     = variable_assignment_expression
///     | for_expression
///     | while_expression
///     | if_expression
///     | code_block
///     ;
fn parse_control_flow_expression(p: &mut Parser) -> ParseResult<Expression> {
    parse_variable_assignment_expression(p)
        .alt(|| parse_for_expression(p))
        .alt(|| parse_while_expression(p))
        .alt(|| parse_if_else_expression(p))
        .alt(|| parse_code_block_expression(p))
}

/// expression
///     = [KW_RETURN | variable_declaration], control_flow_expression
///     ;
pub fn parse_expression(p: &mut Parser) -> ParseResult<Expression> {
    if let TokenType::Keyword(Keyword::Return) = p.token()?.token_type {
        p.pop();
        if let Some(expression) = parse_control_flow_expression(p)? {
            Ok(Some(Expression::Return(Box::new(expression))))
        } else {
            Err(p.error(ParserErrorVariant::ReturnMissingExpression))
        }
    } else if let Some(variable_declaration) = parse_variable_declaration(p)? {
        if let Some(expression) = parse_control_flow_expression(p)? {
            Ok(Some(Expression::Declaration {
                identifier: variable_declaration.identifier,
                data_type: variable_declaration.data_type,
                expression: Box::new(expression),
            }))
        } else {
            Err(p.error(ParserErrorVariant::VariableDeclarationMissingExpression))
        }
    } else {
        parse_control_flow_expression(p)
    }
}
