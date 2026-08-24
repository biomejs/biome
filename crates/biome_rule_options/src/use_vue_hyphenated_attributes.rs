use biome_deserialize_macros::{Deserializable, Merge};
use rustc_hash::FxHashSet;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseVueHyphenatedAttributesOptions {
    /// List of attribute names to ignore when checking for uppercase letters.
    pub ignore: Option<FxHashSet<String>>,

    /// List of HTML tags whose attributes should not be checked for uppercase letters.
    pub ignore_tags: Option<FxHashSet<String>>,
}
