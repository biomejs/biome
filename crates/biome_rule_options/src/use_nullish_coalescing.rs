use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseNullishCoalescingOptions {
    /// Ignore `||` in conditional test positions (if/while/for/ternary test)
    ///
    /// Default: `true`
    pub ignore_conditional_tests: bool,
}

impl Default for UseNullishCoalescingOptions {
    fn default() -> Self {
        Self {
            ignore_conditional_tests: true,
        }
    }
}
