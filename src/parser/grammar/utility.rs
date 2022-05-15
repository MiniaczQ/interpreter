// Re-export traits
pub use crate::{
    parser::{ErrorHandler, ExtScannable},
    scannable::Scannable,
};

// Re-export types with short aliases
pub use crate::parser::{
    keywords::Keyword as Kw, operators::Operator as Op, token::TokenType as Type, Parser,
    ParserError as Erro, ParserErrorVariant as ErroVar, ParserWarningVariant as WarnVar,
};

// Named types
pub type OptRes<T> = Result<Option<T>, Erro>;
pub type Res<T> = Result<T, Erro>;

// Helper methods
pub trait ParsingHelper: ExtScannable {
    /// Whether the current parser token is a specific keyword
    fn parse_kw(&mut self, kw1: Kw) -> Res<bool> {
        if let Type::Keyword(kw2) = self.token()?.token_type {
            if kw1 == kw2 {
                self.pop();
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Whether the current parser token is a specific operator
    fn parse_op(&mut self, op1: Op) -> Res<bool> {
        if let Type::Operator(op2) = self.token()?.token_type {
            if op1 == op2 {
                self.pop();
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Whether the current parser token is an identifier
    fn parse_id(&mut self) -> OptRes<String> {
        if let Type::Identifier(id) = self.token()?.token_type {
            self.pop();
            return Ok(Some(id));
        }
        Ok(None)
    }

    /// Whether the current parser token is a string
    fn parse_str(&mut self) -> OptRes<String> {
        if let Type::String(s) = self.token()?.token_type {
            self.pop();
            return Ok(Some(s));
        }
        Ok(None)
    }

    /// Whether the current parser token is an integer
    fn parse_int(&mut self) -> OptRes<i64> {
        if let Type::Int(v) = self.token()?.token_type {
            self.pop();
            return Ok(Some(v));
        }
        Ok(None)
    }

    /// Whether the current parser token is a float
    fn parse_float(&mut self) -> OptRes<f64> {
        if let Type::Float(v) = self.token()?.token_type {
            self.pop();
            return Ok(Some(v));
        }
        Ok(None)
    }
}

impl<T: ExtScannable> ParsingHelper for T {}
