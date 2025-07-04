use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseImportTypeOptions {
    /// The style to apply when import types. Default to "auto"
    pub style: Style,
}

/// Rule's options.
#[derive(
    Debug, Default, Copy, Clone, Deserializable, Eq, PartialEq, serde::Deserialize, serde::Serialize,
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
