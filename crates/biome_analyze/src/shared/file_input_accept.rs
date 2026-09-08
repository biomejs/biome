//! Validation and normalization for file input `accept` attribute values.
//!
//! The attribute contains a comma-separated set of unique file type specifiers. Each specifier
//! is a filename extension beginning with `.`, a MIME type without parameters, or one of the
//! wildcards `audio/*`, `image/*`, and `video/*`.
//!
//! See the [HTML Standard](https://html.spec.whatwg.org/multipage/input.html#attr-input-accept)
//! for the complete syntax and browser behavior.

use biome_unicode_table::{Dispatch::*, lookup_byte};
use smallvec::SmallVec;

const ALLOWED_WILDCARD_MIME_TYPES: &[&str] = &["audio/*", "image/*", "video/*"];

/// How many tokens to store on the stack without allocating on the heap. It's an arbitrary guess.
const INLINE_TOKEN_CAPACITY: usize = 8;

/// The reason an `accept` value is invalid.
#[derive(Debug, PartialEq)]
pub enum InvalidAcceptValue {
    EmptyEntry,
    Extension,
    MimeType,
    WildcardMimeType,
}

impl InvalidAcceptValue {
    /// Returns diagnostic advice for the invalid value.
    pub const fn explanation(&self) -> &'static str {
        match self {
            Self::EmptyEntry => "Empty entries are not allowed.",
            Self::Extension => {
                "File extensions must start with . and cannot contain whitespace, commas, slashes, wildcards, or template markers."
            }
            Self::MimeType => "MIME types must use valid type/subtype syntax.",
            Self::WildcardMimeType => "Wildcard MIME types must be audio/*, image/*, or video/*.",
        }
    }
}

/// The classification of an `accept` value.
#[derive(Debug, PartialEq)]
pub enum AcceptValueClassification {
    /// The value is valid and already uses its canonical form.
    Valid,
    /// The value contains a file type specifier that normalization cannot repair.
    Invalid(InvalidAcceptValue),
    /// Normalization can produce a valid, canonical form for the value.
    NeedsNormalization,
}

/// A file input `accept` attribute value.
///
/// Normalization:
/// - deduplicates items
/// - lowercases them
/// - splits items with `, ` which removes extra whitespace
/// - replaces legacy mime types with their modern equivalents
/// - makes file extentions start with `.`
pub struct FileInputAcceptValue<'a> {
    source: &'a str,
}

impl<'a> FileInputAcceptValue<'a> {
    /// Creates an `accept` value from its source text.
    pub const fn new(source: &'a str) -> Self {
        Self { source }
    }

    /// Classifies the value as canonical, normalizable, or invalid.
    pub fn classify(&self) -> AcceptValueClassification {
        let mut needs_normalization = false;
        let mut seen_tokens = SmallVec::<[CanonicalAcceptToken<'_>; INLINE_TOKEN_CAPACITY]>::new();

        for (index, raw_token) in self.source.split(',').enumerate() {
            let token = raw_token.trim();
            if token.is_empty() {
                return AcceptValueClassification::Invalid(InvalidAcceptValue::EmptyEntry);
            }

            let canonical = match Self::canonical_token(token) {
                Ok(canonical) => canonical,
                Err(error) => return AcceptValueClassification::Invalid(error),
            };
            let has_canonical_spacing = if index == 0 {
                raw_token == token
            } else {
                raw_token.strip_prefix(' ') == Some(token)
            };
            let is_duplicate = seen_tokens.contains(&canonical);
            needs_normalization |= !has_canonical_spacing || !canonical.eq(token) || is_duplicate;
            if !is_duplicate {
                seen_tokens.push(canonical);
            }
        }

        if needs_normalization {
            AcceptValueClassification::NeedsNormalization
        } else {
            AcceptValueClassification::Valid
        }
    }

    /// Returns the canonical form, or an error if the value is invalid.
    pub fn normalized(&self) -> Result<String, InvalidAcceptValue> {
        let mut output = String::with_capacity(self.source.len());
        let mut seen_tokens = SmallVec::<[CanonicalAcceptToken<'_>; INLINE_TOKEN_CAPACITY]>::new();

        for raw_token in self.source.split(',') {
            let token = Self::canonical_token(raw_token.trim())?;
            if seen_tokens.contains(&token) {
                continue;
            }
            if !output.is_empty() {
                output.push_str(", ");
            }
            token.push_to(&mut output);
            seen_tokens.push(token);
        }

        Ok(output)
    }

    /// Parses one token and records its normalized form.
    fn canonical_token(token: &str) -> Result<CanonicalAcceptToken<'_>, InvalidAcceptValue> {
        if token.starts_with('.') {
            Self::canonical_extension(token)
        } else if token.contains('/') {
            Self::canonical_mime_type(token)
        } else {
            Self::canonical_bare_extension(token)
        }
    }

    fn canonical_extension(token: &str) -> Result<CanonicalAcceptToken<'_>, InvalidAcceptValue> {
        if token.strip_prefix('.').is_some_and(|extension| {
            !extension.is_empty() && !Self::has_invalid_extension_character(extension)
        }) {
            Ok(CanonicalAcceptToken {
                value: token,
                prefix_dot: false,
            })
        } else {
            Err(InvalidAcceptValue::Extension)
        }
    }

    fn canonical_bare_extension(
        token: &str,
    ) -> Result<CanonicalAcceptToken<'_>, InvalidAcceptValue> {
        if !token.is_empty() && !Self::has_invalid_extension_character(token) {
            Ok(CanonicalAcceptToken {
                value: token,
                prefix_dot: true,
            })
        } else {
            Err(InvalidAcceptValue::Extension)
        }
    }

    fn has_invalid_extension_character(value: &str) -> bool {
        value.chars().any(|character| {
            character.is_whitespace() || matches!(character, ',' | '/' | '*' | '$' | '{' | '}')
        })
    }

    fn canonical_mime_type(token: &str) -> Result<CanonicalAcceptToken<'_>, InvalidAcceptValue> {
        let essence = token
            .split_once(';')
            .map_or(token, |(essence, _)| essence)
            .trim();
        if essence.contains('*') {
            return ALLOWED_WILDCARD_MIME_TYPES
                .iter()
                .any(|expected| essence.eq_ignore_ascii_case(expected))
                .then_some(CanonicalAcceptToken {
                    value: essence,
                    prefix_dot: false,
                })
                .ok_or(InvalidAcceptValue::WildcardMimeType);
        }

        let Some((media_type, subtype)) = essence.split_once('/') else {
            return Err(InvalidAcceptValue::MimeType);
        };
        if media_type.is_empty()
            || subtype.is_empty()
            || subtype.contains('/')
            || !media_type.bytes().all(Self::is_http_token_byte)
            || !subtype.bytes().all(Self::is_http_token_byte)
        {
            return Err(InvalidAcceptValue::MimeType);
        }

        let value = if essence.eq_ignore_ascii_case("application/x-rar-compressed") {
            "application/vnd.rar"
        } else if essence.eq_ignore_ascii_case("application/x-zip-compressed") {
            "application/zip"
        } else if essence.eq_ignore_ascii_case("image/jpg") {
            "image/jpeg"
        } else if essence.eq_ignore_ascii_case("image/svg") {
            "image/svg+xml"
        } else if essence.eq_ignore_ascii_case("image/x-icon") {
            "image/vnd.microsoft.icon"
        } else {
            essence
        };
        Ok(CanonicalAcceptToken {
            value,
            prefix_dot: false,
        })
    }

    fn is_http_token_byte(byte: u8) -> bool {
        matches!(
            lookup_byte(byte),
            IDT | ZER
                | DIG
                | EXL
                | HAS
                | DOL
                | PRC
                | AMP
                | MUL
                | PLS
                | MIN
                | PRD
                | CRT
                | TPL
                | PIP
                | TLD
        ) || byte == b'\''
    }
}

struct CanonicalAcceptToken<'a> {
    value: &'a str,
    prefix_dot: bool,
}

impl CanonicalAcceptToken<'_> {
    fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.prefix_dot
            .then_some('.')
            .into_iter()
            .chain(self.value.chars().flat_map(char::to_lowercase))
    }

    fn push_to(&self, output: &mut String) {
        output.extend(self.chars());
    }
}

impl PartialEq for CanonicalAcceptToken<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.chars().eq(other.chars())
    }
}

impl PartialEq<str> for CanonicalAcceptToken<'_> {
    fn eq(&self, other: &str) -> bool {
        self.chars().eq(other.chars())
    }
}

#[cfg(test)]
mod tests {
    use super::{AcceptValueClassification, FileInputAcceptValue, InvalidAcceptValue};

    #[test]
    fn accepts_normalized_values() {
        assert_eq!(
            FileInputAcceptValue::new("image/png, .png, audio/*").classify(),
            AcceptValueClassification::Valid
        );
    }

    #[test]
    fn normalizes_values() {
        let value = "IMAGE/JPG,.PNG, image/jpeg";
        assert_eq!(
            FileInputAcceptValue::new(value).classify(),
            AcceptValueClassification::NeedsNormalization
        );
        assert_eq!(
            FileInputAcceptValue::new(value).normalized(),
            Ok("image/jpeg, .png".to_string())
        );

        let value = "image/svg; charset=utf-8";
        assert_eq!(
            FileInputAcceptValue::new(value).classify(),
            AcceptValueClassification::NeedsNormalization
        );
        assert_eq!(
            FileInputAcceptValue::new(value).normalized(),
            Ok("image/svg+xml".to_string())
        );

        let value = "image/png;foo=*";
        assert_eq!(
            FileInputAcceptValue::new(value).classify(),
            AcceptValueClassification::NeedsNormalization
        );
        assert_eq!(
            FileInputAcceptValue::new(value).normalized(),
            Ok("image/png".to_string())
        );
    }

    #[test]
    fn rejects_invalid_values() {
        assert_eq!(
            FileInputAcceptValue::new("image/png,").classify(),
            AcceptValueClassification::Invalid(InvalidAcceptValue::EmptyEntry)
        );
        assert_eq!(
            FileInputAcceptValue::new("text/*").classify(),
            AcceptValueClassification::Invalid(InvalidAcceptValue::WildcardMimeType)
        );
        assert_eq!(
            FileInputAcceptValue::new("image//png").classify(),
            AcceptValueClassification::Invalid(InvalidAcceptValue::MimeType)
        );
    }

    #[test]
    fn detects_duplicates_beyond_inline_capacity() {
        let value = ".a, .b, .c, .d, .e, .f, .g, .h, .i, .A";
        assert_eq!(
            FileInputAcceptValue::new(value).classify(),
            AcceptValueClassification::NeedsNormalization
        );
        assert_eq!(
            FileInputAcceptValue::new(value).normalized(),
            Ok(".a, .b, .c, .d, .e, .f, .g, .h, .i".to_string())
        );
    }
}
