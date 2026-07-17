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

impl NoLabelWithoutControlOptions {
    const DEFAULT_INPUT_COMPONENTS: [&str; 7] = [
        "input", "meter", "output", "progress", "select", "textarea", "button",
    ];
    const DEFAULT_LABEL_ATTRIBUTES: [&str; 3] = ["aria-label", "aria-labelledby", "alt"];
    const DEFAULT_LABEL_COMPONENTS: [&str; 1] = ["label"];

    /// Returns [`Self::input_components`] merged with [`Self::DEFAULT_INPUT_COMPONENTS`].
    pub fn input_components(&self) -> Vec<&str> {
        let mut result = Self::DEFAULT_INPUT_COMPONENTS.to_vec();
        if let Some(input) = &self.input_components {
            result.extend(input.iter().map(|s| s.as_ref()));
        }
        result
    }

    /// Returns [`Self::label_attributes`] merged with [`Self::DEFAULT_LABEL_ATTRIBUTES`].
    pub fn label_attributes(&self) -> Vec<&str> {
        let mut result = Self::DEFAULT_LABEL_ATTRIBUTES.to_vec();
        if let Some(input) = &self.label_attributes {
            result.extend(input.iter().map(|s| s.as_ref()));
        }
        result
    }

    /// Returns [`Self::label_components`] merged with [`Self::DEFAULT_LABEL_COMPONENTS`].
    pub fn label_components(&self) -> Vec<&str> {
        let mut result = Self::DEFAULT_LABEL_COMPONENTS.to_vec();
        if let Some(input) = &self.label_components {
            result.extend(input.iter().map(|s| s.as_ref()));
        }
        result
    }
}
