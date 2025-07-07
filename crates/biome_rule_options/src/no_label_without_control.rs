use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoLabelWithoutControlOptions {
    /// Array of component names that should be considered the same as an `input` element.
    pub input_components: Box<[Box<str>]>,
    /// Array of attributes that should be treated as the `label` accessible text content.
    pub label_attributes: Box<[Box<str>]>,
    /// Array of component names that should be considered the same as a `label` element.
    pub label_components: Box<[Box<str>]>,
}
