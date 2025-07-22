use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoConsoleOptions {
    /// Allowed calls on the console object.
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub allow: Box<[Box<str>]>,
}
