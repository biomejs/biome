use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoRedundantRolesOptions {
    /// Array of element names that the rule should ignore.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_elements: Option<Box<[Box<str>]>>,
}

impl NoRedundantRolesOptions {
    /// Returns [`Self::ignore_elements`] if it is set.
    /// Otherwise, returns an empty slice.
    pub fn ignore_elements(&self) -> &[Box<str>] {
        self.ignore_elements.as_deref().unwrap_or_default()
    }
}
