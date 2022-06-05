use crate::{
    interpreter::{context::Context, ExecutionError, ExecutionErrorVariant},
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
        if let Expression::Identifier(identifier) = &*self.identifier {
            let arguments: Vec<Value> = self
                .arguments
                .iter()
                .map(|v| v.eval(ctx))
                .collect::<Result<_, ExecutionError>>()?;
            ctx.call_function(&identifier.0, arguments)
        } else {
            Err(ExecutionError::new(
                ExecutionErrorVariant::ExpectedIdentifier,
            ))
        }
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
    use crate::{
        interpreter::{test_utils::tests::TestCtx, ExecutionErrorVariant},
        parser::grammar::{
            expressions::{
                binary::{BinaryExpr, BinaryOperator},
                function_call::FunctionCallExpr,
                identifier::IdentifierExpr,
                parse_expression,
                return_expr::ReturnExpr,
                statement::Statement,
            },
            function::{FunctionDefinition, Parameter},
            DataType,
        },
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

    #[test]
    fn eval_nothing() {
        let mut ctx = TestCtx::new();
        ctx.functions.insert(
            "a".to_owned(),
            Box::new(FunctionDefinition::new(
                "a".to_owned(),
                vec![],
                vec![],
                DataType::None,
            )),
        );
        assert_eq!(
            FunctionCallExpr::new(IdentifierExpr::new("a".to_owned()).into(), vec![])
                .eval(&ctx)
                .unwrap(),
            Value::None
        );
    }

    #[test]
    fn eval_identity_ending_expr() {
        let mut ctx = TestCtx::new();
        ctx.functions.insert(
            "a".to_owned(),
            Box::new(FunctionDefinition::new(
                "a".to_owned(),
                vec![Parameter::new("b".to_owned(), DataType::Integer)],
                vec![
                    Value::Int(7).into(),
                    Statement::Semicolon,
                    IdentifierExpr::new("b".to_owned()).into(),
                ],
                DataType::Integer,
            )),
        );
        assert_eq!(
            FunctionCallExpr::new(
                IdentifierExpr::new("a".to_owned()).into(),
                vec![Value::Int(10).into()]
            )
            .eval(&ctx)
            .unwrap(),
            Value::Int(10)
        );
    }

    #[test]
    fn eval_many_args() {
        let mut ctx = TestCtx::new();
        ctx.functions.insert(
            "a".to_owned(),
            Box::new(FunctionDefinition::new(
                "a".to_owned(),
                vec![
                    Parameter::new("b".to_owned(), DataType::Integer),
                    Parameter::new("c".to_owned(), DataType::Integer),
                ],
                vec![BinaryExpr::new(
                    IdentifierExpr::new("b".to_owned()).into(),
                    BinaryOperator::Addition,
                    IdentifierExpr::new("c".to_owned()).into(),
                )
                .into()],
                DataType::Integer,
            )),
        );
        assert_eq!(
            FunctionCallExpr::new(
                IdentifierExpr::new("a".to_owned()).into(),
                vec![Value::Int(10).into(), Value::Int(10).into()]
            )
            .eval(&ctx)
            .unwrap(),
            Value::Int(20)
        );
    }

    #[test]
    fn eval_wrong_param_type() {
        let mut ctx = TestCtx::new();
        ctx.functions.insert(
            "a".to_owned(),
            Box::new(FunctionDefinition::new(
                "a".to_owned(),
                vec![Parameter::new("b".to_owned(), DataType::Integer)],
                vec![],
                DataType::Integer,
            )),
        );
        assert_eq!(
            FunctionCallExpr::new(
                IdentifierExpr::new("a".to_owned()).into(),
                vec![Value::Float(10.0).into()]
            )
            .eval(&ctx)
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::InvalidType
        );
    }

    #[test]
    fn eval_wrong_param_count() {
        let mut ctx = TestCtx::new();
        ctx.functions.insert(
            "a".to_owned(),
            Box::new(FunctionDefinition::new(
                "a".to_owned(),
                vec![
                    Parameter::new("b".to_owned(), DataType::Integer),
                    Parameter::new("c".to_owned(), DataType::Integer),
                ],
                vec![],
                DataType::Integer,
            )),
        );
        assert_eq!(
            FunctionCallExpr::new(
                IdentifierExpr::new("a".to_owned()).into(),
                vec![Value::Int(10).into()]
            )
            .eval(&ctx)
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::InvalidArgumentCount
        );
        assert_eq!(
            FunctionCallExpr::new(
                IdentifierExpr::new("a".to_owned()).into(),
                vec![
                    Value::Int(10).into(),
                    Value::Int(10).into(),
                    Value::Int(10).into()
                ]
            )
            .eval(&ctx)
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::InvalidArgumentCount
        );
    }

    #[test]
    fn eval_identity_return() {
        let mut ctx = TestCtx::new();
        ctx.functions.insert(
            "a".to_owned(),
            Box::new(FunctionDefinition::new(
                "a".to_owned(),
                vec![Parameter::new("b".to_owned(), DataType::Integer)],
                vec![
                    Value::Int(7).into(),
                    Statement::Semicolon,
                    ReturnExpr::new(IdentifierExpr::new("b".to_owned()).into()).into(),
                    Statement::Semicolon,
                    Value::Int(7).into(),
                ],
                DataType::Integer,
            )),
        );
        assert_eq!(
            FunctionCallExpr::new(
                IdentifierExpr::new("a".to_owned()).into(),
                vec![Value::Int(10).into()]
            )
            .eval(&ctx)
            .unwrap(),
            Value::Int(10)
        );
    }

    #[test]
    fn eval_return_nothing() {
        let mut ctx = TestCtx::new();
        ctx.functions.insert(
            "a".to_owned(),
            Box::new(FunctionDefinition::new(
                "a".to_owned(),
                vec![Parameter::new("b".to_owned(), DataType::Integer)],
                vec![
                    Value::Int(7).into(),
                    Statement::Semicolon,
                    ReturnExpr::empty().into(),
                    Statement::Semicolon,
                    Value::Int(7).into(),
                ],
                DataType::None,
            )),
        );
        assert_eq!(
            FunctionCallExpr::new(
                IdentifierExpr::new("a".to_owned()).into(),
                vec![Value::Int(10).into()]
            )
            .eval(&ctx)
            .unwrap(),
            Value::None
        );
    }

    #[test]
    fn eval_too_many_semicolons() {
        let mut ctx = TestCtx::new();
        ctx.functions.insert(
            "a".to_owned(),
            Box::new(FunctionDefinition::new(
                "a".to_owned(),
                vec![],
                vec![
                    Statement::Semicolon,
                    Statement::Semicolon,
                    Statement::Semicolon,
                ],
                DataType::None,
            )),
        );
        assert_eq!(
            FunctionCallExpr::new(IdentifierExpr::new("a".to_owned()).into(), vec![])
                .eval(&ctx)
                .unwrap(),
            Value::None
        );
    }

    #[test]
    fn eval_too_many_expressions() {
        let mut ctx = TestCtx::new();
        ctx.functions.insert(
            "a".to_owned(),
            Box::new(FunctionDefinition::new(
                "a".to_owned(),
                vec![],
                vec![Value::Int(8).into(), Value::Int(8).into()],
                DataType::None,
            )),
        );
        assert_eq!(
            FunctionCallExpr::new(IdentifierExpr::new("a".to_owned()).into(), vec![])
                .eval(&ctx)
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::ExpectedSemicolon
        );
    }

    #[test]
    fn eval_invalid_type() {
        let mut ctx = TestCtx::new();
        ctx.functions.insert(
            "a".to_owned(),
            Box::new(FunctionDefinition::new(
                "a".to_owned(),
                vec![Parameter::new("b".to_owned(), DataType::Integer)],
                vec![IdentifierExpr::new("b".to_owned()).into()],
                DataType::Float,
            )),
        );
        assert_eq!(
            FunctionCallExpr::new(
                IdentifierExpr::new("a".to_owned()).into(),
                vec![Value::Int(10).into()]
            )
            .eval(&ctx)
            .unwrap_err()
            .variant,
            ExecutionErrorVariant::InvalidType
        );
    }

    #[test]
    fn eval_missing() {
        let ctx = TestCtx::new();
        assert_eq!(
            FunctionCallExpr::new(IdentifierExpr::new("a".to_owned()).into(), vec![])
                .eval(&ctx)
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::FunctionDoesNotExist
        );
    }

    #[test]
    fn eval_wrong_expression() {
        let ctx = TestCtx::new();
        assert_eq!(
            FunctionCallExpr::new(Value::Int(8).into(), vec![])
                .eval(&ctx)
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::ExpectedIdentifier
        );
    }
}
