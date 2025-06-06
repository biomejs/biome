use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

use crate::TokenText;

/// Type that allows deserializing a string without heap-allocation when possible.
///
/// This is analogous to [std::borrow::Cow], except for strings.
#[derive(Clone, Debug)]
pub enum Text {
    Borrowed(TokenText),
    Owned(String),
    Static(&'static str),
}

impl Borrow<str> for Text {
    fn borrow(&self) -> &str {
        self.text()
    }
}

impl Default for Text {
    fn default() -> Self {
        Self::Owned(String::new())
    }
}

impl Deref for Text {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.text()
    }
}

impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

impl Eq for Text {}

impl From<TokenText> for Text {
    fn from(text: TokenText) -> Self {
        Self::Borrowed(text)
    }
}

impl From<&'static str> for Text {
    fn from(string: &'static str) -> Self {
        Self::Static(string)
    }
}

impl Hash for Text {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.text().hash(state);
    }
}

impl Ord for Text {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.text().cmp(other.text())
    }
}

impl PartialEq for Text {
    fn eq(&self, other: &Self) -> bool {
        self.text() == other.text()
    }
}

impl PartialEq<&'_ str> for Text {
    fn eq(&self, other: &&str) -> bool {
        self.text() == *other
    }
}

impl PartialOrd for Text {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<Text> for String {
    fn from(value: Text) -> Self {
        match value {
            Text::Borrowed(token_text) => token_text.to_string(),
            Text::Owned(string) => string,
            Text::Static(string) => string.to_string(),
        }
    }
}

impl From<Text> for Box<str> {
    fn from(value: Text) -> Self {
        Self::from(value.text())
    }
}

impl Text {
    pub fn text(&self) -> &str {
        match self {
            Self::Borrowed(token_text) => token_text.text(),
            Self::Owned(string) => string,
            Self::Static(string) => string,
        }
    }
}
