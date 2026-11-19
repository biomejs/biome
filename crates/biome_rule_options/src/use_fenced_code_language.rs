//! Configuration for the `useFencedCodeLanguage` rule.

use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
/// Configures the language tags and info-string content accepted by fenced code blocks.
pub struct UseFencedCodeLanguageOptions {
    /// The language tags a fenced code block is allowed to declare.
    /// An empty list accepts every language tag.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub allowed_languages: Option<Box<[Box<str>]>>,
    /// Requires the info string to contain only a language tag, without metadata or other
    /// non-whitespace content.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub language_only: Option<bool>,
}

impl biome_deserialize::Merge for UseFencedCodeLanguageOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(allowed_languages) = other.allowed_languages {
            self.allowed_languages = Some(allowed_languages);
        }
        if let Some(language_only) = other.language_only {
            self.language_only = Some(language_only);
        }
    }
}
