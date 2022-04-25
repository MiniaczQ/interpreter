use crate::{
    lexer::lexem::{Lexem, LexemBuilder, LexemType},
    scannable::Scannable,
};

/// Matches an integer or a float constant
pub fn match_numerical(tb: &mut LexemBuilder) -> Option<Lexem> {
    if tb.peek().is_ascii_digit() {
        let mut integer_part: i64 = tb.peek() as i64 - '0' as i64;
        if tb.peek() != '0' {
            tb.pop();
            loop {
                if tb.peek().is_ascii_digit() {
                    if let Some(new_integer_part) = integer_part.checked_mul(10) {
                        integer_part = new_integer_part;
                        integer_part += tb.peek() as i64 - '0' as i64;
                        tb.pop();
                    } else {
                        eprintln!(
                            "Float decimal part too big from {} to {}.",
                            tb.get_start(),
                            tb.get_here()
                        );
                        break;
                    }
                } else if tb.peek() == '_' {
                    tb.pop();
                } else {
                    break;
                }
            }
        } else {
            tb.pop();
        }
        if let Some(token) = match_float(tb, integer_part) {
            Some(token)
        } else {
            tb.bake(LexemType::Int(integer_part))
        }
    } else {
        None
    }
}

/// Matches a float constant
fn match_float(tb: &mut LexemBuilder, integer_part: i64) -> Option<Lexem> {
    if tb.peek() == '.' {
        tb.pop();
        if tb.peek().is_ascii_digit() {
            let mut digits = 1;
            let mut decimal_part: i64 = tb.peek() as i64 - '0' as i64;
            tb.pop();
            loop {
                if tb.peek().is_ascii_digit() {
                    if let Some(new_decimal_part) = decimal_part.checked_mul(10) {
                        decimal_part = new_decimal_part;
                        digits += 1;
                        decimal_part += tb.peek() as i64 - '0' as i64;
                        tb.pop();
                    } else {
                        eprintln!(
                            "Float decimal part too big from {} to {}.",
                            tb.get_start(),
                            tb.get_here()
                        );
                        break;
                    }
                } else if tb.peek() == '_' {
                    tb.pop();
                } else {
                    break;
                }
            }
            tb.bake(LexemType::Float(
                integer_part as f64 + decimal_part as f64 / 10f64.powf(digits as f64),
            ))
        } else {
            tb.bake(LexemType::Float(integer_part as f64))
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::{
        lexem::{Lexem, LexemType},
        matchers::test_utils::{lexem_with, matcher_with},
    };

    use super::match_numerical;

    fn matcher(string: &'static str) -> Option<Lexem> {
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
    fn int_above_limit() {
        assert_eq!(
            matcher("101273576184162375213625468214"),
            int_lexem(1012735761841623752, (1, 1), (1, 20))
        );
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
    fn float_above_limit() {
        assert_eq!(
            matcher("0.101273576184162375213625468214"),
            float_lexem(0.10127357618416238, (1, 1), (1, 22))
        );
    }

    #[test]
    fn empty() {
        assert_eq!(matcher(""), None);
    }
}