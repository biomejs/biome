use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseFencedCodeLanguageOptions {
    /// The list of languages a fenced code block is allowed to declare.
    /// When empty, any language is accepted.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub allowed_languages: Option<Box<[Box<str>]>>,
    /// Require the info string to contain exactly the language, without surrounding whitespace
    /// or other content.
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
