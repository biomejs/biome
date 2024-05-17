use std::ops::Deref;

use biome_deserialize_macros::Deserializable;

/// A restricted regular expression only supports widespread syntaxes:
///
/// - Greedy quantifiers `*`, `?`, `+`, `{n}`, `{n,m}`, `{n,}`, `{m}`
/// - Non-greedy quantifiers `*?`, `??`, `+?`, `{n}?`, `{n,m}?`, `{n,}?`, `{m}?`
/// - Any character matcher `.`
/// - Character classes `[a-z]`, `[xyz]`, `[^a-z]`
/// - Alternations `|`
/// - Capturing groups `()`
/// - Non-capturing groups `(?:)`
/// - A limited set of escaped characters including all regex special characters
///   and regular string escape characters `\f`, `\n`, `\r`, `\t`, `\v`
///
/// A restricted regular expression is implictly delimited by the anchors `^` and `$`.
#[derive(Clone, Debug, Deserializable, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct RestrictedRegex(regex::Regex);
impl RestrictedRegex {
    /// Similar to [regex::Regex::as_str], but returns the original regex representation,
    /// without the implicit anchors and the implicit group.
    pub fn as_source(&self) -> &str {
        let repr = self.0.as_str();
        debug_assert!(repr.starts_with("^(?:"));
        debug_assert!(repr.ends_with(")$"));
        &repr[4..(repr.len() - 2)]
    }
}
impl From<RestrictedRegex> for String {
    fn from(value: RestrictedRegex) -> Self {
        value.into()
    }
}
impl Deref for RestrictedRegex {
    type Target = regex::Regex;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl TryFrom<String> for RestrictedRegex {
    type Error = regex::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        is_restricted_regex(&value)?;
        regex::Regex::new(&format!("^(?:{value})$")).map(RestrictedRegex)
    }
}
impl TryFrom<&str> for RestrictedRegex {
    type Error = regex::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        is_restricted_regex(value)?;
        regex::Regex::new(&format!("^(?:{value})$")).map(RestrictedRegex)
    }
}
#[cfg(feature = "schemars")]
impl schemars::JsonSchema for RestrictedRegex {
    fn schema_name() -> String {
        "Regex".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
}
impl Eq for RestrictedRegex {}
impl PartialEq for RestrictedRegex {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_str() == other.0.as_str()
    }
}

/// Rteurns an error if `pattern` doesn't follow the restricted regular expression syntax.
fn is_restricted_regex(pattern: &str) -> Result<(), regex::Error> {
    let mut it = pattern.bytes();
    let mut is_in_char_class = false;
    while let Some(c) = it.next() {
        match c {
            b'\\' => {
                // Accept a restrictive set of escape sequence
                // We keep only escaped chars that behave identically
                // in unicode-enabled and unicode-disabled RegExes.
                if let Some(c) = it.next() {
                    if !matches!(
                        c,
                        b'^' | b'|'
                            | b'*'
                            | b'?'
                            | b'{'
                            | b'}'
                            | b'['
                            | b']'
                            | b'-'
                            | b'$'
                            | b'f'
                            | b'n'
                            | b'r'
                            | b't'
                            | b'v'
                    ) {
                        // Escape sequences https://docs.rs/regex/latest/regex/#escape-sequences
                        // and Perl char classes https://docs.rs/regex/latest/regex/#perl-character-classes-unicode-friendly
                        return Err(regex::Error::Syntax(format!(
                            "Escape sequence \\{c} is not supported."
                        )));
                    }
                } else {
                    return Err(regex::Error::Syntax(
                        r"`\` should be followed by a character.".to_string(),
                    ));
                }
            }
            b'^' | b'$' if !is_in_char_class => {
                // Anchors are implicit and always present in a restricted regex
                return Err(regex::Error::Syntax(format!(
                    "The anchor \\{c} is not supported. It is implciitly present."
                )));
            }
            b'[' if is_in_char_class => {
                return Err(regex::Error::Syntax(
                    "Nested character class are not supported.".to_string(),
                ));
            }
            b'[' => {
                is_in_char_class = true;
            }
            b']' => {
                is_in_char_class = false;
            }
            b'&' | b'~' | b'-' if is_in_char_class => {
                if it.next() == Some(c) {
                    return Err(regex::Error::Syntax(format!(
                        "Character class operator {c}{c} is not supported."
                    )));
                }
            }
            b'(' if !is_in_char_class => {
                if it.next() == Some(b'?') {
                    match it.next() {
                        Some(b'P' | b'=' | b'!' | b'<') => {
                            return if c == b'P'
                                || (c == b'<' && !matches!(it.next(), Some(b'=' | b'!')))
                            {
                                Err(regex::Error::Syntax(
                                    "Named groups `(?<NAME>)` are not supported.".to_string(),
                                ))
                            } else {
                                Err(regex::Error::Syntax(format!(
                                    "Assertions `(?{c})` are not supported."
                                )))
                            };
                        }
                        Some(b':') => {}
                        _ => {
                            return Err(regex::Error::Syntax(
                                "Group flags `(?flags:)` are not supported.".to_string(),
                            ));
                        }
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(is_restricted_regex("^a").is_err());
        assert!(is_restricted_regex("a$").is_err());
        assert!(is_restricted_regex(r"\").is_err());
        assert!(is_restricted_regex(r"\p{L}").is_err());
        assert!(is_restricted_regex(r"(?i:)").is_err());
        assert!(is_restricted_regex(r"(?=a)").is_err());
        assert!(is_restricted_regex(r"(?!a)").is_err());
        assert!(is_restricted_regex(r"(?<NAME>:a)").is_err());
        assert!(is_restricted_regex(r"[[:digit:]]").is_err());
        assert!(is_restricted_regex(r"[a[bc]d]").is_err());
        assert!(is_restricted_regex(r"[ab--a]").is_err());
        assert!(is_restricted_regex(r"[ab&&a]").is_err());
        assert!(is_restricted_regex(r"[ab~~a]").is_err());

        assert!(is_restricted_regex("").is_ok());
        assert!(is_restricted_regex("abc").is_ok());
        assert!(is_restricted_regex("(?:a)(.+)z").is_ok());
        assert!(is_restricted_regex("[A-Z][^a-z]").is_ok());
        assert!(is_restricted_regex(r"\n\t\v\f").is_ok());
    }
}
