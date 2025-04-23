use biome_js_type_info_macros::Resolvable;
use biome_rowan::Text;

#[derive(Clone, Debug, Default, PartialEq, Resolvable)]
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

#[derive(Clone, Debug, Default, PartialEq, Resolvable)]
pub struct NumberLiteral(f64);

impl NumberLiteral {
    #[inline]
    pub fn as_f64(&self) -> f64 {
        self.0
    }

    #[inline]
    pub fn inverse(&self) -> Self {
        Self(-self.0)
    }

    pub fn parse(s: &str) -> Option<Self> {
        // Remove numeric separators first.
        let s = s.replace('_', "");

        if let Some(s) = s.strip_prefix("0b").or_else(|| s.strip_prefix("0B")) {
            return Some(Self(u64::from_str_radix(s, 2).ok()? as f64));
        }

        if let Some(s) = s.strip_prefix("0o").or_else(|| s.strip_prefix("0O")) {
            return Some(Self(u64::from_str_radix(s, 8).ok()? as f64));
        }

        if let Some(s) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
            return Some(Self(u64::from_str_radix(s, 16).ok()? as f64));
        }

        // Handle a legacy octal literal or a decimal literal with a leading zero.
        if let Some(s) = s.strip_prefix("0") {
            if !s.is_empty() && !s.starts_with('e') && !s.starts_with('E') {
                if !s.contains('8') && !s.contains('9') {
                    return Some(Self(u64::from_str_radix(s, 8).ok()? as f64));
                }

                return Some(Self(s.parse().ok()?));
            }
        }

        Some(Self(s.parse().ok()?))
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Resolvable)]
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
        Self(Text::Owned(value))
    }
}

impl From<&str> for StringLiteral {
    fn from(value: &str) -> Self {
        Self(Text::Owned(value.to_string()))
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
            NumberLiteral::parse("1234567890"),
            Some(NumberLiteral(1_234_567_890.0))
        );
        assert_eq!(NumberLiteral::parse("42"), Some(NumberLiteral(42.0)));
        assert_eq!(NumberLiteral::parse("0"), Some(NumberLiteral(0.0)));
    }

    #[test]
    fn parse_number_legacy_octal() {
        assert_eq!(NumberLiteral::parse("0888"), Some(NumberLiteral(888.0)));
        assert_eq!(NumberLiteral::parse("0777"), Some(NumberLiteral(511.0)));
    }

    #[test]
    fn parse_number_exponential() {
        assert_eq!(NumberLiteral::parse("0e-5"), Some(NumberLiteral(0.0)));
        assert_eq!(NumberLiteral::parse("0e+5"), Some(NumberLiteral(0.0)));
        assert_eq!(NumberLiteral::parse("5e1"), Some(NumberLiteral(50.0)));
        assert_eq!(NumberLiteral::parse("175e-2"), Some(NumberLiteral(1.75)));
        assert_eq!(NumberLiteral::parse("1e3"), Some(NumberLiteral(1000.0)));
        assert_eq!(NumberLiteral::parse("1e-3"), Some(NumberLiteral(0.001)));
        assert_eq!(NumberLiteral::parse("1E3"), Some(NumberLiteral(1000.0)));
    }

    #[test]
    fn parse_number_binary() {
        assert_eq!(
            NumberLiteral::parse("0b10000000000000000000000000000000"),
            Some(NumberLiteral(2_147_483_648.0))
        );
        assert_eq!(
            NumberLiteral::parse("0b01111111100000000000000000000000"),
            Some(NumberLiteral(2_139_095_040.0))
        );
        assert_eq!(
            NumberLiteral::parse("0B00000000011111111111111111111111"),
            Some(NumberLiteral(8_388_607.0))
        );
    }

    #[test]
    fn parse_number_octal() {
        assert_eq!(NumberLiteral::parse("0O755"), Some(NumberLiteral(493.0)));
        assert_eq!(NumberLiteral::parse("0o644"), Some(NumberLiteral(420.0)));
    }

    #[test]
    fn parse_number_hexadecimal() {
        // TODO: This is bigger than u64 and can't parse it via u64::from_str_radix.
        //       However it's a valid number literal in ECMAScript, though it will be truncated
        //       to the precision of f64.
        // assert_eq!(
        //     NumberLiteral::parse("0xFFFFFFFFFFFFFFFFF"),
        //     Some(NumberLiteral(295147905179352830000.0))
        // );
        assert_eq!(
            NumberLiteral::parse("0x123456789ABCDEF"),
            Some(NumberLiteral(81_985_529_216_486_900.0))
        );
        assert_eq!(NumberLiteral::parse("0XA"), Some(NumberLiteral(10.0)));
    }

    #[test]
    fn parse_number_separators() {
        assert_eq!(
            NumberLiteral::parse("1_000_000_000_000"),
            Some(NumberLiteral(1_000_000_000_000.0))
        );
        assert_eq!(
            NumberLiteral::parse("1_050.95"),
            Some(NumberLiteral(1050.95))
        );
        assert_eq!(
            NumberLiteral::parse("0b1010_0001_1000_0101"),
            Some(NumberLiteral(41349.0))
        );
        assert_eq!(
            NumberLiteral::parse("0o2_2_5_6"),
            Some(NumberLiteral(1198.0))
        );
        assert_eq!(
            NumberLiteral::parse("0xA0_B0_C0"),
            Some(NumberLiteral(10_531_008.0))
        );
    }
}
