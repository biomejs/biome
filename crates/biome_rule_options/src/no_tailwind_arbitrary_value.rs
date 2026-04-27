use std::ops::Deref;

use biome_deserialize::{Deserializable, DeserializableValue, DeserializationContext};
use serde::{Deserialize, Serialize};

use crate::use_sorted_classes::UseSortedClassesOptions;

/// Options for the `noTailwindArbitraryValue` rule.
///
/// Controls which attributes and utility functions are checked for arbitrary values.
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct NoTailwindArbitraryValueOptions(UseSortedClassesOptions);

impl Deref for NoTailwindArbitraryValueOptions {
    type Target = UseSortedClassesOptions;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl biome_deserialize::Merge for NoTailwindArbitraryValueOptions {
    fn merge_with(&mut self, other: Self) {
        self.0.merge_with(other.0);
    }
}

// Custom Serialize to match UseSortedClassesOptions format
impl Serialize for NoTailwindArbitraryValueOptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

// Custom Deserialize to match UseSortedClassesOptions format
impl<'de> Deserialize<'de> for NoTailwindArbitraryValueOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        <UseSortedClassesOptions as serde::Deserialize>::deserialize(deserializer).map(Self)
    }
}

// Developer note: `NoTailwindArbitraryValueOptions` hand-writes its
// `impl schemars::JsonSchema` so the schema has a distinct type name, but the
// fields mirror `UseSortedClassesOptions`. Keep this schema in sync whenever
// `UseSortedClassesOptions` changes to avoid drift.
#[cfg(feature = "schema")]
impl schemars::JsonSchema for NoTailwindArbitraryValueOptions {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("NoTailwindArbitraryValueOptions")
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "object",
            "properties": {
                "attributes": {
                    "description": "Additional attributes that will be checked.",
                    "type": ["array", "null"],
                    "items": { "type": "string" }
                },
                "functions": {
                    "description": "Names of the functions or tagged templates that will be checked.",
                    "type": ["array", "null"],
                    "items": { "type": "string" }
                }
            },
            "additionalProperties": false
        })
    }
}

impl Deserializable for NoTailwindArbitraryValueOptions {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        <UseSortedClassesOptions as Deserializable>::deserialize(ctx, value, name).map(Self)
    }
}
