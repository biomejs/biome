use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseImportTypeOptions {
    /// The style to apply when import types. Default to "auto"
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub style: Option<Style>,
}

/// Rule's options.
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    Deserializable,
    Merge,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum Style {
    /// Use the best fitting style according to the situation
    #[default]
    Auto,
    /// Always use inline type keywords
    InlineType,
    /// Always separate types in a dedicated `import type`
    SeparatedType,
}
