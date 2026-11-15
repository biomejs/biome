use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
use std::num::NonZeroU16;
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseTopLevelDescribeOptions {
    /// The maximum number of `describe` blocks allowed at the top level of a file.
    ///
    /// When this option isn't set, any number of top-level `describe` blocks is allowed.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub max_top_level_describes: Option<NonZeroU16>,
}
