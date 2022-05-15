//! Collection of aliases and re-exports for parsing

use crate::parser::{token::TokenType, ParserError};

// Re-export traits
pub use crate::{
    parser::{ErrorHandler, ExtScannable},
    scannable::Scannable,
};

// Re-export types with short aliases
pub use crate::parser::{
    keywords::Keyword as Kw, operators::Operator as Op, Parser, ParserErrorVariant as ErroVar,
    ParserWarningVariant as WarnVar,
};

// Other re-exports
pub use serde::{Deserialize, Serialize};

// Named types
pub type OptRes<T> = Result<Option<T>, ParserError>;
pub type Res<T> = Result<T, ParserError>;

// Helper methods
pub trait ParsingHelper: ExtScannable {
    /// Whether the current parser token is a specific keyword
    fn keyword(&mut self, kw: Kw) -> Res<bool> {
        if let TokenType::Keyword(actual) = self.token()?.token_type {
            if kw == actual {
                self.pop();
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Whether the current parser token is a specific operator
    fn operator(&mut self, op: Op) -> Res<bool> {
        if let TokenType::Operator(actual) = self.token()?.token_type {
            if op == actual {
                self.pop();
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Whether the current parser token is an identifier
    fn identifier(&mut self) -> OptRes<String> {
        if let TokenType::Identifier(id) = self.token()?.token_type {
            self.pop();
            return Ok(Some(id));
        }
        Ok(None)
    }

    /// Whether the current parser token is a string
    fn string(&mut self) -> OptRes<String> {
        if let TokenType::String(s) = self.token()?.token_type {
            self.pop();
            return Ok(Some(s));
        }
        Ok(None)
    }

    /// Whether the current parser token is an integer
    fn integer(&mut self) -> OptRes<i64> {
        if let TokenType::Int(v) = self.token()?.token_type {
            self.pop();
            return Ok(Some(v));
        }
        Ok(None)
    }

    /// Whether the current parser token is a float
    fn float(&mut self) -> OptRes<f64> {
        if let TokenType::Float(v) = self.token()?.token_type {
            self.pop();
            return Ok(Some(v));
        }
        Ok(None)
    }

    /// Whether parser ran out of tokens
    fn has_tokens(&mut self) -> bool {
        self.token().is_ok()
    }
}

impl<T: ExtScannable> ParsingHelper for T {}

/// Result extension for simpler parser control flow.
pub trait ExtResult<T> {
    /// In simple terms, if self if:
    /// - an error      - returns the error
    /// - is none       - returns fallback result
    /// - is a value    - returns the value
    fn alt(self, fallback: impl FnOnce() -> OptRes<T>) -> OptRes<T>;
}

impl<T> ExtResult<T> for OptRes<T> {
    fn alt(self, fallback: impl FnOnce() -> OptRes<T>) -> OptRes<T> {
        if let Ok(opt) = self {
            if opt.is_some() {
                Ok(opt)
            } else {
                fallback()
            }
        } else {
            self
        }
    }
}
