use std::{borrow::Cow, ops::Neg};

use biome_js_type_info_macros::Resolvable;
use biome_rowan::Text;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Resolvable)]
pub struct BooleanLiteral(bool);

impl BooleanLiteral {
    #[inline]
    pub fn as_bool(&self) -> bool {
        self.0
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.as_bytes().first() {
            Some(b't') => Some(Self(true)),
            Some(b'f') => Some(Self(false)),
            _ => None,
        }
    }
}

impl From<bool> for BooleanLiteral {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Resolvable)]
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

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Resolvable)]
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
