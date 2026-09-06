use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

/// Configures the required function style and whether declaration mode permits arrow functions.
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseConsistentFunctionStyleOptions {
    /// The function style to enforce. Default: `"expression"`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub style: Option<FunctionStyle>,
    /// Allow arrow functions when declarations are required. Default: `false`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub allow_arrow_functions: Option<bool>,
}

/// The required form for function definitions: `"expression"` or `"declaration"`.
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum FunctionStyle {
    /// Require function expressions assigned to variables. This is the default.
    #[default]
    Expression,
    /// Require function declarations, subject to the rule's exceptions.
    Declaration,
}
