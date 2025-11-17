use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoConfusingLabelsOptions {
    /// A list of (non-confusing) labels that should be allowed
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub allowed_labels: Option<Box<[Box<str>]>>,
}

impl biome_deserialize::Merge for NoConfusingLabelsOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(allowed_labels) = other.allowed_labels {
            self.allowed_labels = Some(allowed_labels);
        }
    }
}
