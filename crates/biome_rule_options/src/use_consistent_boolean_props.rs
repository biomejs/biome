use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseConsistentBooleanPropsOptions {
    /// How to write JSX props whose value is `true`. Defaults to `explicit`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<BooleanPropMode>,
}

#[derive(
    Default, Clone, Copy, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum BooleanPropMode {
    Implicit,
    #[default]
    Explicit,
}
