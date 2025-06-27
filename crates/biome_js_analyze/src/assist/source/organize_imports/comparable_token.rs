use std::cmp::Ordering;

use biome_rowan::TokenText;
use biome_string_case::StrLikeExtension;
use biome_deserialize_macros::Deserializable;

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, serde::Deserialize, Deserializable, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum SortMode {
    #[default]
    Natural,
    Alphabetical,
}


/// A [TokenText] that is ordered according to the ASCII natural order.
#[derive(Clone, Debug)]
pub struct ComparableToken {
    pub token: TokenText,
    pub sort_mode: SortMode,
}
impl ComparableToken {
    pub fn new(token: TokenText, sort_mode: SortMode) -> Self {
        Self { token, sort_mode }
    }
}
impl AsRef<str> for ComparableToken {
    fn as_ref(&self) -> &str {
        self.token.text()
    }
}
impl Eq for ComparableToken {}
impl PartialEq for ComparableToken {
    fn eq(&self, other: &Self) -> bool {
        self.token.text() == other.token.text()
    }
}
impl std::hash::Hash for ComparableToken {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.token.text().hash(state);
    }
}
impl Ord for ComparableToken {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.sort_mode {
            SortMode::Natural => self.token.text().ascii_nat_cmp(other.token.text()),
            SortMode::Alphabetical => self.token.text().cmp(other.token.text()),
        }
    }
}
impl PartialOrd for ComparableToken {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
