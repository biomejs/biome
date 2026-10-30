use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoShortTimeValuesOptions {
    /// The shortest time allowed for animations and transitions, as a whole number of milliseconds.
    ///
    /// This option is required to enable the rule.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub minimum: Option<u32>,
}

impl NoShortTimeValuesOptions {
    /// Returns [`Self::minimum`] if it is set.
    pub fn minimum(&self) -> Option<u32> {
        self.minimum
    }
}
