use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoVueUndeclaredPropertiesOptions {
    /// Property names that are never reported, for example properties that a plugin adds to
    /// every component.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore: Option<Box<[Box<str>]>>,
}

impl NoVueUndeclaredPropertiesOptions {
    /// Returns [`Self::ignore`] if it is set.
    /// Otherwise, returns an empty slice.
    pub fn ignore(&self) -> &[Box<str>] {
        self.ignore.as_deref().unwrap_or_default()
    }
}

// Not derived: the derived implementation would append lists across extended
// configurations, so a child configuration could never remove an inherited
// name. A set list replaces the inherited one instead.
impl biome_deserialize::Merge for NoVueUndeclaredPropertiesOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(ignore) = other.ignore {
            self.ignore = Some(ignore);
        }
    }
}
