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

/// Converts a possibly signed JavaScript bigint literal to decimal notation.
///
/// The returned text has no separators or leading zeroes and uses a lowercase
/// `n` suffix. Negative zero is represented as `0n`. Returns `None` if `input`
/// is not a valid decimal, binary, octal, or hexadecimal bigint literal.
pub fn canonicalize_js_bigint_literal(input: &str) -> Option<Cow<'_, str>> {
    const LIMB_BASE: u64 = 1_000_000_000;

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

    let mut limbs = vec![0_u32];
    let mut digit_count = 0;
    let mut first_digit = None;
    let mut previous_was_separator = false;

    for byte in digits.bytes() {
        if byte == b'_' {
            if digit_count == 0 || previous_was_separator {
                return None;
            }
            previous_was_separator = true;
            continue;
        }

        let digit = match byte {
            b'0'..=b'9' => u32::from(byte - b'0'),
            b'a'..=b'f' => u32::from(byte - b'a') + 10,
            b'A'..=b'F' => u32::from(byte - b'A') + 10,
            _ => return None,
        };
        if digit >= radix {
            return None;
        }

        first_digit.get_or_insert(digit);
        digit_count += 1;
        previous_was_separator = false;

        let mut carry = u64::from(digit);
        for limb in &mut limbs {
            let value = u64::from(*limb) * u64::from(radix) + carry;
            *limb = (value % LIMB_BASE) as u32;
            carry = value / LIMB_BASE;
        }
        if carry != 0 {
            limbs.push(carry as u32);
        }
    }

    if digit_count == 0 || previous_was_separator {
        return None;
    }
    if radix == 10 && digit_count > 1 && first_digit == Some(0) {
        return None;
    }

    let is_zero = limbs.len() == 1 && limbs[0] == 0;
    if radix == 10 && !digits.contains('_') && (!is_negative || !is_zero) {
        return Some(Cow::Borrowed(input));
    }

    let mut canonical = String::new();
    if is_negative && !is_zero {
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

#[cfg(test)]
mod tests {
    use super::{canonicalize_js_bigint_literal, split_into_radix_and_number};
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
    fn canonicalizes_bigint_literals() {
        let cases = [
            ("0n", "0n"),
            ("-0n", "0n"),
            ("123456789n", "123456789n"),
            ("-123_456_789n", "-123456789n"),
            ("0b1010_0101n", "165n"),
            ("-0B1010n", "-10n"),
            ("0o777_777n", "262143n"),
            ("-0O10n", "-8n"),
            ("0xdead_beefn", "3735928559n"),
            ("-0XFFn", "-255n"),
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
    fn rejects_invalid_bigint_literals() {
        let cases = [
            "", "n", "-n", "+1n", "1", "1N", "01n", "0_0n", "_1n", "1_n", "1__0n", "0bn", "0b2n",
            "0o8n", "0xgn", "--1n",
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
}
