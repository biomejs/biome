use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseNullishCoalescingOptions {
    /// Whether to ignore `||` expressions in conditional test positions
    /// (if/while/for/do-while/ternary conditions).
    ///
    /// When `true` (the default), the rule will not report `||` expressions
    /// that appear in places where the falsy-checking behavior may be intentional.
    ///
    /// Default: `true`
    pub ignore_conditional_tests: Option<bool>,

    /// Whether to ignore ternary expressions that could be simplified
    /// using the nullish coalescing operator.
    ///
    /// Default: `false`
    pub ignore_ternary_tests: Option<bool>,

    /// Whether to ignore `||` expressions when they appear as an argument
    /// to the global `Boolean()` constructor, where boolean coercion is
    /// the explicit intent.
    ///
    /// Only suppresses the diagnostic when `Boolean` resolves to the global
    /// built-in. If `Boolean` is shadowed by a local binding, the diagnostic
    /// is still reported.
    ///
    /// Default: `false`
    pub ignore_boolean_coercion: Option<bool>,
}

impl UseNullishCoalescingOptions {
    pub fn ignore_conditional_tests(&self) -> bool {
        self.ignore_conditional_tests.unwrap_or(true)
    }

    pub fn ignore_ternary_tests(&self) -> bool {
        self.ignore_ternary_tests.unwrap_or(false)
    }

    pub fn ignore_boolean_coercion(&self) -> bool {
        self.ignore_boolean_coercion.unwrap_or(false)
    }
}
