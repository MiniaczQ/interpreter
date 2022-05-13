use crate::{
    lexer::lexem::{Lexem, LexemBuilder, LexemErrorVariant, LexemType},
    scannable::Scannable,
};

/// Turns `0` - `9` characters to their `i64` representation
fn char2num(c: char) -> i64 {
    c as i64 - '0' as i64
}

/// Performs a checked multiplication followed by addition of `a * b + c`
fn checked_mul_add(a: i64, b: i64, c: i64) -> Option<i64> {
    a.checked_mul(b).and_then(|v| v.checked_add(c))
}

/// Matches an integer or a float constant
pub fn match_numerical(lb: &mut LexemBuilder) -> Option<Lexem> {
    if !lb.curr().is_ascii_digit() {
        return None;
    }
    let mut integer_part: i64 = char2num(lb.curr());
    if lb.curr() != '0' {
        lb.pop();
        while lb.curr().is_ascii_digit() || lb.curr() == '_' {
            if lb.curr() == '_' {
                lb.pop();
            } else if let Some(new_integer_part) =
                checked_mul_add(integer_part, 10, char2num(lb.curr()))
            {
                integer_part = new_integer_part;
                lb.pop();
            } else {
                lb.error(LexemErrorVariant::IntegerPartTooBig);
                break;
            }
        }
    } else {
        lb.pop();
    }
    if let Some(token) = match_float(lb, integer_part) {
        Some(token)
    } else {
        lb.bake(LexemType::Int(integer_part))
    }
}

/// Matches a float constant
fn match_float(lb: &mut LexemBuilder, integer_part: i64) -> Option<Lexem> {
    if lb.curr() != '.' {
        return None;
    }
    lb.pop();
    if lb.curr().is_ascii_digit() {
        let mut digits = 1;
        let mut decimal_part: i64 = char2num(lb.curr());
        lb.pop();
        while lb.curr().is_ascii_digit() || lb.curr() == '_' {
            if lb.curr() == '_' {
                lb.pop();
            } else if let Some(new_decimal_part) =
                checked_mul_add(decimal_part, 10, char2num(lb.curr()))
            {
                decimal_part = new_decimal_part;
                digits += 1;
                lb.pop();
            } else {
                lb.error(LexemErrorVariant::DecimalPartTooBig);
                break;
            }
        }
        lb.bake(LexemType::Float(
            integer_part as f64 + decimal_part as f64 / 10f64.powf(digits as f64),
        ))
    } else {
        lb.bake(LexemType::Float(integer_part as f64))
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::{
        lexem::{Lexem, LexemError, LexemErrorVariant, LexemType},
        matchers::test_utils::{lexem_with, matcher_with},
    };

    use super::match_numerical;

    fn matcher(string: &'static str) -> Option<Lexem> {
        let r = matcher_with(match_numerical, string);
        assert!(r.1.is_empty());
        r.0
    }

    fn err_matcher(string: &'static str) -> (Option<Lexem>, Vec<LexemError>) {
        matcher_with(match_numerical, string)
    }

    fn int_lexem(int: i64, start: (usize, usize), stop: (usize, usize)) -> Option<Lexem> {
        lexem_with(LexemType::Int(int), start, stop)
    }

    fn float_lexem(float: f64, start: (usize, usize), stop: (usize, usize)) -> Option<Lexem> {
        lexem_with(LexemType::Float(float), start, stop)
    }

    #[test]
    fn int_simple() {
        assert_eq!(matcher("0"), int_lexem(0, (1, 1), (1, 2)));
        assert_eq!(matcher("0000"), int_lexem(0, (1, 1), (1, 2)));
        assert_eq!(matcher("0_103_0123"), int_lexem(0, (1, 1), (1, 2)));
        assert_eq!(matcher("10"), int_lexem(10, (1, 1), (1, 3)));
        assert_eq!(matcher("1030123"), int_lexem(1030123, (1, 1), (1, 8)));
        assert_eq!(matcher("1_________"), int_lexem(1, (1, 1), (1, 11)));
        assert_eq!(matcher("1_000_000"), int_lexem(1000000, (1, 1), (1, 10)));
    }

    #[test]
    fn int_not() {
        assert_eq!(matcher("_1_0"), None);
        assert_eq!(matcher("-0_103_0123"), None);
    }

    #[test]
    fn int_limit() {
        assert_eq!(
            matcher("9_223_372_036_854_775_807"),
            int_lexem(9223372036854775807, (1, 1), (1, 26))
        );
    }

    #[test]
    fn int_just_above_limit() {
        let (result, errors) = err_matcher("9_223_372_036_854_775_808");
        assert_eq!(result, int_lexem(922337203685477580, (1, 1), (1, 25)));
        assert!(errors[0].variant == LexemErrorVariant::IntegerPartTooBig);
    }

    #[test]
    fn int_above_limit() {
        let (result, errors) = err_matcher("101273576184162375213625468214");
        assert_eq!(result, int_lexem(1012735761841623752, (1, 1), (1, 20)));
        assert!(errors[0].variant == LexemErrorVariant::IntegerPartTooBig);
    }

    #[test]
    fn float_simple() {
        assert_eq!(matcher("0."), float_lexem(0.0, (1, 1), (1, 3)));
        assert_eq!(matcher("0._"), float_lexem(0.0, (1, 1), (1, 3)));
        assert_eq!(matcher("0.0"), float_lexem(0.0, (1, 1), (1, 4)));
        assert_eq!(matcher("0.123"), float_lexem(0.123, (1, 1), (1, 6)));
        assert_eq!(matcher("123.123"), float_lexem(123.123, (1, 1), (1, 8)));
        assert_eq!(matcher("123_.123"), float_lexem(123.123, (1, 1), (1, 9)));
        assert_eq!(
            matcher("1_000_000.000_001"),
            float_lexem(1000000.000001, (1, 1), (1, 18))
        );
        assert_eq!(matcher("1_________.0"), float_lexem(1.0, (1, 1), (1, 13)));
        assert_eq!(
            matcher("1_000_000.00000009"),
            float_lexem(1000000.00000009, (1, 1), (1, 19))
        );
    }

    #[test]
    fn float_not() {
        assert_eq!(matcher("_1.0123"), None);
        assert_eq!(matcher("-0.23141"), None);
    }

    #[test]
    fn float_partial() {
        assert_eq!(matcher("123_._123"), float_lexem(123.0, (1, 1), (1, 6)));
    }

    #[test]
    fn float_limit() {
        assert_eq!(
            matcher("0.9_223_372_036_854_775_807"),
            float_lexem(0.9223372036854776, (1, 1), (1, 28))
        );
        assert_eq!(
            matcher("9_223_372_036_854_775_807.9_223_372_036_854_775_807"),
            float_lexem(9223372036854775807.9223372036854775807, (1, 1), (1, 52))
        );
    }

    #[test]
    fn float_just_above_limit() {
        let (result, errors) = err_matcher("0.9_223_372_036_854_775_808");
        assert_eq!(
            result,
            float_lexem(0.922_337_203_685_477_7, (1, 1), (1, 27))
        );
        assert!(errors[0].variant == LexemErrorVariant::DecimalPartTooBig);
    }

    #[test]
    fn float_above_limit() {
        let (result, errors) = err_matcher("0.101273576184162375213625468214");
        assert_eq!(result, float_lexem(0.10127357618416238, (1, 1), (1, 22)));
        assert!(errors[0].variant == LexemErrorVariant::DecimalPartTooBig);
    }

    #[test]
    fn empty() {
        assert_eq!(matcher(""), None);
    }
}
