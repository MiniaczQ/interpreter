use crate::{
    interpreter::{context::Context, ExecutionError},
    parser::grammar::Value,
};

use super::{
    super::utility::*, binary::parse_logical_alternative_expression, Evaluable, Expression,
};

/// Variable assignment expression
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AssignmentExpr {
    identifier: Box<Expression>,
    expression: Box<Expression>,
}

impl AssignmentExpr {
    pub fn new(identifier: Expression, expression: Expression) -> Self {
        Self {
            identifier: Box::new(identifier),
            expression: Box::new(expression),
        }
    }
}

impl From<AssignmentExpr> for Expression {
    fn from(e: AssignmentExpr) -> Self {
        Expression::Assignment(e)
    }
}

impl Evaluable for AssignmentExpr {
    fn eval(&self, ctx: &dyn Context) -> Result<Value, ExecutionError> {
        todo!()
    }
}

/// variable_assignment_expression
///     = logical_alternative_expression, {ASSIGN, expression}
///     ;
pub fn parse_variable_assignment_expression(p: &mut Parser) -> OptRes<Expression> {
    if let Some(mut lhs) = parse_logical_alternative_expression(p)? {
        while p.operator(Op::Equal)? {
            let rhs = parse_logical_alternative_expression(p)?
                .ok_or_else(|| p.error(ErroVar::AssignmentMissingExpression))?;
            lhs = AssignmentExpr::new(lhs, rhs).into();
        }
        return Ok(Some(lhs));
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use crate::parser::grammar::expressions::{
        assignment::AssignmentExpr, identifier::IdentifierExpr, parse_expression,
    };

    use super::super::super::test_utils::tests::*;

    #[test]
    fn parse() {
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
            AssignmentExpr::new(
                IdentifierExpr::new("a".to_owned()).into(),
                Value::Int(69).into()
            )
            .into()
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn parse_missing_expression() {
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
}
