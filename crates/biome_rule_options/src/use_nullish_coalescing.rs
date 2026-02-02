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
}

impl UseNullishCoalescingOptions {
    pub fn ignore_conditional_tests(&self) -> bool {
        self.ignore_conditional_tests.unwrap_or(true)
    }
}
