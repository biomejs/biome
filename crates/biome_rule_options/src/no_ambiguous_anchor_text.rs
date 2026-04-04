use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoAmbiguousAnchorTextOptions {
    /// It allows users to modify the strings that can be checked for in the anchor text. Useful for specifying other words in other languages
    pub words: Option<Vec<String>>,
}

impl NoAmbiguousAnchorTextOptions {
    pub const DEFAULT_AMBIGUOUS_WORDS: [&str; 5] =
        ["click here", "here", "link", "a link", "learn more"];

    /// Returns [`Self::words`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_AMBIGUOUS_WORDS`].
    pub fn words(&self) -> Vec<String> {
        self.words.clone().unwrap_or_else(|| {
            Self::DEFAULT_AMBIGUOUS_WORDS
                .iter()
                .map(|s| (*s).to_string())
                .collect()
        })
    }
}
