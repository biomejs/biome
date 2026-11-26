use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseConvexArgsValidatorOptions {
    /// Allow Convex functions without an `args` validator when their handler doesn't take an
    /// arguments parameter.
    /// Disabled by default.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_unused_arguments: Option<bool>,
}

impl UseConvexArgsValidatorOptions {
    pub const DEFAULT_IGNORE_UNUSED_ARGUMENTS: bool = false;

    /// Returns [`Self::ignore_unused_arguments`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_IGNORE_UNUSED_ARGUMENTS`].
    pub fn ignore_unused_arguments(&self) -> bool {
        self.ignore_unused_arguments
            .unwrap_or(Self::DEFAULT_IGNORE_UNUSED_ARGUMENTS)
    }
}
