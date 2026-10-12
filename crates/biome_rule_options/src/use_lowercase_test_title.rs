use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseLowercaseTestTitleOptions {
    /// A list of prefixes. Titles that start with one of these prefixes are not checked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_prefixes: Option<Box<[Box<str>]>>,
}

impl UseLowercaseTestTitleOptions {
    /// Returns [`Self::allowed_prefixes`] if it is set.
    /// Otherwise, returns an empty slice.
    pub fn allowed_prefixes(&self) -> &[Box<str>] {
        self.allowed_prefixes.as_deref().unwrap_or_default()
    }
}
