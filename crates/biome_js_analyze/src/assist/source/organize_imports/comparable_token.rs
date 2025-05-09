use std::cmp::Ordering;

use biome_rowan::TokenText;
use biome_string_case::StrLikeExtension;

/// A [TokenText] that is ordered according to the ASCII natural order.
#[derive(Clone, Debug)]
pub struct ComparableToken(pub TokenText);
impl From<TokenText> for ComparableToken {
    fn from(value: TokenText) -> Self {
        Self(value)
    }
}
impl AsRef<str> for ComparableToken {
    fn as_ref(&self) -> &str {
        self.0.text()
    }
}
impl Eq for ComparableToken {}
impl PartialEq for ComparableToken {
    fn eq(&self, other: &Self) -> bool {
        self.0.text() == other.0.text()
    }
}
impl std::hash::Hash for ComparableToken {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.text().hash(state);
    }
}
impl Ord for ComparableToken {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.text().ascii_nat_cmp(other.0.text())
    }
}
impl PartialOrd for ComparableToken {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
