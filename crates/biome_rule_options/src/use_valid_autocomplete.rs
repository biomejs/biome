use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseValidAutocompleteOptions {
    /// `input` like custom components that should be checked.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub input_components: Option<Box<[Box<str>]>>,
}

impl biome_deserialize::Merge for UseValidAutocompleteOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(input_components) = other.input_components {
            self.input_components = Some(input_components);
        }
    }
}
