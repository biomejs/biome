use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoExcessiveStatementsPerFunctionOptions {
    /// The maximum number of statements allowed in a function (default: 10).
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub max: Option<u16>,
    /// When this option is set to `true`, a function that isn't nested in another function is
    /// not checked, as long as it's the only such function in the file (default: false).
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_top_level_functions: Option<bool>,
}

impl NoExcessiveStatementsPerFunctionOptions {
    pub const DEFAULT_MAX: u16 = 10;
    pub const DEFAULT_IGNORE_TOP_LEVEL_FUNCTIONS: bool = false;

    /// Returns [`Self::max`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_MAX`].
    pub fn max(&self) -> u16 {
        self.max.unwrap_or(Self::DEFAULT_MAX)
    }

    /// Returns [`Self::ignore_top_level_functions`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_IGNORE_TOP_LEVEL_FUNCTIONS`].
    pub fn ignore_top_level_functions(&self) -> bool {
        self.ignore_top_level_functions
            .unwrap_or(Self::DEFAULT_IGNORE_TOP_LEVEL_FUNCTIONS)
    }
}
