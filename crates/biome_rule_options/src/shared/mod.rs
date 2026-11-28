use biome_string_case::StrLikeExtension;

pub mod restricted_regex;
pub mod sort_order;
pub mod tailwind_utility_category;

const REDUNDANT_WORDS: [&str; 3] = ["image", "photo", "picture"];

pub fn is_redundant_alt(alt: &str) -> bool {
    REDUNDANT_WORDS.into_iter().any(|word| {
        alt.split_whitespace()
            .any(|x| x.to_ascii_lowercase_cow() == word)
    })
}

// Options shared by every lint rule that has nothing to configure.
//
// This isn't a doc comment because the JSON schema would repeat it as the
// description of the options of each of these rules.
#[derive(
    Default,
    Clone,
    Debug,
    serde::Deserialize,
    biome_deserialize_macros::Deserializable,
    biome_deserialize_macros::Merge,
    Eq,
    PartialEq,
    serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct EmptyRuleOptions {}
