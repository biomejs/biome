use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseConsistentArrayTypeOptions {
    pub syntax: ConsistentArrayType,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum ConsistentArrayType {
    /// `ItemType[]`
    #[default]
    Shorthand,
    /// `Array<ItemType>`
    Generic,
}

impl FromStr for ConsistentArrayType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "shorthand" => Ok(Self::Shorthand),
            "generic" => Ok(Self::Generic),
            _ => Err("Value not supported for array type syntax"),
        }
    }
}
