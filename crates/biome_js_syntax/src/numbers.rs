//! JavaScript numeric literal parsing.

use std::{borrow::Cow, str::FromStr};

/// Split given string into radix and number string.
///
/// It also removes any underscores.
pub fn split_into_radix_and_number(num: &str) -> (u8, Cow<'_, str>) {
    let (radix, raw) = parse_js_number_prefix(num).unwrap_or((10, num));
    let raw = if raw.contains('_') {
        Cow::Owned(raw.replace('_', ""))
    } else {
        Cow::Borrowed(raw)
    };
    (radix, raw)
}

fn parse_js_number_prefix(num: &str) -> Option<(u8, &str)> {
    let mut bytes = num.bytes();
    if bytes.next()? != b'0' {
        return None;
    }
    Some(match bytes.next()? {
        b'x' | b'X' => (16, &num[2..]),
        b'o' | b'O' => (8, &num[2..]),
        b'b' | b'B' => (2, &num[2..]),
        // Legacy octal literals
        b'0'..=b'7' if bytes.all(|b| !matches!(b, b'8' | b'9')) => (8, &num[1..]),
        _ => return None,
    })
}

/// Parse a js number as a string into a number.
pub fn parse_js_number(num: &str) -> Option<f64> {
    let (radix, raw) = split_into_radix_and_number(num);

    if radix == 10 {
        f64::from_str(&raw).ok()
    } else {
        let mut value = 0.0f64;
        let base = radix as f64;
        for c in raw.chars() {
            let digit = c.to_digit(radix as u32)? as f64;
            value = value * base + digit;
        }
        Some(value)
    }
}

/// Parses a numeric literal without rounding intermediate digit prefixes.
///
/// Decimal literals are converted directly to `f64`. Hexadecimal, binary, and
/// octal literals are first read as exact integers, then converted to `f64` once.
/// This matters for large numbers: rounding each prefix while reading its digits
/// can produce a different value from the JavaScript literal.
///
/// For example, these two spellings must produce the same value:
///
/// ```ts
/// 0x1000000000000081 === 1152921504606847232; // true
/// ```
///
/// The input must be numeric literal text already checked by the parser, with
/// any unary sign removed. Numeric separators and legacy octal notation are
/// supported. This function does not validate JavaScript syntax.
///
/// Returns `None` if conversion fails or a non-decimal integer exceeds
/// [`u128::MAX`]. Decimal overflow can return infinity; callers that require a
/// finite value must check the result.
pub fn parse_js_number_with_single_rounding(num: &str) -> Option<f64> {
    let (radix, digits) = split_into_radix_and_number(num);
    if radix == 10 {
        digits.parse::<f64>().ok()
    } else {
        u128::from_str_radix(&digits, u32::from(radix))
            .ok()
            .map(|value| value as f64)
    }
}

const BIGINT_LIMB_BASE: u64 = 1_000_000_000;

fn bigint_digit_value(byte: u8) -> Option<u32> {
    match byte {
        b'0'..=b'9' => Some(u32::from(byte - b'0')),
        b'a'..=b'f' => Some(u32::from(byte - b'a') + 10),
        b'A'..=b'F' => Some(u32::from(byte - b'A') + 10),
        _ => None,
    }
}

fn multiply_add_bigint_chunk(limbs: &mut Vec<u32>, multiplier: u64, mut carry: u64) {
    for limb in &mut *limbs {
        let value = u64::from(*limb) * multiplier + carry;
        *limb = (value % BIGINT_LIMB_BASE) as u32;
        carry = value / BIGINT_LIMB_BASE;
    }
    while carry != 0 {
        limbs.push((carry % BIGINT_LIMB_BASE) as u32);
        carry /= BIGINT_LIMB_BASE;
    }
}

/// Converts a possibly signed JavaScript bigint literal to decimal notation.
///
/// The returned text has no separators or leading zeroes and uses a lowercase
/// `n` suffix. Negative zero is represented as `0n`. The input is returned
/// borrowed when it is already canonical; transformed values are owned.
/// Returns `None` if `input` is not a valid decimal, binary, octal, or
/// hexadecimal bigint literal.
pub fn canonicalize_js_bigint_literal(input: &str) -> Option<Cow<'_, str>> {
    let (is_negative, unsigned) = match input.strip_prefix('-') {
        Some(unsigned) => (true, unsigned),
        None => (false, input),
    };
    let body = unsigned.strip_suffix('n')?;
    let (radix, digits) =
        if let Some(digits) = body.strip_prefix("0b").or_else(|| body.strip_prefix("0B")) {
            (2_u32, digits)
        } else if let Some(digits) = body.strip_prefix("0o").or_else(|| body.strip_prefix("0O")) {
            (8, digits)
        } else if let Some(digits) = body.strip_prefix("0x").or_else(|| body.strip_prefix("0X")) {
            (16, digits)
        } else {
            (10, body)
        };

    let mut digit_count = 0;
    let mut first_digit = None;
    let mut previous_was_separator = false;
    let mut has_separator = false;
    let mut is_zero = true;

    for byte in digits.bytes() {
        if byte == b'_' {
            if digit_count == 0 || previous_was_separator {
                return None;
            }
            previous_was_separator = true;
            has_separator = true;
            continue;
        }

        let digit = bigint_digit_value(byte)?;
        if digit >= radix {
            return None;
        }

        first_digit.get_or_insert(digit);
        digit_count += 1;
        previous_was_separator = false;
        is_zero &= digit == 0;
    }

    if digit_count == 0 || previous_was_separator {
        return None;
    }
    if radix == 10 && digit_count > 1 && first_digit == Some(0) {
        return None;
    }
    if radix == 10 {
        if !has_separator && (!is_negative || !is_zero) {
            return Some(Cow::Borrowed(input));
        }
        let canonical = if is_zero {
            String::from("0n")
        } else {
            input.replace('_', "")
        };
        return Some(Cow::Owned(canonical));
    }
    if is_zero {
        return Some(Cow::Owned(String::from("0n")));
    }

    // Fold this many digits into a single `u64` before each multi-limb
    // multiply, so the big-integer multiplication runs once per chunk instead
    // of once per digit. Each bound keeps `radix.pow(digits_per_chunk)` at most
    // 2^32; combined with limbs below 10^9 that keeps `limb * chunk_multiplier
    // + carry` within `u64`. Raising any bound reintroduces overflow.
    let digits_per_chunk = match radix {
        2 => 32,
        8 => 10,
        16 => 8,
        _ => return None,
    };
    let radix = u64::from(radix);
    let mut limbs = vec![0_u32];
    let mut chunk_value = 0_u64;
    let mut chunk_multiplier = 1_u64;
    let mut chunk_digit_count = 0;

    for byte in digits.bytes() {
        if byte == b'_' {
            continue;
        }
        chunk_value = chunk_value * radix + u64::from(bigint_digit_value(byte)?);
        chunk_multiplier *= radix;
        chunk_digit_count += 1;

        if chunk_digit_count == digits_per_chunk {
            multiply_add_bigint_chunk(&mut limbs, chunk_multiplier, chunk_value);
            chunk_value = 0;
            chunk_multiplier = 1;
            chunk_digit_count = 0;
        }
    }
    if chunk_digit_count != 0 {
        multiply_add_bigint_chunk(&mut limbs, chunk_multiplier, chunk_value);
    }

    let mut canonical = String::with_capacity(limbs.len() * 9 + 2);
    if is_negative {
        canonical.push('-');
    }
    let mut limbs = limbs.iter().rev();
    canonical.push_str(&limbs.next()?.to_string());
    for limb in limbs {
        let limb = limb.to_string();
        canonical.extend(std::iter::repeat_n('0', 9 - limb.len()));
        canonical.push_str(&limb);
    }
    canonical.push('n');
    Some(Cow::Owned(canonical))
}

/// Converts a JavaScript BigInt literal to its canonical property key (decimal representation without trailing 'n').
///
/// Returns `None` if `input` is not a valid BigInt literal.
pub fn canonicalize_js_bigint_property_key(input: &str) -> Option<Cow<'_, str>> {
    let canonical = canonicalize_js_bigint_literal(input)?;
    match canonical {
        Cow::Borrowed(s) => {
            let key = s.strip_suffix('n')?;
            Some(Cow::Borrowed(key))
        }
        Cow::Owned(mut s) => {
            if s.ends_with('n') {
                s.pop();
            }
            Some(Cow::Owned(s))
        }
    }
}

const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;

/// Checks whether the string is already a canonical, simple decimal literal within
/// JavaScript's safe integer range (0 to 2^53 - 1) without decimal points, exponents,
/// separators, alternate radixes, or leading zeroes.
fn is_simple_decimal_literal(input: &str) -> bool {
    if input == "0" {
        return true;
    }
    let bytes = input.as_bytes();
    if bytes.is_empty() || bytes[0] == b'0' {
        return false;
    }
    if !bytes.iter().all(|b| b.is_ascii_digit()) {
        return false;
    }
    if bytes.len() <= 15 {
        return true;
    }
    if bytes.len() == 16 {
        return input.parse::<u64>().is_ok_and(|val| val <= MAX_SAFE_INTEGER);
    }
    false
}

/// Formats a float according to ECMAScript `Number::toString` (ECMA-262 §7.1.17).
///
/// Derives the shortest round-trip significant digits from Rust's Ryu-backed `to_string()`
/// and formats them according to ECMAScript representation rules.
fn format_js_float(val: f64) -> String {
    if val.is_nan() {
        return "NaN".to_string();
    }
    if val == 0.0 {
        return "0".to_string();
    }
    if val < 0.0 {
        return format!("-{}", format_js_float(-val));
    }
    if val.is_infinite() {
        return "Infinity".to_string();
    }

    let s_val = val.to_string();
    let Some((mantissa, exp_str)) = s_val.split_once('e') else {
        return s_val;
    };

    let Ok(exp) = exp_str.parse::<i32>() else {
        return s_val;
    };

    let digits = mantissa.replace('.', "");
    let k = digits.len() as i32;
    let n = exp + 1;

    if k <= n && n <= 21 {
        let mut result = digits;
        result.extend(std::iter::repeat_n('0', (n - k) as usize));
        result
    } else if 0 < n && n <= 21 {
        let n_usize = n as usize;
        let mut result = String::with_capacity(digits.len() + 1);
        result.push_str(&digits[..n_usize]);
        result.push('.');
        result.push_str(&digits[n_usize..]);
        result
    } else if -6 < n && n <= 0 {
        let mut result = String::with_capacity(digits.len() + 2 + (-n) as usize);
        result.push_str("0.");
        result.extend(std::iter::repeat_n('0', (-n) as usize));
        result.push_str(&digits);
        result
    } else {
        let sign = if (n - 1) >= 0 { '+' } else { '-' };
        let exp_abs = (n - 1).unsigned_abs();
        if k == 1 {
            format!("{digits}e{sign}{exp_abs}")
        } else {
            let (first, rest) = digits.split_at(1);
            format!("{first}.{rest}e{sign}{exp_abs}")
        }
    }
}

/// Converts a JavaScript numeric literal to its ECMAScript ToString canonical property name.
///
/// Converts hexadecimal (`0x10`), octal (`0o20`), binary (`0b10`), scientific notation (`1e1`),
/// separators (`1_000`), decimals (`1.0`, `123.00`), and BigInts (`123n`) to their standard
/// decimal property keys.
pub fn canonicalize_js_number_literal(input: &str) -> Option<Cow<'_, str>> {
    if input.ends_with(['n', 'N']) {
        return canonicalize_js_bigint_property_key(input);
    }

    if is_simple_decimal_literal(input) {
        return Some(Cow::Borrowed(input));
    }

    let val = parse_js_number_with_single_rounding(input)?;
    if val.is_nan() {
        return Some(Cow::Borrowed("NaN"));
    }
    if val == 0.0 {
        return Some(Cow::Borrowed("0"));
    }
    if val.is_infinite() {
        return Some(Cow::Borrowed(if val > 0.0 { "Infinity" } else { "-Infinity" }));
    }

    if val.fract() == 0.0 {
        let abs_val = val.abs();
        if abs_val < 1e21 && abs_val <= u128::MAX as f64 {
            let int_val = abs_val as u128;
            if (int_val as f64) == abs_val {
                if val < 0.0 {
                    return Some(Cow::Owned(format!("-{int_val}")));
                } else {
                    return Some(Cow::Owned(int_val.to_string()));
                }
            }
        }
    }

    Some(Cow::Owned(format_js_float(val)))
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::{
        canonicalize_js_bigint_literal, canonicalize_js_bigint_property_key,
        canonicalize_js_number_literal, parse_js_number_with_single_rounding,
        split_into_radix_and_number,
    };
    use biome_js_factory::JsSyntaxTreeBuilder;
    use biome_js_factory::syntax::{JsNumberLiteralExpression, JsSyntaxKind::*};
    use biome_rowan::AstNode;

    fn assert_float(literal: &str, value: f64) {
        let mut tree_builder = JsSyntaxTreeBuilder::new();
        tree_builder.start_node(JS_NUMBER_LITERAL_EXPRESSION);
        tree_builder.token(JS_NUMBER_LITERAL, literal);
        tree_builder.finish_node();

        let node = tree_builder.finish();
        let number_literal = JsNumberLiteralExpression::cast(node).unwrap();
        assert_eq!(number_literal.as_number(), Some(value))
    }

    #[test]
    fn single_rounding_preserves_numeric_values() {
        for (literal, expected) in [
            ("1", 1.0),
            ("1_000", 1000.0),
            ("0.5", 0.5),
            (".5", 0.5),
            ("1e3", 1000.0),
            ("0b101", 5.0),
            ("0o77", 63.0),
            ("077", 63.0),
            ("0xFF", 255.0),
            ("0x20000000000001", 9_007_199_254_740_992.0),
            ("0x1000000000000081", ((1_u64 << 60) + 256) as f64),
        ] {
            assert_eq!(
                parse_js_number_with_single_rounding(literal),
                Some(expected),
                "{literal}"
            );
        }
    }

    #[test]
    fn single_rounding_preserves_conversion_limits() {
        assert_eq!(
            parse_js_number_with_single_rounding("0xffffffffffffffffffffffffffffffff"),
            Some(u128::MAX as f64)
        );
        for literal in [
            format!("0x1{}", "0".repeat(32)),
            format!("0b1{}", "0".repeat(128)),
            format!("0o4{}", "0".repeat(42)),
        ] {
            assert_eq!(parse_js_number_with_single_rounding(&literal), None);
        }
        assert_eq!(
            parse_js_number_with_single_rounding("1e400"),
            Some(f64::INFINITY)
        );
        assert_eq!(parse_js_number_with_single_rounding(""), None);
    }

    #[test]
    fn canonicalizes_bigint_literals() {
        let cases = [
            ("0n", "0n"),
            ("-0n", "0n"),
            ("123456789n", "123456789n"),
            ("-123_456_789n", "-123456789n"),
            ("0b1010_0101n", "165n"),
            ("-0B1010n", "-10n"),
            ("-0b0000n", "0n"),
            ("0b100000000000000000000000000000000n", "4294967296n"),
            ("0o777_777n", "262143n"),
            ("-0O10n", "-8n"),
            ("0o10000000000n", "1073741824n"),
            ("0xdead_beefn", "3735928559n"),
            ("-0XFFn", "-255n"),
            ("0x100000000n", "4294967296n"),
            (
                "0xffffffffffffffffffffffffffffffffn",
                "340282366920938463463374607431768211455n",
            ),
        ];

        for (input, expected) in cases {
            assert_eq!(
                canonicalize_js_bigint_literal(input).as_deref(),
                Some(expected)
            );
        }
    }

    #[test]
    fn canonical_bigint_literal_ownership() {
        let input = String::from("123456789n");
        assert!(matches!(
            canonicalize_js_bigint_literal(&input),
            Some(Cow::Borrowed(canonical)) if std::ptr::eq(canonical, input.as_str())
        ));
        assert!(matches!(
            canonicalize_js_bigint_literal("123_456_789n"),
            Some(Cow::Owned(canonical)) if canonical == "123456789n"
        ));
        assert!(matches!(
            canonicalize_js_bigint_literal("-0n"),
            Some(Cow::Owned(canonical)) if canonical == "0n"
        ));
    }

    #[test]
    fn rejects_invalid_bigint_literals() {
        let cases = [
            "", "n", "-n", "+1n", "1", "1N", "01n", "0_0n", "_1n", "1_n", "1__0n", "0bn", "0b_1n",
            "0b1_n", "0b1__0n", "0b2n", "0o8n", "0xgn", "--1n",
        ];

        for input in cases {
            assert_eq!(canonicalize_js_bigint_literal(input), None, "{input}");
        }
    }

    #[test]
    fn base_10_float() {
        assert_float("1234", 1234.0);
        assert_float("0", 0.0);
        assert_float("9e999", f64::INFINITY);
        assert_float("9e-999", 0.0);
    }

    #[test]
    fn base_16_float() {
        assert_float("0xFF", 255.0);
        assert_float("0XFF", 255.0);
        assert_float("0x0", 0.0);
        assert_float("0xABC", 2748.0);
        assert_float("0XABC", 2748.0);
        // 2^53 + 1
        assert_float("0x20000000000001", 9_007_199_254_740_992.0);
        // 2^1024 (Overflow)
        assert_float(
            "0x10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
            f64::INFINITY,
        );
    }

    #[test]
    fn base_2_float() {
        assert_float("0b0000", 0.0);
        assert_float("0B0000", 0.0);
        assert_float("0b11111111", 255.0);
        assert_float("0B11111111", 255.0);
    }

    #[test]
    fn base_8_float() {
        assert_float("0o77", 63.0);
        assert_float("0O77", 63.0);
        assert_float("0o0", 0.0);
        assert_float("0O0", 0.0);
    }

    #[test]
    fn base_8_legacy_float() {
        assert_float("051", 41.0);
        assert_float("058", 58.0);
    }

    fn assert_split(raw: &str, expected_radix: u8, expected_num: &str) {
        let (radix, num) = split_into_radix_and_number(raw);
        assert_eq!(radix, expected_radix);
        assert_eq!(num, expected_num);
    }

    #[test]
    fn split_hex() {
        assert_split("0x12", 16, "12");
        assert_split("0X12", 16, "12");
        assert_split("0x1_2", 16, "12");
        assert_split("0X1_2", 16, "12");
    }

    #[test]
    fn split_binary() {
        assert_split("0b01", 2, "01");
        assert_split("0b01", 2, "01");
        assert_split("0b0_1", 2, "01");
        assert_split("0b0_1", 2, "01");
    }

    #[test]
    fn split_octal() {
        assert_split("0o12", 8, "12");
        assert_split("0o12", 8, "12");
        assert_split("0o1_2", 8, "12");
        assert_split("0o1_2", 8, "12");
    }

    #[test]
    fn split_legacy_octal() {
        assert_split("012", 8, "12");
        assert_split("012", 8, "12");
        assert_split("01_2", 8, "12");
        assert_split("01_2", 8, "12");
    }

    #[test]
    fn split_legacy_decimal() {
        assert_split("1234", 10, "1234");
        assert_split("1234", 10, "1234");
        assert_split("12_34", 10, "1234");
        assert_split("12_34", 10, "1234");
    }

    #[test]
    fn canonicalizes_number_literals() {
        let cases = [
            ("0", "0"),
            ("1", "1"),
            ("42", "42"),
            ("0x1", "1"),
            ("0x10", "16"),
            ("0XFF", "255"),
            ("0b101", "5"),
            ("0B10", "2"),
            ("0o101", "65"),
            ("0O77", "63"),
            ("077", "63"),
            ("1.0", "1"),
            ("1.", "1"),
            ("100.00", "100"),
            ("123.00", "123"),
            ("1e1", "10"),
            ("1e0", "1"),
            ("1e2", "100"),
            ("1_000", "1000"),
            ("0x1_0", "16"),
            ("0.5", "0.5"),
            (".5", "0.5"),
            ("123.45", "123.45"),
            ("1e21", "1e+21"),
            ("0.0000123456789", "0.0000123456789"),
            ("0.00009999999", "0.00009999999"),
            ("0.0001", "0.0001"),
            ("9007199254740992", "9007199254740992"),
            ("9007199254740993", "9007199254740992"),
            ("123n", "123"),
            ("0x10n", "16"),
            ("0b101n", "5"),
            ("0o101n", "65"),
            ("1_000n", "1000"),
            ("0n", "0"),
            ("-0n", "0"),
        ];

        for (input, expected) in cases {
            assert_eq!(
                canonicalize_js_number_literal(input).as_deref(),
                Some(expected),
                "failed for input: {input}"
            );
        }
    }

    #[test]
    fn canonicalizes_bigint_property_keys() {
        let cases = [
            ("0n", "0"),
            ("-0n", "0"),
            ("123n", "123"),
            ("0x10n", "16"),
            ("0b101n", "5"),
            ("0o101n", "65"),
            ("1_000n", "1000"),
        ];

        for (input, expected) in cases {
            assert_eq!(
                canonicalize_js_bigint_property_key(input).as_deref(),
                Some(expected),
                "failed for input: {input}"
            );
        }
    }
}

