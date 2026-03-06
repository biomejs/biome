use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationContext,
};
use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The Baseline availability level to target.
///
/// - `"widely"` – warn on anything not Baseline widely available.
/// - `"newly"` – warn on anything not at least Baseline newly available.
/// - A year (e.g. `2023`) – warn on anything whose `baseline_low_date` is after
///   that year (i.e. became newly available after that year).
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum AvailabilityTarget {
    /// A named tier: `"widely"` or `"newly"`.
    Named(AvailabilityNamed),
    /// A year cutoff (e.g. `2023`).
    Year(u16),
}

impl Default for AvailabilityTarget {
    fn default() -> Self {
        Self::Named(AvailabilityNamed::Widely)
    }
}

impl biome_deserialize::Merge for AvailabilityTarget {
    fn merge_with(&mut self, other: Self) {
        *self = other;
    }
}

impl Deserializable for AvailabilityTarget {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        Some(if value.visitable_type()? == DeserializableType::Str {
            Self::Named(<AvailabilityNamed as Deserializable>::deserialize(
                ctx, value, name,
            )?)
        } else {
            Self::Year(<u16 as Deserializable>::deserialize(ctx, value, name)?)
        })
    }
}

/// Named Baseline availability tiers.
#[derive(Default, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Deserializable, Merge)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum AvailabilityNamed {
    /// Warn on anything not Baseline widely available (default).
    #[default]
    Widely,
    /// Warn on anything not at least Baseline newly available.
    Newly,
}

/// Options for the `useBaseline` rule.
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseBaselineOptions {
    /// The availability level to target. Defaults to `"widely"`.
    #[serde(skip_serializing_if = "AvailabilityTarget::is_default")]
    pub available: AvailabilityTarget,

    /// CSS properties to exclude from checking (case-insensitive).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allow_properties: Vec<String>,

    /// CSS at-rules to exclude from checking (without `@`, case-insensitive).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allow_at_rules: Vec<String>,

    /// CSS value functions to exclude from checking (case-insensitive).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allow_functions: Vec<String>,

    /// CSS media conditions to exclude from checking (case-insensitive).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allow_media_conditions: Vec<String>,

    /// CSS property values to exclude from checking (maps property name to allowed values, case-insensitive).
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub allow_property_values: HashMap<String, Vec<String>>,

    /// CSS pseudo-selectors to exclude from checking (without `:` or `::`, case-insensitive).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allow_selectors: Vec<String>,
}

impl AvailabilityTarget {
    fn is_default(&self) -> bool {
        matches!(self, Self::Named(AvailabilityNamed::Widely))
    }
}
