use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

/// Options for the `useNullishCoalescing` rule.
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseNullishCoalescingOptions {
    /// Ignore `||` expressions in conditional test positions (default: `true`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_conditional_tests: Option<bool>,

    /// Ignore ternary expressions that check for `null` or `undefined` (default: `false`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_ternary_tests: Option<bool>,

    /// Whether to ignore `||` and `||=` binary operations that are part of a mixed logical expression with `&&` (default: `false`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_mixed_logical_expressions: Option<bool>,

    /// Whether to ignore `||` and `||=` binary operations used inside a `Boolean()` call (default: `false`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_boolean_coercion: Option<bool>,
}

impl UseNullishCoalescingOptions {
    pub fn ignore_conditional_tests(&self) -> bool {
        self.ignore_conditional_tests.unwrap_or(true)
    }

    pub fn ignore_ternary_tests(&self) -> bool {
        self.ignore_ternary_tests.unwrap_or(false)
    }

    pub fn ignore_mixed_logical_expressions(&self) -> bool {
        self.ignore_mixed_logical_expressions.unwrap_or(false)
    }

    pub fn ignore_boolean_coercion(&self) -> bool {
        self.ignore_boolean_coercion.unwrap_or(false)
    }
}
