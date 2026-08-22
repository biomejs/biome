use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseSortedInterfaceMembersOptions {
    /// When enabled, members separated by a blank line are kept in their own
    /// section and sorted only within that section. This preserves logical
    /// groupings the author intentionally introduced with empty lines.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub partition_by_new_line: Option<bool>,
}
