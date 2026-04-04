use std::str::FromStr;

use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseInputNameOptions {
    /// Check that the input type name follows the convention <mutationName>Input
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub check_input_type: Option<CheckInputType>,
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum CheckInputType {
    /// Don't check the input type
    #[default]
    Off,
    /// Check the input type (case-insensitive)
    Loose,
    /// Check the input type (case-sensitive)
    Strict,
}

impl FromStr for CheckInputType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "off" => Ok(Self::Off),
            "loose" => Ok(Self::Loose),
            "strict" => Ok(Self::Strict),
            _ => Err("Value not supported, expected 'off', 'loose' or 'strict'"),
        }
    }
}
