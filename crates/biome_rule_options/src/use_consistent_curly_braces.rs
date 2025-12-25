use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseConsistentCurlyBracesOptions {
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub props: Option<CurlyBracesBehavior>,
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub children: Option<CurlyBracesBehavior>,
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum CurlyBracesBehavior {
    Always,
    #[default]
    Never,
    Ignore,
}
