//! Decoding of TOML strings and keys.

use crate::{
    AnyTomlKeyPart, TomlBareKey, TomlQuotedKey, TomlString, TomlSyntaxKind, TomlSyntaxToken,
};
use biome_rowan::{SyntaxResult, TextRange, TextSize, TokenText};
use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Deref;

/// The decoded value of a TOML string or key.
///
/// The value borrows the token text unless decoding escape sequences required
/// an allocation. Equality and hashing compare the decoded text, so `"a"`,
/// `'a'`, `"a"`, and the bare key `a` are equal.
#[derive(Clone)]
pub enum TomlDecodedText {
    Borrowed(TokenText),
    Owned(Box<str>),
}

impl TomlDecodedText {
    pub fn text(&self) -> &str {
        match self {
            Self::Borrowed(text) => text,
            Self::Owned(text) => text,
        }
    }
}

impl Deref for TomlDecodedText {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.text()
    }
}

impl Borrow<str> for TomlDecodedText {
    fn borrow(&self) -> &str {
        self.text()
    }
}

impl PartialEq for TomlDecodedText {
    fn eq(&self, other: &Self) -> bool {
        self.text() == other.text()
    }
}

impl Eq for TomlDecodedText {}

impl PartialEq<&str> for TomlDecodedText {
    fn eq(&self, other: &&str) -> bool {
        self.text() == *other
    }
}

impl Hash for TomlDecodedText {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.text().hash(state);
    }
}

impl Debug for TomlDecodedText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.text(), f)
    }
}

impl Display for TomlDecodedText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.text(), f)
    }
}

impl TomlString {
    /// Returns the text between the delimiters. For multi-line strings, a
    /// newline that immediately follows the opening delimiter is excluded.
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        Ok(inner_string_text(&self.value_token()?))
    }

    /// Returns the value of the string with its escape sequences decoded.
    pub fn decoded_value(&self) -> SyntaxResult<TomlDecodedText> {
        Ok(decode_string_token(&self.value_token()?))
    }
}

impl TomlBareKey {
    pub fn name(&self) -> SyntaxResult<TokenText> {
        Ok(self.value_token()?.token_text_trimmed())
    }
}

impl TomlQuotedKey {
    /// Returns the name of the key with its quotes removed and escape
    /// sequences decoded.
    pub fn name(&self) -> SyntaxResult<TomlDecodedText> {
        Ok(decode_string_token(&self.value_token()?))
    }
}

impl AnyTomlKeyPart {
    /// Returns the name of this key part, or `None` if the key part is bogus or
    /// incomplete.
    pub fn name(&self) -> Option<TomlDecodedText> {
        match self {
            Self::TomlBareKey(key) => key.name().ok().map(TomlDecodedText::Borrowed),
            Self::TomlQuotedKey(key) => key.name().ok(),
            Self::TomlBogus(_) => None,
        }
    }
}

/// Returns the text of a string token between its delimiters.
///
/// For multi-line strings, a newline that immediately follows the opening
/// delimiter is excluded. Other tokens are returned without trivia.
pub fn inner_string_text(token: &TomlSyntaxToken) -> TokenText {
    let text = token.token_text_trimmed();
    let delimiter_len = match token.kind() {
        TomlSyntaxKind::TOML_BASIC_STRING_LITERAL | TomlSyntaxKind::TOML_LITERAL_STRING_LITERAL => {
            1
        }
        TomlSyntaxKind::TOML_ML_BASIC_STRING_LITERAL
        | TomlSyntaxKind::TOML_ML_LITERAL_STRING_LITERAL => 3,
        _ => return text,
    };

    let bytes = text.as_bytes();
    let len = bytes.len();
    // An unterminated string doesn't have a closing delimiter.
    let end = if len >= 2 * delimiter_len
        && bytes[len - delimiter_len..]
            .iter()
            .all(|byte| *byte == bytes[0])
    {
        len - delimiter_len
    } else {
        len
    };
    let end = TextSize::from(end as u32);

    let mut start = TextSize::from(delimiter_len as u32).min(end);
    if delimiter_len == 3 {
        let rest = &text[usize::from(start)..usize::from(end)];
        if rest.starts_with('\n') {
            start += TextSize::from(1);
        } else if rest.starts_with("\r\n") {
            start += TextSize::from(2);
        }
    }

    text.slice(TextRange::new(start, end))
}

/// Returns the decoded value of a string token: the text between its
/// delimiters with escape sequences of basic strings decoded.
pub fn decode_string_token(token: &TomlSyntaxToken) -> TomlDecodedText {
    let inner = inner_string_text(token);
    let is_basic = matches!(
        token.kind(),
        TomlSyntaxKind::TOML_BASIC_STRING_LITERAL | TomlSyntaxKind::TOML_ML_BASIC_STRING_LITERAL
    );

    if is_basic && inner.contains('\\') {
        TomlDecodedText::Owned(unescape_basic_string(&inner).into_boxed_str())
    } else {
        TomlDecodedText::Borrowed(inner)
    }
}

/// Decodes the escape sequences of the content of a basic string.
///
/// A backslash at the end of a line removes itself and all whitespace and
/// newlines that follow it. Invalid escape sequences are kept verbatim; the
/// parser reports them.
pub fn unescape_basic_string(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut rest = text;

    while let Some(index) = rest.find('\\') {
        result.push_str(&rest[..index]);
        let escape = &rest[index..];
        let mut chars = escape[1..].chars();

        let consumed = match chars.next() {
            Some('b') => push(&mut result, '\u{8}', 2),
            Some('t') => push(&mut result, '\t', 2),
            Some('n') => push(&mut result, '\n', 2),
            Some('f') => push(&mut result, '\u{c}', 2),
            Some('r') => push(&mut result, '\r', 2),
            Some('e') => push(&mut result, '\u{1b}', 2),
            Some('"') => push(&mut result, '"', 2),
            Some('\\') => push(&mut result, '\\', 2),
            Some('x') => push_code_point(&mut result, escape, 2),
            Some('u') => push_code_point(&mut result, escape, 4),
            Some('U') => push_code_point(&mut result, escape, 8),
            Some(' ' | '\t' | '\n' | '\r') => {
                let trimmed = escape[1..].trim_start_matches([' ', '\t', '\n', '\r']);
                let skipped = &escape[1..escape.len() - trimmed.len()];
                if skipped.contains('\n') {
                    escape.len() - trimmed.len()
                } else {
                    result.push('\\');
                    1
                }
            }
            _ => {
                result.push('\\');
                1
            }
        };

        rest = &escape[consumed..];
    }

    result.push_str(rest);
    result
}

fn push(result: &mut String, char: char, consumed: usize) -> usize {
    result.push(char);
    consumed
}

/// Decodes `\xHH`, `\uHHHH`, or `\UHHHHHHHH` at the start of `escape` and
/// returns the number of bytes consumed.
fn push_code_point(result: &mut String, escape: &str, digits: usize) -> usize {
    let decoded = escape
        .get(2..2 + digits)
        .filter(|hex| hex.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .and_then(|hex| u32::from_str_radix(hex, 16).ok())
        .and_then(char::from_u32);

    match decoded {
        Some(char) => push(result, char, 2 + digits),
        None => {
            result.push('\\');
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::unescape_basic_string;

    #[test]
    fn unescapes_simple_escapes() {
        assert_eq!(
            unescape_basic_string(r#"a\tb\n\"c\"\\\e\b\f\r"#),
            "a\tb\n\"c\"\\\u{1b}\u{8}\u{c}\r"
        );
    }

    #[test]
    fn unescapes_code_points() {
        assert_eq!(
            unescape_basic_string(r"\x41é\U0001F600"),
            "A\u{e9}\u{1F600}"
        );
    }

    #[test]
    fn trims_line_ending_backslash() {
        assert_eq!(unescape_basic_string("a \\  \n  \n\tb"), "a b");
        assert_eq!(unescape_basic_string("a\\\r\nb"), "ab");
    }

    #[test]
    fn keeps_invalid_escapes() {
        assert_eq!(unescape_basic_string(r"\q\u12"), r"\q\u12");
        assert_eq!(unescape_basic_string(r"\ x"), r"\ x");
        assert_eq!(unescape_basic_string(r"\uD800"), r"\uD800");
    }
}
