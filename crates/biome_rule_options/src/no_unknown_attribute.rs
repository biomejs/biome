use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoUnknownAttributeOptions {
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore: Option<Box<[Box<str>]>>,
}

impl NoUnknownAttributeOptions {
    /// Returns [`Self::ignore`] if it is set.
    /// Otherwise, returns an empty slice.
    pub fn ignore(&self) -> &[Box<str>] {
        self.ignore.as_deref().unwrap_or_default()
    }
}

impl biome_deserialize::Merge for NoUnknownAttributeOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(ignore) = other.ignore {
            self.ignore = Some(ignore);
        }
    }
}
