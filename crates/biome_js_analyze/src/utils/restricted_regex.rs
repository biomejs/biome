use std::{ops::Deref, str::FromStr};

use biome_deserialize::{DeserializableValue, DeserializationContext, DeserializationDiagnostic};
use biome_rowan::{TextRange, TextSize};

/// A restricted regular expression only supports widespread syntaxes:
///
/// - Greedy quantifiers `*`, `?`, `+`, `{n}`, `{n,m}`, `{n,}`, `{m}`
/// - Non-greedy quantifiers `*?`, `??`, `+?`, `{n}?`, `{n,m}?`, `{n,}?`, `{m}?`
/// - Any character matcher `.`
/// - Character classes `[a-z]`, `[xyz]`, `[^a-z]`
/// - Alternations `|`
/// - Capturing groups `()`
/// - Non-capturing groups `(?:)`
/// - Non-capturing groups with flags `(?flags:)` and negated flags `(?-flags:)`
///   Supported flags:
///   - `i`: ignore case
///   - `m`: multiline mode
///   - `s`: single line mode (`.` matches also `\n`)
/// - A limited set of escaped characters including all regex special characters
///   and regular string escape characters `\f`, `\n`, `\r`, `\t`, `\v`
///
/// A restricted regular expression is implicitly delimited by the anchors `^` and `$`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct RestrictedRegex(regex::Regex);

impl RestrictedRegex {
    /// Returns the original string of this regex.
    pub fn as_str(&self) -> &str {
        let repr = self.0.as_str();
        debug_assert!(repr.starts_with("^(?:"));
        debug_assert!(repr.ends_with(")$"));
        &repr[4..(repr.len() - 2)]
    }
}

impl Deref for RestrictedRegex {
    type Target = regex::Regex;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for RestrictedRegex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<RestrictedRegex> for String {
    fn from(value: RestrictedRegex) -> Self {
        value.to_string()
    }
}

impl FromStr for RestrictedRegex {
    type Err = RestrictedRegexError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        validate_restricted_regex(value)?;
        regex::Regex::new(&format!("^(?:{value})$"))
            .map(RestrictedRegex)
            .map_err(|error| RestrictedRegexError { error, index: None })
    }
}

impl TryFrom<String> for RestrictedRegex {
    type Error = RestrictedRegexError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

// We use a custom impl to precisely report the location of the error.
impl biome_deserialize::Deserializable for RestrictedRegex {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        let regex = String::deserialize(ctx, value, name)?;
        match regex.parse() {
            Ok(regex) => Some(regex),
            Err(error) => {
                let range = value.range();
                let range = error.index().map_or(range, |index| {
                    TextRange::at(range.start() + TextSize::from(1 + index), 1u32.into())
                });
                ctx.report(
                    DeserializationDiagnostic::new(format_args!("{error}")).with_range(range),
                );
                None
            }
        }
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

#[derive(Debug)]
pub struct RestrictedRegexError {
    error: regex::Error,
    index: Option<u32>,
}
impl RestrictedRegexError {
    fn new(error: regex::Error, index: usize) -> Self {
        Self {
            error,
            index: Some(index as u32),
        }
    }

    /// Returns the index in the glob where the error is located.
    pub fn index(&self) -> Option<u32> {
        self.index
    }
}
impl std::error::Error for RestrictedRegexError {}
impl std::fmt::Display for RestrictedRegexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.error.fmt(f)
    }
}

/// Returns an error if `pattern` doesn't follow the restricted regular expression syntax.
fn validate_restricted_regex(pattern: &str) -> Result<(), RestrictedRegexError> {
    let mut it = pattern.bytes().enumerate();
    let mut is_in_char_class = false;
    while let Some((i, c)) = it.next() {
        match c {
            b'\\' => {
                // Accept a restrictive set of escape sequence
                // We keep only escaped chars that behave identically
                // in unicode-enabled and unicode-disabled RegExes.
                if let Some((_, c)) = it.next() {
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
                            | b'\\'
                    ) {
                        if c.is_ascii() {
                            // SAFETY: `c` is ASCIIaccording to the conditional
                            let c = c as char;
                            // Escape sequences https://docs.rs/regex/latest/regex/#escape-sequences
                            // and Perl char classes https://docs.rs/regex/latest/regex/#perl-character-classes-unicode-friendly
                            return Err(RestrictedRegexError::new(
                                regex::Error::Syntax(format!(
                                    "Escape sequence \\{c} is not supported."
                                )),
                                i,
                            ));
                        } else {
                            return Err(RestrictedRegexError::new(
                                regex::Error::Syntax(
                                    "Escape sequence cannot contain a multi-byte character."
                                        .to_string(),
                                ),
                                i,
                            ));
                        }
                    }
                } else {
                    return Err(RestrictedRegexError::new(
                        regex::Error::Syntax(r"`\` should be followed by a character.".to_string()),
                        i,
                    ));
                }
            }
            b'^' | b'$' if !is_in_char_class => {
                // Anchors are implicit and always present in a restricted regex
                return Err(RestrictedRegexError::new(
                    regex::Error::Syntax(
                        "Anchors `^` and `$` are not supported. They are implciitly present."
                            .to_string(),
                    ),
                    i,
                ));
            }
            b'[' if is_in_char_class => {
                return Err(RestrictedRegexError::new(
                    regex::Error::Syntax("Nested character class are not supported.".to_string()),
                    i,
                ));
            }
            b'[' => {
                is_in_char_class = true;
            }
            b']' => {
                is_in_char_class = false;
            }
            b'&' | b'~' | b'-' if is_in_char_class => {
                if it.next().is_some_and(|(_, x)| x == c) {
                    return Err(RestrictedRegexError::new(
                        regex::Error::Syntax(
                            "Character class operator `&&`, `~~`, `--` are not supported."
                                .to_string(),
                        ),
                        i,
                    ));
                }
            }
            b'(' if !is_in_char_class => {
                let mut lookahead = it.clone();
                if let Some((_, b'?')) = lookahead.next() {
                    it.next();
                    match it.next() {
                        Some((i, b'P' | b'=' | b'!' | b'<')) => {
                            return if c == b'P'
                                || (c == b'<' && !matches!(it.next(), Some((_, b'=' | b'!'))))
                            {
                                Err(RestrictedRegexError::new(
                                    regex::Error::Syntax(
                                        "Named groups `(?<NAME>)` are not supported.".to_string(),
                                    ),
                                    i,
                                ))
                            } else {
                                Err(RestrictedRegexError::new(regex::Error::Syntax(
                                "Assertions `(?P)`, `(?=)`, `(?!)`,`(?<)` are not supported."
                                    .to_string(),
                            ), i))
                            };
                        }
                        Some((_, b':')) => {}
                        c => {
                            let mut current = c;
                            while matches!(current, Some((_, b'i' | b'm' | b's' | b'-'))) {
                                current = it.next()
                            }
                            match current {
                                Some((_, b':')) => {}
                                Some((_, b')')) => {
                                    return Err(RestrictedRegexError::new(
                                        regex::Error::Syntax(
                                            "Group modifiers `(?flags)` are not supported."
                                                .to_string(),
                                        ),
                                        i,
                                    ));
                                }
                                Some((i, c)) if c.is_ascii() => {
                                    // SAFETY: `c` is ASCII according to the guard
                                    let c = c as char;
                                    return Err(RestrictedRegexError::new(
                                        regex::Error::Syntax(format!(
                                            "Group flags `(?{c}:)` are not supported."
                                        )),
                                        i,
                                    ));
                                }
                                _ => {
                                    return Err(RestrictedRegexError::new(
                                        regex::Error::Syntax(
                                            "Unterminated non-capturing group.".to_string(),
                                        ),
                                        i,
                                    ));
                                }
                            }
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
    fn test_validate_restricted_regex() {
        assert!(validate_restricted_regex("^a").is_err());
        assert!(validate_restricted_regex("a$").is_err());
        assert!(validate_restricted_regex(r"\").is_err());
        assert!(validate_restricted_regex(r"\p{L}").is_err());
        assert!(validate_restricted_regex(r"\😀").is_err());
        assert!(validate_restricted_regex(r"(?=a)").is_err());
        assert!(validate_restricted_regex(r"(?!a)").is_err());
        assert!(validate_restricted_regex(r"(?<NAME>:a)").is_err());
        assert!(validate_restricted_regex(r"[[:digit:]]").is_err());
        assert!(validate_restricted_regex(r"[a[bc]d]").is_err());
        assert!(validate_restricted_regex(r"[ab--a]").is_err());
        assert!(validate_restricted_regex(r"[ab&&a]").is_err());
        assert!(validate_restricted_regex(r"[ab~~a]").is_err());

        assert!(validate_restricted_regex("").is_ok());
        assert!(validate_restricted_regex("abc").is_ok());
        assert!(validate_restricted_regex("(?:a)(.+)z").is_ok());
        assert!(validate_restricted_regex("(?ims:a)(.+)z").is_ok());
        assert!(validate_restricted_regex("(?-ims:a)(.+)z").is_ok());
        assert!(validate_restricted_regex("(?i-ms:a)(.+)z").is_ok());
        assert!(validate_restricted_regex("[A-Z][^a-z]").is_ok());
        assert!(validate_restricted_regex(r"\n\t\v\f").is_ok());
        assert!(validate_restricted_regex("([^_])").is_ok());
        assert!(validate_restricted_regex(r"(\$)").is_ok());
    }
}
