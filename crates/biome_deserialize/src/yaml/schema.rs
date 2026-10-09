//! The type of scalars: null, boolean, number, or string, resolved with the
//! [core schema](https://yaml.org/spec/1.2.2/#103-core-schema) and its tags.
use crate::TextNumber;
use biome_rowan::{AstNodeList, SyntaxKind, Text, TokenText};
use biome_unicode_table::{
    Dispatch::{DIG, IDT, ZER},
    lookup_byte,
};
use biome_yaml_syntax::{AnyYamlFlowScalar, YamlPropertyList, YamlSyntaxKind};

/// The value of a flow scalar.
pub(super) enum Scalar {
    Null,
    Bool(bool),
    Int(TextNumber),
    Float(TextNumber),
    Str(Text),
    /// A scalar that doesn't match its tag.
    Invalid(CoreTag),
}

/// Resolves the value of `scalar`, whose content is `text`. A tag of the core schema in its
/// `properties` sets its type, otherwise plain scalars are resolved with the core schema, and
/// quoted scalars are strings.
pub(super) fn resolve_flow_scalar(
    scalar: &AnyYamlFlowScalar,
    text: TokenText,
    properties: &YamlPropertyList,
) -> Scalar {
    match CoreTag::from_properties(properties) {
        None if matches!(scalar, AnyYamlFlowScalar::YamlPlainScalar(_)) => {
            resolve_plain_scalar(text)
        }
        None | Some(CoreTag::Str) => Scalar::Str(text.into()),
        Some(tag) => {
            let resolved = resolve_plain_scalar(text);
            if tag.accepts(&resolved) {
                resolved
            } else {
                Scalar::Invalid(tag)
            }
        }
    }
}

/// A scalar tag of the [core schema](https://yaml.org/spec/1.2.2/#103-core-schema).
#[derive(Clone, Copy)]
pub(super) enum CoreTag {
    Null,
    Bool,
    Int,
    Float,
    Str,
}

impl CoreTag {
    fn from_properties(properties: &YamlPropertyList) -> Option<Self> {
        properties.iter().find_map(|property| {
            let token = property.as_yaml_tag_property()?.value_token().ok()?;
            let tag = match token.text_trimmed() {
                "!!null" | "!<tag:yaml.org,2002:null>" => Self::Null,
                "!!bool" | "!<tag:yaml.org,2002:bool>" => Self::Bool,
                "!!int" | "!<tag:yaml.org,2002:int>" => Self::Int,
                "!!float" | "!<tag:yaml.org,2002:float>" => Self::Float,
                // The non-specific tag `!` makes a scalar a string too
                "!" | "!!str" | "!<tag:yaml.org,2002:str>" => Self::Str,
                _ => return None,
            };
            Some(tag)
        })
    }

    pub(super) fn name(self) -> &'static str {
        match self {
            Self::Null => "!!null",
            Self::Bool => "!!bool",
            Self::Int => "!!int",
            Self::Float => "!!float",
            Self::Str => "!!str",
        }
    }

    /// Whether `scalar` is a valid value for this tag.
    fn accepts(self, scalar: &Scalar) -> bool {
        matches!(
            (self, scalar),
            (Self::Null, Scalar::Null)
                | (Self::Bool, Scalar::Bool(_))
                | (Self::Int, Scalar::Int(_))
                | (Self::Float, Scalar::Int(_) | Scalar::Float(_))
                | (Self::Str, _)
        )
    }
}

/// Resolves a plain scalar with the [core schema](https://yaml.org/spec/1.2.2/#103-core-schema).
fn resolve_plain_scalar(text: TokenText) -> Scalar {
    match text.text() {
        "" | "~" | "null" | "Null" | "NULL" => Scalar::Null,
        "true" | "True" | "TRUE" => Scalar::Bool(true),
        "false" | "False" | "FALSE" => Scalar::Bool(false),
        // Numbers use the syntax of Rust, so that they can be parsed
        ".inf" | ".Inf" | ".INF" | "+.inf" | "+.Inf" | "+.INF" => {
            Scalar::Float(owned_number("inf"))
        }
        "-.inf" | "-.Inf" | "-.INF" => Scalar::Float(owned_number("-inf")),
        ".nan" | ".NaN" | ".NAN" => Scalar::Float(owned_number("NaN")),
        value => {
            let radix_digits = value
                .strip_prefix("0o")
                .map(|digits| (digits, 8))
                .or_else(|| value.strip_prefix("0x").map(|digits| (digits, 16)));
            if let Some((digits, radix)) = radix_digits
                && is_digits(digits, radix)
            {
                // Integers that overflow are kept as they are, and fail to parse later
                return Scalar::Int(u128::from_str_radix(digits, radix).map_or_else(
                    |_| TextNumber(text.clone()),
                    |value| owned_number(&value.to_string()),
                ));
            }
            if is_integer(value) {
                Scalar::Int(TextNumber(without_plus_sign(text)))
            } else if is_float(value) {
                Scalar::Float(TextNumber(without_plus_sign(text)))
            } else {
                Scalar::Str(text.into())
            }
        }
    }
}

fn owned_number(text: &str) -> TextNumber {
    TextNumber(TokenText::new_raw(
        YamlSyntaxKind::PLAIN_LITERAL.to_raw(),
        text,
    ))
}

/// Removes the leading `+` of a number, which `serde_json` doesn't parse.
fn without_plus_sign(text: TokenText) -> TokenText {
    text.strip_prefix('+').unwrap_or(text)
}

fn is_digits(text: &str, radix: u32) -> bool {
    !text.is_empty()
        && text.bytes().all(|byte| match lookup_byte(byte) {
            ZER => true,
            DIG => u32::from(byte - b'0') < radix,
            IDT => radix == 16 && matches!(byte, b'a'..=b'f' | b'A'..=b'F'),
            _ => false,
        })
}

/// Whether `text` matches `[-+]?[0-9]+`.
fn is_integer(text: &str) -> bool {
    is_digits(text.strip_prefix(['-', '+']).unwrap_or(text), 10)
}

/// Whether `text` matches `[-+]?(\.[0-9]+|[0-9]+(\.[0-9]*)?)([eE][-+]?[0-9]+)?`.
fn is_float(text: &str) -> bool {
    let text = text.strip_prefix(['-', '+']).unwrap_or(text);
    let (mantissa, exponent) = match text.split_once(['e', 'E']) {
        Some((mantissa, exponent)) => (mantissa, Some(exponent)),
        None => (text, None),
    };
    let is_mantissa_valid = match mantissa.split_once('.') {
        Some(("", fraction)) => is_digits(fraction, 10),
        Some((integer, "")) => is_digits(integer, 10),
        Some((integer, fraction)) => is_digits(integer, 10) && is_digits(fraction, 10),
        None => is_digits(mantissa, 10),
    };
    is_mantissa_valid
        && exponent.is_none_or(|exponent| {
            is_digits(exponent.strip_prefix(['-', '+']).unwrap_or(exponent), 10)
        })
}
