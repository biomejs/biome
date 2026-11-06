use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseObjectFromEntriesOptions {
    /// Extra functions that build an object from key-value pairs, such as `utils.fromPairs`.
    /// A name can contain dots to match a function accessed through an object.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub functions: Option<Box<[Box<str>]>>,
}

impl UseObjectFromEntriesOptions {
    /// Returns [`Self::functions`] if it is set.
    /// Otherwise, returns an empty slice.
    pub fn functions(&self) -> &[Box<str>] {
        self.functions.as_deref().unwrap_or_default()
    }
}

impl biome_deserialize::Merge for UseObjectFromEntriesOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(functions) = other.functions {
            self.functions = Some(functions);
        }
    }
}
