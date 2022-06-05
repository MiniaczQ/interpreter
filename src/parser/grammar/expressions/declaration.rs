use crate::{
    interpreter::{context::Context, types::validate_type, ExecutionError},
    parser::grammar::{types::parse_type, DataType, Value},
};

use super::{super::utility::*, parse_control_flow_expression, Evaluable, Expression};

/// Variable declaration expression
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DeclarationExpr {
    identifier: String,
    data_type: DataType,
    expression: Box<Expression>,
}

impl DeclarationExpr {
    pub fn new(identifier: String, data_type: DataType, expression: Expression) -> Self {
        Self {
            identifier,
            data_type,
            expression: Box::new(expression),
        }
    }
}

impl From<DeclarationExpr> for Expression {
    fn from(e: DeclarationExpr) -> Self {
        Expression::Declaration(e)
    }
}

impl Evaluable for DeclarationExpr {
    fn eval(&self, ctx: &dyn Context) -> Result<Value, ExecutionError> {
        let value = self.expression.eval(ctx)?;
        validate_type(self.data_type, &value)?;
        ctx.new_variable(&self.identifier, value.clone())?;
        Ok(value)
    }
}

/// variable_declaration
///     = KW_LET, IDENTIFIER, COLON, TYPE_SIGNATURE, type, ASSIGN, control_flow_expression
///     ;
pub fn parse_variable_declaration(p: &mut Parser) -> OptRes<Expression> {
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
    let expression = parse_control_flow_expression(p)?
        .ok_or_else(|| p.error(ErroVar::VariableDeclarationMissingExpression))?;
    Ok(Some(
        DeclarationExpr::new(identifier, data_type, expression).into(),
    ))
}

#[cfg(test)]
mod tests {
    use crate::{
        interpreter::{test_utils::tests::TestCtx, ExecutionErrorVariant},
        parser::grammar::{expressions::parse_expression, DataType},
    };

    use super::{super::super::test_utils::tests::*, DeclarationExpr};

    #[test]
    fn parse() {
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
            DeclarationExpr::new(
                "a".to_owned(),
                grammar::DataType::Integer,
                Value::Int(1337).into()
            )
            .into()
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn parse_missing_type_separator() {
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
            DeclarationExpr::new(
                "a".to_owned(),
                grammar::DataType::Integer,
                Value::Int(42).into()
            )
            .into()
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
    fn parse_missing_equals_sign() {
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
            DeclarationExpr::new(
                "a".to_owned(),
                grammar::DataType::Integer,
                Value::Int(2137).into()
            )
            .into()
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
    fn parse_missing_type() {
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
    fn parse_missing_identifier() {
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
    fn parse_missing_expression() {
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

    fn expr(identifier: &str, data_type: DataType, value: Value) -> DeclarationExpr {
        DeclarationExpr::new(identifier.to_owned(), data_type, value.into())
    }

    #[test]
    fn eval_ok() {
        let ctx = TestCtx::new();
        assert_eq!(
            expr("a", DataType::Integer, Value::Int(8))
                .eval(&ctx)
                .unwrap(),
            Value::Int(8)
        );
        assert_eq!(ctx.variables.borrow_mut().get("a").unwrap(), &Value::Int(8));
    }

    #[test]
    fn eval_fail() {
        let ctx = TestCtx::new();
        ctx.variables
            .borrow_mut()
            .insert("a".to_owned(), Value::Int(8));
        assert_eq!(
            expr("a", DataType::Integer, Value::Int(8))
                .eval(&ctx)
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::VariableAlreadyExists
        );
    }
}
