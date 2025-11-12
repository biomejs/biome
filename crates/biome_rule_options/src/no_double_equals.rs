use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoDoubleEqualsOptions {
    /// If `true`, an exception is made when comparing with `null`, as it's often relied on to check
    /// both for `null` or `undefined`.
    ///
    /// If `false`, no such exception will be made.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_null: Option<bool>,
}

impl NoDoubleEqualsOptions {
    pub const DEFAULT_IGNORE_NULL: bool = true;

    /// Returns [`Self::ignore_null`] if it is set
    /// Otherwise, returns [`Self::DEFAULT_IGNORE_NULL`].
    pub fn ignore_null(&self) -> bool {
        self.ignore_null.unwrap_or(Self::DEFAULT_IGNORE_NULL)
    }
}
