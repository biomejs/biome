use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoLabelWithoutControlOptions {
    /// Array of component names that should be considered the same as an `input` element.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub input_components: Option<Box<[Box<str>]>>,
    /// Array of attributes that should be treated as the `label` accessible text content.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub label_attributes: Option<Box<[Box<str>]>>,
    /// Array of component names that should be considered the same as a `label` element.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub label_components: Option<Box<[Box<str>]>>,
}

impl biome_deserialize::Merge for NoLabelWithoutControlOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(input_components) = other.input_components {
            self.input_components = Some(input_components);
        }
        if let Some(label_attributes) = other.label_attributes {
            self.label_attributes = Some(label_attributes);
        }
        if let Some(label_components) = other.label_components {
            self.label_components = Some(label_components);
        }
    }
}
