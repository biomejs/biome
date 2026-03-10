use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
/// Options for the `useExplicitReturnType` rule.
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize, Merge)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseExplicitReturnTypeOptions {
    /// Whether to allow IIFEs (Immediately Invoked Function Expressions) without explicit return types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_iifes: Option<bool>,

    /// A list of function names that are allowed to not have explicit return types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_names: Option<Vec<String>>,

    /// Whether to ignore function expressions (functions that are not part of a declaration).
    /// When `true`, only declarations (function statements and class methods) are checked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_expressions: Option<bool>,
}
