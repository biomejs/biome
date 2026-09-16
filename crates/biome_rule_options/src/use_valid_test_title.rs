use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseValidTestTitleOptions {
    /// A list of words that are disallowed in test titles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disallowed_words: Option<Box<[Box<str>]>>,
}
