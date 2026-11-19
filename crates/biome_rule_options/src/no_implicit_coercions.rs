use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoImplicitCoercionsOptions {
    /// Whether to allow implicitly coercing values to booleans via `!!value`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub allow_double_negation: Option<bool>,
}

impl NoImplicitCoercionsOptions {
    /// Whether to allow implicitly coercing values to booleans via `!!value`.
    pub fn double_negation(&self) -> bool {
        self.allow_double_negation.unwrap_or_default()
    }
}
