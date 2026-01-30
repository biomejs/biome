use std::ops::Deref;

use biome_deserialize::{
    Deserializable, DeserializableTypes, DeserializableValue, DeserializationContext,
    DeserializationDiagnostic, DeserializationVisitor, TextRange,
};
use biome_rowan::Text;
use serde::{Deserialize, Serialize};

use crate::use_sorted_classes::UseSortedClassesOptions;

/// Options for the `noDuplicateClasses` assist action.
///
/// Controls which JSX attributes and utility functions are checked for duplicate classes.
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct NoDuplicateClassesOptions(UseSortedClassesOptions);

impl NoDuplicateClassesOptions {
    /// Create new options with the given attributes and functions.
    pub fn new(attributes: Option<Box<[Box<str>]>>, functions: Option<Box<[Box<str>]>>) -> Self {
        Self(UseSortedClassesOptions {
            attributes,
            functions,
        })
    }
}

impl Deref for NoDuplicateClassesOptions {
    type Target = UseSortedClassesOptions;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl biome_deserialize::Merge for NoDuplicateClassesOptions {
    fn merge_with(&mut self, other: Self) {
        self.0.merge_with(other.0);
    }
}

// Custom Serialize to match UseSortedClassesOptions format
impl Serialize for NoDuplicateClassesOptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

// Custom Deserialize to match UseSortedClassesOptions format
impl<'de> Deserialize<'de> for NoDuplicateClassesOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        <UseSortedClassesOptions as serde::Deserialize>::deserialize(deserializer).map(Self)
    }
}

// Custom JsonSchema to generate proper schema with distinct type name
#[cfg(feature = "schema")]
impl schemars::JsonSchema for NoDuplicateClassesOptions {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("NoDuplicateClassesOptions")
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        // Generate schema based on the inner UseSortedClassesOptions but with our type name
        // The schema is already correct, we just need the distinct type name (handled by schema_name)
        UseSortedClassesOptions::json_schema(generator)
    }
}

const ALLOWED_OPTIONS: &[&str] = &["attributes", "functions"];

impl Deserializable for NoDuplicateClassesOptions {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        value.deserialize(ctx, NoDuplicateClassesOptionsVisitor, name)
    }
}

struct NoDuplicateClassesOptionsVisitor;

impl DeserializationVisitor for NoDuplicateClassesOptionsVisitor {
    type Output = NoDuplicateClassesOptions;

    const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::MAP;

    fn visit_map(
        self,
        ctx: &mut impl DeserializationContext,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: TextRange,
        _name: &str,
    ) -> Option<Self::Output> {
        let mut inner = UseSortedClassesOptions::default();

        let mut attributes = Vec::new();
        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(ctx, &key, "") else {
                continue;
            };
            match key_text.text() {
                "attributes" => {
                    if let Some(attributes_option) =
                        Deserializable::deserialize(ctx, &value, &key_text)
                    {
                        attributes.extend::<Vec<Box<str>>>(attributes_option);
                    }
                }
                "functions" => {
                    inner.functions = Deserializable::deserialize(ctx, &value, &key_text)
                }
                unknown_key => ctx.report(DeserializationDiagnostic::new_unknown_key(
                    unknown_key,
                    key.range(),
                    ALLOWED_OPTIONS,
                )),
            }
        }
        inner.attributes = if attributes.is_empty() {
            None
        } else {
            Some(attributes.into_boxed_slice())
        };
        Some(NoDuplicateClassesOptions(inner))
    }
}
