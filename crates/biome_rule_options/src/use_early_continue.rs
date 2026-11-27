use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseEarlyContinueOptions {
    /// The largest number of statements that an `if` statement at the end of a loop body can
    /// contain before the rule reports it (default: 1).
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub maximum_statements: Option<u16>,
}

impl UseEarlyContinueOptions {
    pub const DEFAULT_MAXIMUM_STATEMENTS: u16 = 1;

    /// Returns [`Self::maximum_statements`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_MAXIMUM_STATEMENTS`].
    pub fn maximum_statements(&self) -> u16 {
        self.maximum_statements
            .unwrap_or(Self::DEFAULT_MAXIMUM_STATEMENTS)
    }
}
