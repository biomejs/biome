use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoExcessiveSelectorClassesOptions {
    /// The maximum number of class selectors allowed in a single selector.
    ///
    /// This option is required to enable the rule.
    /// Use `0` to disallow class selectors entirely.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub max_classes: Option<u16>,
}

impl NoExcessiveSelectorClassesOptions {
    /// Returns [`Self::max_classes`] if it is set.
    pub fn max_classes(&self) -> Option<u16> {
        self.max_classes
    }
}
