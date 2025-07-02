use std::cmp::Ordering;

use biome_rowan::TokenText;

use crate::StrLikeExtension;

/// A [TokenText] that is ordered according to the ASCII natural order using [crate::CldrAsciiCollator].
#[derive(Clone, Debug)]
pub struct ComparableToken {
    pub token: TokenText,
}
impl ComparableToken {
    pub const fn new(token: TokenText) -> Self {
        Self { token }
    }

    pub fn ascii_nat_cmp(&self, other: &Self) -> Ordering {
        self.token.text().ascii_nat_cmp(other.token.text())
    }

    pub fn lexicographic_cmp(&self, other: &Self) -> Ordering {
        self.token.text().lexicographic_cmp(other.token.text())
    }
}
impl From<TokenText> for ComparableToken {
    fn from(value: TokenText) -> Self {
        Self::new(value)
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
        self.token.hash(state);
    }
}
impl Ord for ComparableToken {
    fn cmp(&self, other: &Self) -> Ordering {
        self.token.text().ascii_nat_cmp(other.token.text())
    }
}
impl PartialOrd for ComparableToken {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
