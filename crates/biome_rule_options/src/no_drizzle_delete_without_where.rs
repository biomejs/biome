use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoDrizzleDeleteWithoutWhereOptions {
    /// List of variable names to consider as Drizzle ORM instances.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub drizzle_object_name: Option<Box<[Box<str>]>>,
}

impl biome_deserialize::Merge for NoDrizzleDeleteWithoutWhereOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(drizzle_object_name) = other.drizzle_object_name {
            self.drizzle_object_name = Some(drizzle_object_name);
        }
    }
}
