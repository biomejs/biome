use std::{borrow::Cow, ops::Neg};

use biome_js_syntax::numbers::canonicalize_js_bigint_literal;
use biome_rowan::Text;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct BooleanLiteral(bool);

impl BooleanLiteral {
    #[inline]
    pub fn as_bool(&self) -> bool {
        self.0
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "true" => Some(Self(true)),
            "false" => Some(Self(false)),
            _ => None,
        }
    }
}

impl From<bool> for BooleanLiteral {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct NumberLiteral(Text);

impl NumberLiteral {
    pub fn new(text: Text) -> Self {
        Self(text)
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.text()
    }

    #[inline]
    pub fn text(&self) -> &Text {
        &self.0
    }

    pub fn to_f64(&self) -> Option<f64> {
        let parse = |text: &str| {
            // Remove numeric separators first.
            // Most of numbers have no separators.
            // Thus we check if the number contains any separator before calling `replace` that allocates a String.
            let s = if text.contains('_') {
                Cow::Owned(text.replace('_', ""))
            } else {
                Cow::Borrowed(text)
            };

            match s.get(..2) {
                Some("0b" | "0B") => Some(u64::from_str_radix(&s[2..], 2).ok()? as f64),
                Some("0o" | "0O") => Some(u64::from_str_radix(&s[2..], 8).ok()? as f64),
                Some("0x" | "0X") => Some(u64::from_str_radix(&s[2..], 16).ok()? as f64),
                Some("Na") => (s == "NaN").then_some(f64::NAN),
                Some(prefix)
                    if prefix.starts_with('0')
                        && !prefix.ends_with(['e', 'E'])
                        && !s[1..].contains(['8', '9']) =>
                {
                    Some(u64::from_str_radix(&s[1..], 8).ok()? as f64)
                }
                _ => s.parse().ok(),
            }
        };

        let text = self.as_str();
        if let Some(text) = text.strip_prefix('-') {
            parse(text).map(Neg::neg)
        } else {
            parse(text)
        }
    }
}

/// Converts a number to a string the way JavaScript does, as in `` `${value}` ``.
pub(crate) fn number_to_js_string(value: f64) -> String {
    if value == 0.0 {
        return "0".to_string();
    }
    if !value.is_finite() || (1e-6..1e21).contains(&value.abs()) {
        return match value {
            f64::INFINITY => "Infinity".to_string(),
            f64::NEG_INFINITY => "-Infinity".to_string(),
            value if value.is_nan() => "NaN".to_string(),
            // Both languages print the shortest digits that round-trip, and
            // JavaScript uses positional notation in this range.
            value => value.to_string(),
        };
    }
    let formatted = format!("{value:e}");
    match formatted.split_once('e') {
        Some((mantissa, exponent)) if !exponent.starts_with('-') => {
            format!("{mantissa}e+{exponent}")
        }
        _ => formatted,
    }
}

/// Converts a string to a number the way JavaScript's `Number(text)` does.
///
/// Returns `None` where JavaScript returns `NaN`.
pub(crate) fn js_string_to_number(text: &str) -> Option<f64> {
    let text = text.trim_matches(|c: char| (c.is_whitespace() && c != '\u{85}') || c == '\u{feff}');
    if text.is_empty() {
        return Some(0.0);
    }
    let radix = match text.get(..2) {
        Some("0x" | "0X") => Some(16),
        Some("0o" | "0O") => Some(8),
        Some("0b" | "0B") => Some(2),
        _ => None,
    };
    if let Some(radix) = radix {
        let digits = &text[2..];
        if digits.is_empty() {
            return None;
        }
        return digits.chars().try_fold(0.0, |value, c| {
            Some(value * f64::from(radix) + f64::from(c.to_digit(radix)?))
        });
    }
    match text {
        "Infinity" | "+Infinity" => Some(f64::INFINITY),
        "-Infinity" => Some(f64::NEG_INFINITY),
        // Rust also accepts names such as `inf` and `NaN`, which JavaScript
        // doesn't.
        text if text.bytes().all(|byte| {
            byte.is_ascii_digit() || matches!(byte, b'.' | b'e' | b'E' | b'+' | b'-')
        }) =>
        {
            text.parse().ok()
        }
        _ => None,
    }
}

/// Returns whether `text` matches `` `${number}` ``, which accepts any text
/// that JavaScript converts to a finite number.
pub(crate) fn is_number_string(text: &str) -> bool {
    !text.is_empty() && js_string_to_number(text).is_some_and(f64::is_finite)
}

/// Returns the number that `text` spells, if converting that number back to
/// a string produces `text` again.
pub(crate) fn number_from_canonical_string(text: &str) -> Option<f64> {
    js_string_to_number(text)
        .filter(|value| value.is_finite() && number_to_js_string(*value) == text)
}

/// Returns `text` as the canonical text of a bigint literal, such as `10n`
/// for `0xa`, if `text` matches `` `${bigint}` ``.
///
/// Like `BigInt(text)`, this accepts an optional minus sign followed by a
/// decimal, binary, octal, or hexadecimal integer without separators.
pub(crate) fn bigint_literal_from_string(text: &str) -> Option<String> {
    if text.contains('_') {
        return None;
    }
    canonicalize_js_bigint_literal(&format!("{text}n")).map(Cow::into_owned)
}

/// Returns the text JavaScript produces when converting a bigint literal
/// such as `0x10n` to a string, as in `` `${value}` ``.
pub(crate) fn bigint_literal_to_js_string(literal: &str) -> Option<String> {
    let mut canonical = canonicalize_js_bigint_literal(literal)?.into_owned();
    canonical.pop();
    Some(canonical)
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct RegexpLiteral {
    /// The pattern to match against.
    ///
    /// Examples:
    ///
    /// ```js
    /// const regex1 = /hello/;
    /// //     Pattern: ^^^^^
    ///
    /// const regex2 = new RegExp("world", "i");
    /// //                Pattern: ^^^^^
    /// ```
    pub pattern: Text,

    /// The flags carried by the expression.
    ///
    /// Examples:
    ///
    /// ```js
    /// const regex1 = /hello/i;
    /// //              Flag: ^
    ///
    /// const regex2 = new RegExp("world", "g");
    /// //                            Flag: ^
    /// ```
    pub flags: Text,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct StringLiteral(Text);

impl StringLiteral {
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.text()
    }
}

// TODO: parse escape sequences
impl AsRef<Text> for StringLiteral {
    fn as_ref(&self) -> &Text {
        &self.0
    }
}

impl From<Text> for StringLiteral {
    fn from(value: Text) -> Self {
        Self(value)
    }
}

impl From<String> for StringLiteral {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}

impl From<&str> for StringLiteral {
    fn from(value: &str) -> Self {
        Self(value.to_string().into())
    }
}

impl From<StringLiteral> for Text {
    fn from(value: StringLiteral) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_boolean() {
        assert_eq!(BooleanLiteral::parse("true"), Some(BooleanLiteral(true)));
        assert_eq!(BooleanLiteral::parse("false"), Some(BooleanLiteral(false)));
        assert_eq!(BooleanLiteral::parse("truthy"), None);
        assert_eq!(BooleanLiteral::parse("falsehood"), None);
    }

    #[test]
    fn parse_number_basic() {
        assert_eq!(
            NumberLiteral(Text::new_static("1234567890")).to_f64(),
            Some(1_234_567_890.0)
        );
        assert_eq!(NumberLiteral(Text::new_static("42")).to_f64(), Some(42.0));
        assert_eq!(NumberLiteral(Text::new_static("0")).to_f64(), Some(0.0));
    }

    #[test]
    fn parse_number_legacy_octal() {
        assert_eq!(
            NumberLiteral(Text::new_static("0888")).to_f64(),
            Some(888.0)
        );
        assert_eq!(
            NumberLiteral(Text::new_static("0788")).to_f64(),
            Some(788.0)
        );
        assert_eq!(
            NumberLiteral(Text::new_static("0777")).to_f64(),
            Some(511.0)
        );
    }

    #[test]
    fn parse_number_exponential() {
        assert_eq!(NumberLiteral(Text::new_static("0e-5")).to_f64(), Some(0.0));
        assert_eq!(NumberLiteral(Text::new_static("0e+5")).to_f64(), Some(0.0));
        assert_eq!(NumberLiteral(Text::new_static("5e1")).to_f64(), Some(50.0));
        assert_eq!(
            NumberLiteral(Text::new_static("175e-2")).to_f64(),
            Some(1.75)
        );
        assert_eq!(
            NumberLiteral(Text::new_static("1e3")).to_f64(),
            Some(1000.0)
        );
        assert_eq!(
            NumberLiteral(Text::new_static("1e-3")).to_f64(),
            Some(0.001)
        );
        assert_eq!(
            NumberLiteral(Text::new_static("1E3")).to_f64(),
            Some(1000.0)
        );
    }

    #[test]
    fn parse_number_binary() {
        assert_eq!(
            NumberLiteral(Text::new_static("0b10000000000000000000000000000000")).to_f64(),
            Some(2_147_483_648.0)
        );
        assert_eq!(
            NumberLiteral(Text::new_static("0b01111111100000000000000000000000")).to_f64(),
            Some(2_139_095_040.0)
        );
        assert_eq!(
            NumberLiteral(Text::new_static("0B00000000011111111111111111111111")).to_f64(),
            Some(8_388_607.0)
        );
    }

    #[test]
    fn parse_number_octal() {
        assert_eq!(
            NumberLiteral(Text::new_static("0O755")).to_f64(),
            Some(493.0)
        );
        assert_eq!(
            NumberLiteral(Text::new_static("0o644")).to_f64(),
            Some(420.0)
        );
    }

    #[test]
    fn parse_number_hexadecimal() {
        // TODO: This is bigger than u64 and can't parse it via u64::from_str_radix.
        //       However it's a valid number literal in ECMAScript, though it will be truncated
        //       to the precision of f64.
        // assert_eq!(
        //     NumberLiteral(Text::new_static("0xFFFFFFFFFFFFFFFFF"),
        //     Some(295147905179352830000.0)
        // );
        assert_eq!(
            NumberLiteral(Text::new_static("0x123456789ABCDEF")).to_f64(),
            Some(81_985_529_216_486_900.0)
        );
        assert_eq!(NumberLiteral(Text::new_static("0XA")).to_f64(), Some(10.0));
    }

    #[test]
    fn number_to_js_string_matches_javascript() {
        assert_eq!(number_to_js_string(16.0), "16");
        assert_eq!(number_to_js_string(-0.0), "0");
        assert_eq!(number_to_js_string(0.1), "0.1");
        assert_eq!(number_to_js_string(1e-6), "0.000001");
        assert_eq!(number_to_js_string(1e-7), "1e-7");
        assert_eq!(number_to_js_string(1.5e21), "1.5e+21");
        assert_eq!(number_to_js_string(1e20), "100000000000000000000");
    }

    #[test]
    fn number_strings_match_javascript() {
        for text in ["1.5", " 1 ", "0x10", "-1e3", ".5", "5.", " "] {
            assert!(is_number_string(text), "{text}");
        }
        for text in ["", "1_0", "Infinity", "inf", "NaN", "-0x10", "0x", "1e"] {
            assert!(!is_number_string(text), "{text}");
        }
        assert_eq!(number_from_canonical_string("42"), Some(42.0));
        assert_eq!(number_from_canonical_string("042"), None);
        assert_eq!(number_from_canonical_string("1e3"), None);
        assert_eq!(number_from_canonical_string("-0"), None);
    }

    #[test]
    fn bigint_strings_match_javascript() {
        assert_eq!(bigint_literal_from_string("10").as_deref(), Some("10n"));
        assert_eq!(bigint_literal_from_string("-0x10").as_deref(), Some("-16n"));
        for text in ["", "01", "1.0", "1_0", "+1", "1n"] {
            assert_eq!(bigint_literal_from_string(text), None, "{text}");
        }
        assert_eq!(bigint_literal_to_js_string("0x10n").as_deref(), Some("16"));
        assert_eq!(bigint_literal_to_js_string("-0n").as_deref(), Some("0"));
    }

    #[test]
    fn parse_number_separators() {
        assert_eq!(
            NumberLiteral(Text::new_static("1_000_000_000_000")).to_f64(),
            Some(1_000_000_000_000.0)
        );
        assert_eq!(
            NumberLiteral(Text::new_static("1_050.95")).to_f64(),
            Some(1050.95)
        );
        assert_eq!(
            NumberLiteral(Text::new_static("0b1010_0001_1000_0101")).to_f64(),
            Some(41349.0)
        );
        assert_eq!(
            NumberLiteral(Text::new_static("0o2_2_5_6")).to_f64(),
            Some(1198.0)
        );
        assert_eq!(
            NumberLiteral(Text::new_static("0xA0_B0_C0")).to_f64(),
            Some(10_531_008.0)
        );
    }
}
