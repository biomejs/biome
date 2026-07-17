use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoBaseToStringOptions {
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignored_type_names: Option<Box<[Box<str>]>>,
}

impl biome_deserialize::Merge for NoBaseToStringOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(ignored_type_names) = other.ignored_type_names {
            self.ignored_type_names = Some(ignored_type_names);
        }
    }
}
