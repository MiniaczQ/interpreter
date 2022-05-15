//! Collection of aliases and re-exports for parsing

use crate::parser::ParserError;

// Re-export traits
pub use crate::{
    parser::{ErrorHandler, ExtScannable},
    scannable::Scannable,
};

// Re-export types with short aliases
pub use crate::parser::{
    keywords::Keyword as Kw, operators::Operator as Op, token::TokenType as Type, Parser,
    ParserErrorVariant as ErroVar, ParserWarningVariant as WarnVar,
};

// Named types
pub type OptRes<T> = Result<Option<T>, ParserError>;
pub type Res<T> = Result<T, ParserError>;

// Helper methods
pub trait ParsingHelper: ExtScannable {
    /// Whether the current parser token is a specific keyword
    fn keyword(&mut self, kw: Kw) -> Res<bool> {
        if let Type::Keyword(actual) = self.token()?.token_type {
            if kw == actual {
                self.pop();
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Whether the current parser token is a specific operator
    fn operator(&mut self, op: Op) -> Res<bool> {
        if let Type::Operator(actual) = self.token()?.token_type {
            if op == actual {
                self.pop();
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Whether the current parser token is an identifier
    fn identifier(&mut self) -> OptRes<String> {
        if let Type::Identifier(id) = self.token()?.token_type {
            self.pop();
            return Ok(Some(id));
        }
        Ok(None)
    }

    /// Whether the current parser token is a string
    fn string(&mut self) -> OptRes<String> {
        if let Type::String(s) = self.token()?.token_type {
            self.pop();
            return Ok(Some(s));
        }
        Ok(None)
    }

    /// Whether the current parser token is an integer
    fn integer(&mut self) -> OptRes<i64> {
        if let Type::Int(v) = self.token()?.token_type {
            self.pop();
            return Ok(Some(v));
        }
        Ok(None)
    }

    /// Whether the current parser token is a float
    fn float(&mut self) -> OptRes<f64> {
        if let Type::Float(v) = self.token()?.token_type {
            self.pop();
            return Ok(Some(v));
        }
        Ok(None)
    }
}

impl<T: ExtScannable> ParsingHelper for T {}
