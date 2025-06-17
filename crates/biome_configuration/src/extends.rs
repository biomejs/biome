use biome_console::markup;
use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationContext,
    DeserializationDiagnostic,
};
use biome_deserialize_macros::Merge;
use biome_diagnostics::Severity;
use serde::{Deserialize, Serialize};

/// A list of paths to other JSON files, used to extends the current configuration.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Merge)]
#[serde(deny_unknown_fields, rename_all = "camelCase", untagged)]
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
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        Some(if value.visitable_type()? == DeserializableType::Str {
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

#[cfg(feature = "schema")]
impl schemars::JsonSchema for Extends {
    fn schema_name() -> String {
        "Extends".to_string()
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::schema::Schema {
        #[derive(serde::Deserialize, schemars::JsonSchema)]
        #[serde(untagged)]
        #[expect(dead_code)]
        enum ExtendsSchema {
            List(Vec<String>),
            String(String),
        }
        ExtendsSchema::json_schema(generator)
    }
}
