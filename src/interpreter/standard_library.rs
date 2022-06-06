use std::{
    cell::RefCell,
    io::{Stdout, Write},
};

use crate::parser::grammar::Value;

use super::{callable::Callable, context::Context, ExecutionError, ExecutionErrorVariant};

/// Possible write buffers for print function
#[allow(dead_code)]
pub enum PrintOuts {
    Std(Stdout),
    Vec(Vec<u8>),
}

impl Write for PrintOuts {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            PrintOuts::Std(s) => s.write(buf),
            PrintOuts::Vec(s) => s.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            PrintOuts::Std(s) => s.flush(),
            PrintOuts::Vec(s) => s.flush(),
        }
    }
}

/// Prints all of the provided arguments to the generic `Write`r.
/// Arguments can be of any type and count.
///
/// Never fails.
pub struct Print(pub RefCell<PrintOuts>);

impl Callable for Print {
    fn call(&self, _ctx: &dyn Context, args: Vec<Value>) -> Result<Value, ExecutionError> {
        if !args.is_empty() {
            for arg in args {
                writeln!(self.0.borrow_mut(), "{arg}").ok();
            }
        } else {
            writeln!(self.0.borrow_mut()).ok();
        }
        Ok(Value::None)
    }
}

/// Attempts to turn provided argument into an integer.
/// Accepts exactly 1 argument.
///
/// Fails for `none`, `bool`, `list` and non-integer-like `string`s or wrong amount of arguments.
pub struct CastInt;

impl Callable for CastInt {
    fn call(&self, _ctx: &dyn Context, args: Vec<Value>) -> Result<Value, ExecutionError> {
        if args.len() != 1 {
            return Err(ExecutionError::new(
                ExecutionErrorVariant::InvalidArgumentCount,
            ));
        }
        match &args[0] {
            Value::Int(v) => Ok(Value::Int(*v)),
            Value::Float(v) => Ok(Value::Int(*v as i64)),
            Value::String(v) => v
                .parse::<i64>()
                .map(Value::Int)
                .map_err(|_| ExecutionError::new(ExecutionErrorVariant::CastFailed)),
            _ => Err(ExecutionError::new(ExecutionErrorVariant::InvalidType)),
        }
    }
}

/// Attempts to turn provided argument into a float.
/// Accepts exactly 1 argument.
///
/// Fails for `none`, `bool`, `list` and non-float-like `string`s or wrong amount of arguments.
pub struct CastFloat;

impl Callable for CastFloat {
    fn call(&self, _ctx: &dyn Context, args: Vec<Value>) -> Result<Value, ExecutionError> {
        if args.len() != 1 {
            return Err(ExecutionError::new(
                ExecutionErrorVariant::InvalidArgumentCount,
            ));
        }
        match &args[0] {
            Value::Int(v) => Ok(Value::Float(*v as f64)),
            Value::Float(v) => Ok(Value::Float(*v)),
            Value::String(v) => v
                .parse::<f64>()
                .map(Value::Float)
                .map_err(|_| ExecutionError::new(ExecutionErrorVariant::CastFailed)),
            _ => Err(ExecutionError::new(ExecutionErrorVariant::InvalidType)),
        }
    }
}

/// Attempts to turn provided argument into a string.
/// Accepts exactly 1 argument.
///
/// Fails when wrong amount of arguments.
pub struct CastString;

impl Callable for CastString {
    fn call(&self, _ctx: &dyn Context, args: Vec<Value>) -> Result<Value, ExecutionError> {
        if args.len() != 1 {
            return Err(ExecutionError::new(
                ExecutionErrorVariant::InvalidArgumentCount,
            ));
        }
        match &args[0] {
            Value::String(v) => Ok(Value::String(v.clone())),
            v => Ok(Value::String(format!("{v}"))),
        }
    }
}

/// Attempts to turn provided argument into a bool.
/// Accepts exactly 1 argument.
///
/// Fails for `none`, `list` and non-bool-like `string`s or wrong amount of arguments.
pub struct CastBool;

impl Callable for CastBool {
    fn call(&self, _ctx: &dyn Context, args: Vec<Value>) -> Result<Value, ExecutionError> {
        if args.len() != 1 {
            return Err(ExecutionError::new(
                ExecutionErrorVariant::InvalidArgumentCount,
            ));
        }
        match &args[0] {
            Value::Int(v) => Ok(Value::Bool(*v != 0)),
            Value::Float(v) => Ok(Value::Bool(*v != 0.0)),
            Value::String(v) => match v.as_str() {
                "true" => Ok(Value::Bool(true)),
                "false" => Ok(Value::Bool(false)),
                _ => Err(ExecutionError::new(ExecutionErrorVariant::CastFailed)),
            },
            Value::Bool(v) => Ok(Value::Bool(*v)),
            _ => Err(ExecutionError::new(ExecutionErrorVariant::InvalidType)),
        }
    }
}

/// Returns type of the provided argument.
/// Accepts exactly 1 argument.
///
/// Fails when wrong amount of arguments.
pub struct GetType;

impl Callable for GetType {
    fn call(&self, _ctx: &dyn Context, args: Vec<Value>) -> Result<Value, ExecutionError> {
        if args.len() != 1 {
            return Err(ExecutionError::new(
                ExecutionErrorVariant::InvalidArgumentCount,
            ));
        }
        match &args[0] {
            Value::Int(_) => Ok(Value::String("int".to_owned())),
            Value::Float(_) => Ok(Value::String("float".to_owned())),
            Value::Bool(_) => Ok(Value::String("bool".to_owned())),
            Value::String(_) => Ok(Value::String("string".to_owned())),
            Value::List(_) => Ok(Value::String("list".to_owned())),
            Value::None => Ok(Value::String("none".to_owned())),
        }
    }
}

/// Returns the length of a `list` or `string`.
///
/// Fails when argument is not a `list` or `string`, or wrong amount of arguments.
pub struct ListLength;

impl Callable for ListLength {
    fn call(&self, _ctx: &dyn Context, args: Vec<Value>) -> Result<Value, ExecutionError> {
        if args.len() != 1 {
            return Err(ExecutionError::new(
                ExecutionErrorVariant::InvalidArgumentCount,
            ));
        }
        if let Value::List(list) = &args[0] {
            Ok(Value::Int(list.len() as i64))
        } else if let Value::String(string) = &args[0] {
            Ok(Value::Int(string.len() as i64))
        } else {
            Err(ExecutionError::new(ExecutionErrorVariant::InvalidType))
        }
    }
}

/// Returns provided list with appended element.
///
/// Fails when first argument is not a list or wrong amount of arguments.
pub struct ListPush;

impl Callable for ListPush {
    fn call(&self, _ctx: &dyn Context, args: Vec<Value>) -> Result<Value, ExecutionError> {
        if args.len() != 2 {
            return Err(ExecutionError::new(
                ExecutionErrorVariant::InvalidArgumentCount,
            ));
        }
        let mut args = args.into_iter();
        if let Value::List(mut list) = args.next().unwrap() {
            let v = args.next().unwrap();
            list.push(v);
            Ok(Value::List(list))
        } else {
            Err(ExecutionError::new(ExecutionErrorVariant::InvalidType))
        }
    }
}

/// Standard library context.
///
/// Provides standard functions without the ability to store variables
pub struct StandardCtx {
    pub std_print: Print,
    pub std_cast_int: CastInt,
    pub std_cast_float: CastFloat,
    pub std_cast_string: CastString,
    pub std_cast_bool: CastBool,
    pub std_type: GetType,
    pub std_length: ListLength,
    pub std_push: ListPush,
}

impl StandardCtx {
    pub fn new(writeable: PrintOuts) -> Self {
        Self {
            std_print: Print(RefCell::new(writeable)),
            std_cast_int: CastInt,
            std_cast_float: CastFloat,
            std_cast_string: CastString,
            std_cast_bool: CastBool,
            std_type: GetType,
            std_length: ListLength,
            std_push: ListPush,
        }
    }
}

impl Context for StandardCtx {
    fn get_variable(&self, _id: &str) -> Result<Value, ExecutionError> {
        Err(ExecutionError::new(
            ExecutionErrorVariant::VariableDoesNotExist,
        ))
    }

    fn set_variable(&self, _id: &str, _value: Value) -> Result<(), ExecutionError> {
        Err(ExecutionError::new(
            ExecutionErrorVariant::VariableDoesNotExist,
        ))
    }

    fn new_variable(&self, _id: &str, _value: Value) -> Result<(), ExecutionError> {
        Err(ExecutionError::new(
            ExecutionErrorVariant::VariableAlreadyExists,
        ))
    }

    fn ret(&self, _value: Value) {
        unreachable!()
    }

    fn is_ret(&self) -> bool {
        unreachable!()
    }

    fn call_function(&self, id: &str, args: Vec<Value>) -> Result<Value, ExecutionError> {
        match id {
            "print" => self.std_print.call(self, args),
            "cast_int" => self.std_cast_int.call(self, args),
            "cast_float" => self.std_cast_float.call(self, args),
            "cast_string" => self.std_cast_string.call(self, args),
            "cast_bool" => self.std_cast_bool.call(self, args),
            "type" => self.std_type.call(self, args),
            "length" => self.std_length.call(self, args),
            "push" => self.std_push.call(self, args),
            _ => Err(ExecutionError::new(
                ExecutionErrorVariant::FunctionDoesNotExist,
            )),
        }
    }

    fn name(&self) -> String {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::{
        interpreter::{
            callable::Callable,
            standard_library::{
                CastFloat, CastInt, CastString, GetType, ListLength, ListPush, PrintOuts,
            },
            test_utils::tests::TestCtx,
            ExecutionErrorVariant,
        },
        parser::grammar::Value,
    };

    use super::Print;

    #[test]
    fn print_ok() {
        let print_func = Print(RefCell::new(PrintOuts::Vec(Vec::new())));
        let ctx = TestCtx::new();
        print_func
            .call(
                &ctx,
                vec![Value::Int(3), Value::String("abc".to_owned()), Value::None],
            )
            .unwrap();
        print_func.call(&ctx, vec![]).unwrap();
        if let PrintOuts::Vec(buffer) = print_func.0.replace(PrintOuts::Vec(vec![])) {
            assert_eq!(std::str::from_utf8(&buffer).unwrap(), "3\nabc\nNone\n\n");
        }
    }

    #[test]
    fn cast_int_ok() {
        let cast_int = CastInt;
        let ctx = TestCtx::new();
        assert_eq!(
            cast_int.call(&ctx, vec![Value::Int(8)]).unwrap(),
            Value::Int(8)
        );
        assert_eq!(
            cast_int.call(&ctx, vec![Value::Float(8.0)]).unwrap(),
            Value::Int(8)
        );
        assert_eq!(
            cast_int.call(&ctx, vec![Value::Float(8.9)]).unwrap(),
            Value::Int(8)
        );
        assert_eq!(
            cast_int.call(&ctx, vec![Value::Float(-8.9)]).unwrap(),
            Value::Int(-8)
        );
        assert_eq!(
            cast_int
                .call(&ctx, vec![Value::String("8".to_owned())])
                .unwrap(),
            Value::Int(8)
        );
    }

    #[test]
    fn cast_int_fail() {
        let cast_int = CastInt;
        let ctx = TestCtx::new();
        assert_eq!(
            cast_int
                .call(&ctx, vec![Value::String("avasd".to_owned())])
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::CastFailed
        );
        assert_eq!(
            cast_int
                .call(&ctx, vec![Value::Bool(true)])
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::InvalidType
        );
        assert_eq!(
            cast_int.call(&ctx, vec![Value::None]).unwrap_err().variant,
            ExecutionErrorVariant::InvalidType
        );
        assert_eq!(
            cast_int
                .call(&ctx, vec![Value::List(vec![])])
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::InvalidType
        );
        assert_eq!(
            cast_int
                .call(&ctx, vec![Value::Int(8), Value::Int(8)])
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::InvalidArgumentCount
        );
        assert_eq!(
            cast_int.call(&ctx, vec![]).unwrap_err().variant,
            ExecutionErrorVariant::InvalidArgumentCount
        );
    }

    #[test]
    fn cast_float_ok() {
        let cast_float = CastFloat;
        let ctx = TestCtx::new();
        assert_eq!(
            cast_float.call(&ctx, vec![Value::Int(8)]).unwrap(),
            Value::Float(8.0)
        );
        assert_eq!(
            cast_float.call(&ctx, vec![Value::Float(8.0)]).unwrap(),
            Value::Float(8.0)
        );
        assert_eq!(
            cast_float.call(&ctx, vec![Value::Float(8.9)]).unwrap(),
            Value::Float(8.9)
        );
        assert_eq!(
            cast_float.call(&ctx, vec![Value::Float(-8.9)]).unwrap(),
            Value::Float(-8.9)
        );
        assert_eq!(
            cast_float
                .call(&ctx, vec![Value::String("8".to_owned())])
                .unwrap(),
            Value::Float(8.0)
        );
        assert_eq!(
            cast_float
                .call(&ctx, vec![Value::String("8.9".to_owned())])
                .unwrap(),
            Value::Float(8.9)
        );
    }

    #[test]
    fn cast_float_fail() {
        let cast_float = CastFloat;
        let ctx = TestCtx::new();
        assert_eq!(
            cast_float
                .call(&ctx, vec![Value::String("abasdf".to_owned())])
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::CastFailed
        );
        assert_eq!(
            cast_float
                .call(&ctx, vec![Value::Bool(true)])
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::InvalidType
        );
        assert_eq!(
            cast_float
                .call(&ctx, vec![Value::None])
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::InvalidType
        );
        assert_eq!(
            cast_float
                .call(&ctx, vec![Value::List(vec![])])
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::InvalidType
        );
        assert_eq!(
            cast_float
                .call(&ctx, vec![Value::Float(8.0), Value::Float(8.0)])
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::InvalidArgumentCount
        );
        assert_eq!(
            cast_float.call(&ctx, vec![]).unwrap_err().variant,
            ExecutionErrorVariant::InvalidArgumentCount
        );
    }

    #[test]
    fn cast_string_ok() {
        let cast_string = CastString;
        let ctx = TestCtx::new();
        assert_eq!(
            cast_string.call(&ctx, vec![Value::Int(8)]).unwrap(),
            Value::String("8".to_owned())
        );
        assert_eq!(
            cast_string.call(&ctx, vec![Value::Float(8.5)]).unwrap(),
            Value::String("8.5".to_owned())
        );
        assert_eq!(
            cast_string
                .call(&ctx, vec![Value::String("8".to_owned())])
                .unwrap(),
            Value::String("8".to_owned())
        );
        assert_eq!(
            cast_string.call(&ctx, vec![Value::Bool(true)]).unwrap(),
            Value::String("true".to_owned())
        );
        assert_eq!(
            cast_string.call(&ctx, vec![Value::Bool(false)]).unwrap(),
            Value::String("false".to_owned())
        );
        assert_eq!(
            cast_string.call(&ctx, vec![Value::None]).unwrap(),
            Value::String("None".to_owned())
        );
        assert_eq!(
            cast_string
                .call(
                    &ctx,
                    vec![Value::List(vec![Value::Int(8), Value::Bool(true)])]
                )
                .unwrap(),
            Value::String("[8, true]".to_owned())
        );
    }

    #[test]
    fn cast_string_fail() {
        let cast_string = CastString;
        let ctx = TestCtx::new();
        assert_eq!(
            cast_string.call(&ctx, vec![]).unwrap_err().variant,
            ExecutionErrorVariant::InvalidArgumentCount
        );
        assert_eq!(
            cast_string
                .call(&ctx, vec![Value::Float(8.5), Value::Float(8.5)])
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::InvalidArgumentCount
        );
    }

    #[test]
    fn get_type_ok() {
        let get_type = GetType;
        let ctx = TestCtx::new();
        assert_eq!(
            get_type.call(&ctx, vec![Value::Int(8)]).unwrap(),
            Value::String("int".to_owned())
        );
        assert_eq!(
            get_type.call(&ctx, vec![Value::Float(8.5)]).unwrap(),
            Value::String("float".to_owned())
        );
        assert_eq!(
            get_type
                .call(&ctx, vec![Value::String("8".to_owned())])
                .unwrap(),
            Value::String("string".to_owned())
        );
        assert_eq!(
            get_type.call(&ctx, vec![Value::Bool(true)]).unwrap(),
            Value::String("bool".to_owned())
        );
        assert_eq!(
            get_type.call(&ctx, vec![Value::None]).unwrap(),
            Value::String("none".to_owned())
        );
        assert_eq!(
            get_type
                .call(
                    &ctx,
                    vec![Value::List(vec![Value::Int(8), Value::Bool(true)])]
                )
                .unwrap(),
            Value::String("list".to_owned())
        );
    }

    #[test]
    fn get_type_fail() {
        let get_type = GetType;
        let ctx = TestCtx::new();
        assert_eq!(
            get_type.call(&ctx, vec![]).unwrap_err().variant,
            ExecutionErrorVariant::InvalidArgumentCount
        );
        assert_eq!(
            get_type
                .call(&ctx, vec![Value::Float(8.5), Value::Float(8.5)])
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::InvalidArgumentCount
        );
    }

    #[test]
    fn list_len_ok() {
        let length = ListLength;
        let ctx = TestCtx::new();
        assert_eq!(
            length.call(&ctx, vec![Value::List(vec![])]).unwrap(),
            Value::Int(0)
        );
        assert_eq!(
            length
                .call(&ctx, vec![Value::List(vec![Value::Int(8)])])
                .unwrap(),
            Value::Int(1)
        );
        assert_eq!(
            length
                .call(&ctx, vec![Value::List(vec![Value::Int(8), Value::None])])
                .unwrap(),
            Value::Int(2)
        );
        assert_eq!(
            length
                .call(&ctx, vec![Value::String("".to_owned())])
                .unwrap(),
            Value::Int(0)
        );
        assert_eq!(
            length
                .call(&ctx, vec![Value::String("a".to_owned())])
                .unwrap(),
            Value::Int(1)
        );
        assert_eq!(
            length
                .call(&ctx, vec![Value::String("ab".to_owned())])
                .unwrap(),
            Value::Int(2)
        );
    }

    #[test]
    fn list_length_fail() {
        let length = ListLength;
        let ctx = TestCtx::new();
        assert_eq!(
            length.call(&ctx, vec![]).unwrap_err().variant,
            ExecutionErrorVariant::InvalidArgumentCount
        );
        assert_eq!(
            length.call(&ctx, vec![Value::Int(0)]).unwrap_err().variant,
            ExecutionErrorVariant::InvalidType
        );
        assert_eq!(
            length
                .call(&ctx, vec![Value::Float(1.0)])
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::InvalidType
        );
        assert_eq!(
            length
                .call(&ctx, vec![Value::Bool(true)])
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::InvalidType
        );
        assert_eq!(
            length.call(&ctx, vec![Value::None]).unwrap_err().variant,
            ExecutionErrorVariant::InvalidType
        );
        assert_eq!(
            length
                .call(&ctx, vec![Value::Int(8), Value::None])
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::InvalidArgumentCount
        );
    }

    #[test]
    fn list_push_ok() {
        let push = ListPush;
        let ctx = TestCtx::new();
        assert_eq!(
            push.call(&ctx, vec![Value::List(vec![]), Value::Int(8)])
                .unwrap(),
            Value::List(vec![Value::Int(8)])
        );
        assert_eq!(
            push.call(&ctx, vec![Value::List(vec![Value::Int(8)]), Value::None])
                .unwrap(),
            Value::List(vec![Value::Int(8), Value::None])
        );
    }

    #[test]
    fn list_push_fail() {
        let push = ListPush;
        let ctx = TestCtx::new();
        assert_eq!(
            push.call(&ctx, vec![]).unwrap_err().variant,
            ExecutionErrorVariant::InvalidArgumentCount
        );
        assert_eq!(
            push.call(&ctx, vec![Value::Int(0)]).unwrap_err().variant,
            ExecutionErrorVariant::InvalidArgumentCount
        );
        assert_eq!(
            push.call(&ctx, vec![Value::Int(0), Value::Int(0), Value::Int(0)])
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::InvalidArgumentCount
        );
        assert_eq!(
            push.call(&ctx, vec![Value::Int(0), Value::Int(0)])
                .unwrap_err()
                .variant,
            ExecutionErrorVariant::InvalidType
        );
    }
}
