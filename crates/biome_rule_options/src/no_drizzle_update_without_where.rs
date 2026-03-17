use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoDrizzleUpdateWithoutWhereOptions {
    /// List of variable names to consider as Drizzle ORM instances.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub drizzle_object_name: Vec<String>,
}
