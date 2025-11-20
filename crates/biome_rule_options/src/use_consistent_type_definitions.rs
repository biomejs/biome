use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseConsistentTypeDefinitionsOptions {
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub style: Option<ConsistentTypeDefinition>,
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum ConsistentTypeDefinition {
    /// Prefer using `interface` for object type definitions
    #[default]
    Interface,
    /// Prefer using `type` for object type definitions
    Type,
}

impl FromStr for ConsistentTypeDefinition {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "interface" => Ok(Self::Interface),
            "type" => Ok(Self::Type),
            _ => Err("Value not supported for consistent type definition style"),
        }
    }
}
