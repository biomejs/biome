use std::ops::Deref;

use biome_deserialize::{Deserializable, DeserializableValue, DeserializationContext};
use serde::{Deserialize, Serialize};

use crate::use_sorted_classes::UseSortedClassesOptions;

/// Options for the `noDuplicateClasses` assist action.
///
/// Controls which JSX attributes and utility functions are checked for duplicate classes.
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct NoDuplicateClassesOptions(UseSortedClassesOptions);

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

impl Deserializable for NoDuplicateClassesOptions {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        <UseSortedClassesOptions as Deserializable>::deserialize(ctx, value, name).map(Self)
    }
}
