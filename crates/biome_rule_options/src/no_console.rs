use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoConsoleOptions {
    /// Allowed calls on the console object.
    #[serde(default, skip_serializing_if = "Option::<_>::is_none")]
    pub allow: Option<Box<[Box<str>]>>,
}

impl biome_deserialize::Merge for NoConsoleOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(allow) = other.allow {
            self.allow = Some(allow);
        }
    }
}
