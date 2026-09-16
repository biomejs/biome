use std::{borrow::Cow, ops::Neg};

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
                        && s[1..].chars().all(|digit| ('0'..='7').contains(&digit)) =>
                {
                    u64::from_str_radix(&s[1..], 8)
                        .ok()
                        .map(|value| value as f64)
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

    /// Returns the ECMAScript property-key spelling for this numeric literal.
    pub fn to_property_key(&self) -> Option<String> {
        let number = self.to_f64()?;
        let mut buffer = ryu_js::Buffer::new();
        Some(buffer.format(number).to_string())
    }
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

    /// Returns the semantic value represented by the raw string content.
    pub fn decoded(&self) -> Option<Text> {
        decode_js_string_content(&self.0)
    }
}

/// Decodes raw, quote-free JavaScript string content into its semantic value.
pub fn decode_js_string_content(value: &Text) -> Option<Text> {
    if !value.contains('\\') {
        return Some(value.clone());
    }
    biome_js_syntax::try_unescape_js_string_content(value.text())
}

/// Encodes semantic string content as quote-free JavaScript string content.
pub fn encode_js_string_content(value: &str) -> Text {
    let encoded = serde_json::to_string(value).expect("serializing a string cannot fail");
    let encoded = encoded
        .strip_prefix('"')
        .and_then(|encoded| encoded.strip_suffix('"'))
        .expect("serde_json serializes strings with surrounding quotes");
    let encoded = encoded
        .replace('\u{2028}', "\\u2028")
        .replace('\u{2029}', "\\u2029");
    Text::new_owned(encoded.into_boxed_str())
}

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
    fn parse_number_fractional_values() {
        assert_eq!(NumberLiteral(Text::new_static("0.5")).to_f64(), Some(0.5));
        assert_eq!(NumberLiteral(Text::new_static("-0.5")).to_f64(), Some(-0.5));
        assert_eq!(
            NumberLiteral(Text::new_static("0.125")).to_f64(),
            Some(0.125)
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

    #[test]
    fn number_property_keys_use_ecmascript_spelling() {
        let cases = [
            ("1", "1"),
            ("0x1", "1"),
            ("1e-7", "1e-7"),
            ("1e21", "1e+21"),
            ("-0", "0"),
            ("01", "1"),
            ("0.5", "0.5"),
            ("-0.5", "-0.5"),
            ("0.125", "0.125"),
            ("0777", "511"),
            ("0788", "788"),
            ("0888", "888"),
        ];

        for (literal, property_key) in cases {
            assert_eq!(
                NumberLiteral(Text::new_static(literal)).to_property_key(),
                Some(property_key.to_string())
            );
        }

        assert_eq!(
            NumberLiteral(Text::new_static("not-a-number")).to_property_key(),
            None
        );
    }

    #[test]
    fn string_literals_preserve_source_and_decode_semantics() {
        let escaped = StringLiteral::from(Text::new_static(r"\u0061"));
        assert_eq!(escaped.as_str(), r"\u0061");
        assert_eq!(escaped.decoded(), Some(Text::new_static("a")));

        let newline = StringLiteral::from(Text::new_static(r"\n"));
        assert_eq!(newline.decoded(), Some(Text::new_static("\n")));

        let backslash = StringLiteral::from(Text::new_static(r"\\n"));
        assert_eq!(backslash.decoded(), Some(Text::new_static(r"\n")));
        assert_eq!(
            StringLiteral::from(Text::new_static(r"\07")).decoded(),
            None
        );
        assert_eq!(
            StringLiteral::from(Text::new_static(r"\ud800")).decoded(),
            None
        );
    }

    #[test]
    fn string_content_encoding_is_source_safe() {
        assert_eq!(encode_js_string_content("\""), Text::new_static(r#"\""#));
        assert_eq!(encode_js_string_content("\\"), Text::new_static(r"\\"));
        assert_eq!(encode_js_string_content("\n"), Text::new_static(r"\n"));
        assert_eq!(encode_js_string_content("\0"), Text::new_static(r"\u0000"));
        assert_eq!(
            encode_js_string_content("a\u{2028}b\u{2029}c"),
            Text::new_static(r"a\u2028b\u2029c")
        );
    }
}
