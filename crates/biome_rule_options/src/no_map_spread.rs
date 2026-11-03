use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoMapSpreadOptions {
    /// Ignores `map()` and `flatMap()` calls on a variable that is read again after the call.
    /// Code that reads the original array later may rely on the copies to keep
    /// the original items unchanged. Enabled by default.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_rereads: Option<bool>,

    /// Ignores `map()` and `flatMap()` calls on a function parameter. The caller owns the
    /// array, so changing its items in place could affect code outside the
    /// function. Enabled by default.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_args: Option<bool>,
}

impl NoMapSpreadOptions {
    pub const DEFAULT_IGNORE_REREADS: bool = true;
    pub const DEFAULT_IGNORE_ARGS: bool = true;

    /// Returns [`Self::ignore_rereads`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_IGNORE_REREADS`].
    pub fn ignore_rereads(&self) -> bool {
        self.ignore_rereads.unwrap_or(Self::DEFAULT_IGNORE_REREADS)
    }

    /// Returns [`Self::ignore_args`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_IGNORE_ARGS`].
    pub fn ignore_args(&self) -> bool {
        self.ignore_args.unwrap_or(Self::DEFAULT_IGNORE_ARGS)
    }
}
