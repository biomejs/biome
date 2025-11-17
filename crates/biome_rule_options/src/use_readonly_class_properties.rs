use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseReadonlyClassPropertiesOptions {
    /// When `true`, the keywords `public`, `protected`, and `private` are analyzed by the rule.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub check_all_properties: Option<bool>,
}

impl UseReadonlyClassPropertiesOptions {
    pub const DEFAULT_CHECK_ALL_PROPERTIES: bool = false;

    /// Returns [`Self::check_all_properties`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_CHECK_ALL_PROPERTIES`].
    pub fn check_all_properties(&self) -> bool {
        self.check_all_properties
            .unwrap_or(Self::DEFAULT_CHECK_ALL_PROPERTIES)
    }
}
