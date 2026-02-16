use biome_string_case::StrLikeExtension;

pub mod restricted_regex;
pub mod sort_order;

const REDUNDANT_WORDS: [&str; 3] = ["image", "photo", "picture"];

pub fn is_redundant_alt(alt: &str) -> bool {
    REDUNDANT_WORDS.into_iter().any(|word| {
        alt.split_whitespace()
            .any(|x| x.to_ascii_lowercase_cow() == word)
    })
}
