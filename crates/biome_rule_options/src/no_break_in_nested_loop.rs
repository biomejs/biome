use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoBreakInNestedLoopOptions {
    /// Also report `continue` statements without a label inside nested loops and inside
    /// `switch` statements in loops (default: false).
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub check_continue: Option<bool>,
}

impl NoBreakInNestedLoopOptions {
    pub const DEFAULT_CHECK_CONTINUE: bool = false;

    /// Returns [`Self::check_continue`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_CHECK_CONTINUE`].
    pub fn check_continue(&self) -> bool {
        self.check_continue.unwrap_or(Self::DEFAULT_CHECK_CONTINUE)
    }
}
