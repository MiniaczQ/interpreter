use crate::{
    interpreter::{context::Context, ExecutionError},
    parser::grammar::Value,
};

use super::{super::utility::*, Evaluable, Expression};

/// Identifier expression
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct IdentifierExpr(pub String);

impl IdentifierExpr {
    pub fn new(identifier: String) -> Self {
        Self(identifier)
    }
}

impl From<IdentifierExpr> for Expression {
    fn from(e: IdentifierExpr) -> Self {
        Expression::Identifier(e)
    }
}

impl Evaluable for IdentifierExpr {
    fn eval(&self, ctx: &dyn Context) -> Result<Value, ExecutionError> {
        ctx.get_variable(&self.0)
    }
}

/// IDENTIFIER
pub fn parse_identifier_expression(p: &mut Parser) -> OptRes<Expression> {
    if let Some(identifier) = p.identifier()? {
        return Ok(Some(IdentifierExpr::new(identifier).into()));
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use crate::{
        interpreter::{test_utils::tests::TestCtx, ExecutionErrorVariant},
        parser::grammar::expressions::{identifier::IdentifierExpr, parse_expression},
    };

    use super::super::super::test_utils::tests::*;

    #[test]
    fn parse() {
        let (result, warnings) = partial_parse(
            vec![
                dummy_token(TokenType::Identifier("a".to_owned())),
                dummy_token(TokenType::Operator(Op::Semicolon)),
            ],
            parse_expression,
        );
        assert_eq!(
            result.unwrap().unwrap(),
            IdentifierExpr("a".to_owned()).into()
        );

        assert!(warnings.is_empty());
    }

    #[test]
    fn eval_ok() {
        let ctx = TestCtx::new();
        ctx.variables
            .borrow_mut()
            .insert("a".to_owned(), Value::Int(8));
        assert_eq!(
            IdentifierExpr::new("a".to_owned()).eval(&ctx).unwrap(),
            Value::Int(8)
        );
    }

    #[test]
    fn eval_fail() {
        let ctx = TestCtx::new();
        assert_eq!(
            IdentifierExpr("a".to_owned())
                .eval(&ctx)
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::VariableDoesNotExist
        );
    }
}
