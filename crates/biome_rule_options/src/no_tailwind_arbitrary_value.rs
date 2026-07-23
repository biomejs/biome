use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

/// Options for the `noTailwindArbitraryValue` rule.
///
/// Controls which attributes and utility functions are checked for arbitrary values.
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoTailwindArbitraryValueOptions {
    /// Additional attributes that will be checked.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub attributes: Option<Box<[Box<str>]>>,
    /// Names of the functions or tagged templates that will be checked.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub functions: Option<Box<[Box<str>]>>,
}
