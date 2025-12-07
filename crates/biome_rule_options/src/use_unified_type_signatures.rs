use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseUnifiedTypeSignaturesOptions {
    /// Whether to ignore overloads with differently named parameters.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_differently_named_parameters: Option<bool>,

    /// Whether to ignore overloads with different JSDoc comments.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_different_js_doc: Option<bool>,
}
