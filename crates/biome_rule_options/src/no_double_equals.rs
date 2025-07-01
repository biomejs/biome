use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoDoubleEqualsOptions {
    /// If `true`, an exception is made when comparing with `null`, as it's often relied on to check
    /// both for `null` or `undefined`.
    ///
    /// If `false`, no such exception will be made.
    #[serde(
        default = "ignore_null_default",
        skip_serializing_if = "is_ignore_null_default"
    )]
    pub ignore_null: bool,
}

impl Default for NoDoubleEqualsOptions {
    fn default() -> Self {
        Self {
            ignore_null: ignore_null_default(),
        }
    }
}

fn ignore_null_default() -> bool {
    true
}

fn is_ignore_null_default(value: &bool) -> bool {
    value == &ignore_null_default()
}
