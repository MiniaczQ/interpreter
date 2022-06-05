use crate::{
    interpreter::{context::Context, ExecutionError},
    parser::grammar::Value,
};

use super::{
    super::utility::*, identifier::parse_identifier_expression, parse_expression, Evaluable,
    Expression,
};

/// Function call expression
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct FunctionCallExpr {
    identifier: Box<Expression>,
    arguments: Vec<Expression>,
}

impl FunctionCallExpr {
    pub fn new(identifier: Expression, arguments: Vec<Expression>) -> Self {
        Self {
            identifier: Box::new(identifier),
            arguments,
        }
    }
}

impl From<FunctionCallExpr> for Expression {
    fn from(e: FunctionCallExpr) -> Self {
        Expression::FunctionCall(e)
    }
}

impl Evaluable for FunctionCallExpr {
    fn eval(&self, ctx: &dyn Context) -> Result<Value, ExecutionError> {
        todo!()
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
pub fn parse_identifier_or_function_call_expression(p: &mut Parser) -> OptRes<Expression> {
    if let Some(mut expression) = parse_identifier_expression(p)? {
        if let Some(arguments) = parse_function_call(p)? {
            expression = FunctionCallExpr::new(expression, arguments).into();
        }
        return Ok(Some(expression));
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use crate::parser::grammar::expressions::{
        function_call::FunctionCallExpr, identifier::IdentifierExpr, parse_expression,
    };

    use super::super::super::test_utils::tests::*;

    #[test]
    fn parse() {
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
            FunctionCallExpr::new(
                IdentifierExpr::new("a".to_owned()).into(),
                vec![
                    Value::Int(30).into(),
                    Value::String("ccc".to_owned()).into()
                ],
            )
            .into()
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn parse_empty() {
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
            FunctionCallExpr::new(IdentifierExpr::new("a".to_owned()).into(), vec![],).into()
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn parse_trailing_comma() {
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
            FunctionCallExpr::new(
                IdentifierExpr::new("a".to_owned()).into(),
                vec![
                    Value::Int(30).into(),
                    Value::String("ccc".to_owned()).into()
                ],
            )
            .into()
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
    fn parse_missing_closing_bracket() {
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
            FunctionCallExpr::new(
                IdentifierExpr::new("a".to_owned()).into(),
                vec![
                    Value::Int(30).into(),
                    Value::String("ccc".to_owned()).into()
                ],
            )
            .into()
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
}
