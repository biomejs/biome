use crate::restricted_regex::RestrictedRegex;
use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseValidTestTitleOptions {
    /// When `true`, leading and trailing whitespace in titles will not be checked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_spaces: Option<bool>,

    /// When `true`, non-string titles in `describe` and `suite` blocks will be allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_type_of_describe_name: Option<bool>,

    /// When `true`, non-string titles in `test` and `it` blocks will be allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_type_of_test_name: Option<bool>,

    /// A list of words that are disallowed in test titles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disallowed_words: Option<Box<[Box<str>]>>,

    /// Regular expressions that titles must not match.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub must_not_match: Option<Box<[RestrictedRegex]>>,

    /// Regular expressions that titles must match.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub must_match: Option<Box<[RestrictedRegex]>>,
}

impl UseValidTestTitleOptions {
    pub const DEFAULT_IGNORE_SPACES: bool = false;
    pub const DEFAULT_IGNORE_TYPE_OF_DESCRIBE_NAME: bool = false;
    pub const DEFAULT_IGNORE_TYPE_OF_TEST_NAME: bool = false;

    pub fn ignore_spaces(&self) -> bool {
        self.ignore_spaces.unwrap_or(Self::DEFAULT_IGNORE_SPACES)
    }

    pub fn ignore_type_of_describe_name(&self) -> bool {
        self.ignore_type_of_describe_name
            .unwrap_or(Self::DEFAULT_IGNORE_TYPE_OF_DESCRIBE_NAME)
    }

    pub fn ignore_type_of_test_name(&self) -> bool {
        self.ignore_type_of_test_name
            .unwrap_or(Self::DEFAULT_IGNORE_TYPE_OF_TEST_NAME)
    }
}
