use biome_console::markup;
use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationContext,
    DeserializationDiagnostic,
};
use biome_deserialize_macros::Merge;
use biome_diagnostics::Severity;
use serde::de::{self, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// Extends the current configuration with settings from other Biome configurations.
///
/// `"//"` extends the root configuration from any nesting depth. A list extends configurations by
/// relative path or installed package specifier. Biome merges list entries from left to right:
/// later configurations take precedence for single-value options, list entries are combined, and
/// the current configuration is applied last.
#[derive(Clone, Debug, Eq, PartialEq, Merge)]
pub enum Extends {
    List(Box<[Box<str>]>),
    ExtendsRoot,
}

impl Extends {
    pub fn as_list(&self) -> Option<&[Box<str>]> {
        match self {
            Self::List(list) => Some(list),
            Self::ExtendsRoot => None,
        }
    }

    pub const fn extends_root(&self) -> bool {
        matches!(self, Self::ExtendsRoot)
    }
}

impl Default for Extends {
    fn default() -> Self {
        Self::List(Box::new([]))
    }
}

impl Deserializable for Extends {
    fn deserialize<V: DeserializableValue>(
        ctx: &mut dyn DeserializationContext<State = V::State>,
        value: &V,
        name: &str,
    ) -> Option<Self> {
        Some(if value.visitable_type(ctx)? == DeserializableType::Str {
            let deserialized_value: String = Deserializable::deserialize(ctx, value, name)?;
            if &deserialized_value == "//" {
                Self::ExtendsRoot
            } else {
                ctx.report(
                    DeserializationDiagnostic::new(markup! {
                        "The `extends` field must be either `//` or an array of paths."
                    })
                    .with_custom_severity(Severity::Error)
                    .with_range(value.range()),
                );
                return None;
            }
        } else {
            Self::List(Deserializable::deserialize(ctx, value, name)?)
        })
    }
}

// A derived `untagged` impl writes `ExtendsRoot` as `null`. Biome reads a
// nested configuration file with `null` as a second root.
impl Serialize for Extends {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::List(list) => list.serialize(serializer),
            Self::ExtendsRoot => serializer.serialize_str("//"),
        }
    }
}

impl<'de> Deserialize<'de> for Extends {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ExtendsVisitor;

        impl<'de> Visitor<'de> for ExtendsVisitor {
            type Value = Extends;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("`//` or an array of paths")
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
                if value == "//" {
                    Ok(Extends::ExtendsRoot)
                } else {
                    Err(E::invalid_value(de::Unexpected::Str(value), &self))
                }
            }

            fn visit_seq<A: SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error> {
                Deserialize::deserialize(de::value::SeqAccessDeserializer::new(seq))
                    .map(Extends::List)
            }
        }

        deserializer.deserialize_any(ExtendsVisitor)
    }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for Extends {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("Extends")
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "description": "Extends the current configuration with settings from other Biome configurations. `//` extends the root configuration from any nesting depth. A list extends configurations by relative path or installed package specifier. Biome merges list entries from left to right: later configurations take precedence for single-value options, list entries are combined, and the current configuration is applied last.",
            "oneOf": [
                {
                    "type": "array",
                    "items": {
                        "type": "string"
                    }
                },
                {
                    "type": "string",
                    "const": "//"
                }
            ]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_round_trip() {
        let cases = [
            (Extends::ExtendsRoot, r#""//""#),
            (
                Extends::List(Box::new(["./base.json".into()])),
                r#"["./base.json"]"#,
            ),
        ];
        for (extends, json) in cases {
            assert_eq!(serde_json::to_string(&extends).unwrap(), json);
            assert_eq!(serde_json::from_str::<Extends>(json).unwrap(), extends);
        }
    }

    #[test]
    fn serde_rejects_other_strings() {
        assert!(serde_json::from_str::<Extends>(r#""./base.json""#).is_err());
        assert!(serde_json::from_str::<Extends>("null").is_err());
    }
}
