use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseMaxParamsOptions {
    /// Maximum number of parameters allowed (default: 4)
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub max: Option<u8>,
}

impl UseMaxParamsOptions {
    pub const DEFAULT_MAX: u8 = 4;

    /// Returns [`Self::max`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_MAX`].
    pub fn max(&self) -> u8 {
        self.max.unwrap_or(Self::DEFAULT_MAX)
    }
}
