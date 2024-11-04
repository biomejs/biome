use biome_rowan::{TextRange, TextSize};

/// A restricted glob pattern only supports the following syntaxes:
///
/// - star `*` that matches zero or more characters inside a path segment
/// - globstar `**` that matches zero or more path segments
/// - Use `\*` to escape `*`
/// - `?`, `[`, `]`, `{`, and `}` must be escaped using `\`.
///   These characters are reserved for future use.
/// - Use `!` as first character to negate the glob
///
/// A path segment is delimited by path separator `/` or the start/end of the path.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct RestrictedGlob {
    is_negated: bool,
    glob: globset::GlobMatcher,
}
impl RestrictedGlob {
    /// Returns `true` if this glob is negated.
    ///
    /// ```
    /// use biome_js_analyze::utils::restricted_glob::RestrictedGlob;
    ///
    /// let glob = "!*.js".parse::<RestrictedGlob>().unwrap();
    /// assert!(glob.is_negated());
    ///
    /// let glob = "*.js".parse::<RestrictedGlob>().unwrap();
    /// assert!(!glob.is_negated());
    /// ```
    pub fn is_negated(&self) -> bool {
        self.is_negated
    }

    /// Tests whether the given path matches this pattern.
    pub fn is_match(&self, path: impl AsRef<std::path::Path>) -> bool {
        self.is_raw_match(path) != self.is_negated
    }

    /// Tests whether the given path matches this pattern, ignoring the negation.
    fn is_raw_match(&self, path: impl AsRef<std::path::Path>) -> bool {
        self.glob.is_match(path)
    }

    /// Tests whether the given path matches this pattern.
    pub fn is_match_candidate(&self, path: &CandidatePath<'_>) -> bool {
        self.is_raw_match_candidate(path) != self.is_negated
    }

    /// Tests whether the given path matches this pattern, ignoring the negation.
    fn is_raw_match_candidate(&self, path: &CandidatePath<'_>) -> bool {
        self.glob.is_match_candidate(&path.0)
    }
}
impl PartialEq for RestrictedGlob {
    fn eq(&self, other: &Self) -> bool {
        self.is_negated == other.is_negated && self.glob.glob() == other.glob.glob()
    }
}
impl Eq for RestrictedGlob {}
impl std::hash::Hash for RestrictedGlob {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.is_negated.hash(state);
        self.glob.glob().hash(state);
    }
}
impl std::fmt::Display for RestrictedGlob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = self.glob.glob();
        let negation = if self.is_negated { "!" } else { "" };
        write!(f, "{negation}{repr}")
    }
}
impl From<RestrictedGlob> for String {
    fn from(value: RestrictedGlob) -> Self {
        value.to_string()
    }
}
impl std::str::FromStr for RestrictedGlob {
    type Err = RestrictedGlobError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (is_negated, value) = if let Some(stripped) = value.strip_prefix('!') {
            (true, stripped)
        } else {
            (false, value)
        };
        validate_restricted_glob(value)?;
        let mut glob_builder = globset::GlobBuilder::new(value);
        // Allow escaping with `\` on all platforms.
        glob_builder.backslash_escape(true);
        // Only `**` can match `/`
        glob_builder.literal_separator(true);
        match glob_builder.build() {
            Ok(glob) => Ok(RestrictedGlob {
                is_negated,
                glob: glob.compile_matcher(),
            }),
            Err(error) => Err(RestrictedGlobError::Generic(
                error.kind().to_string().into_boxed_str(),
            )),
        }
    }
}
impl TryFrom<String> for RestrictedGlob {
    type Error = RestrictedGlobError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
// We use a custom impl to precisely report the location of the error.
impl biome_deserialize::Deserializable for RestrictedGlob {
    fn deserialize(
        value: &impl biome_deserialize::DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<biome_deserialize::DeserializationDiagnostic>,
    ) -> Option<Self> {
        let glob = String::deserialize(value, name, diagnostics)?;
        match glob.parse() {
            Ok(glob) => Some(glob),
            Err(error) => {
                let range = value.range();
                let range = error.index().map_or(range, |index| {
                    TextRange::at(range.start() + TextSize::from(1 + index), 1u32.into())
                });
                diagnostics.push(
                    biome_deserialize::DeserializationDiagnostic::new(format_args!("{error}"))
                        .with_range(range),
                );
                None
            }
        }
    }
}
#[cfg(feature = "schemars")]
impl schemars::JsonSchema for RestrictedGlob {
    fn schema_name() -> String {
        "Regex".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
}

/// A candidate path for matching.
///
/// Constructing candidates has a very small cost associated with it, so
/// callers may find it beneficial to amortize that cost when matching a single
/// path against multiple globs or sets of globs.
#[derive(Debug, Clone)]
pub struct CandidatePath<'a>(globset::Candidate<'a>);
impl<'a> CandidatePath<'a> {
    /// Create a new candidate for matching from the given path.
    pub fn new(path: &'a impl AsRef<std::path::Path>) -> Self {
        Self(globset::Candidate::new(path))
    }

    /// Tests whether the current path matches `glob`.
    pub fn matches(&self, glob: &RestrictedGlob) -> bool {
        glob.is_match_candidate(self)
    }

    /// Match against a list of globs where negated globs are handled as exceptions.
    ///
    /// Let's take an example:
    ///
    /// ```
    /// use biome_js_analyze::utils::restricted_glob::{CandidatePath, RestrictedGlob};
    ///
    /// let globs: &[RestrictedGlob] = &[
    ///     "*".parse().unwrap(),
    ///     "!a*".parse().unwrap(),
    ///     "a".parse().unwrap(),
    /// ];
    ///
    /// assert!(CandidatePath::new(&"b").matches_with_exceptions(globs));
    /// assert!(CandidatePath::new(&"a").matches_with_exceptions(globs));
    ///
    /// assert!(!CandidatePath::new(&"abc").matches_with_exceptions(globs));
    /// ```
    ///
    /// - `b` matches `*` and is not excluded by the exception `!a*`.
    ///   Thus, `b` matches the list of globs.
    /// - `abc` matches the first glob `*`, however it is excluded by the exception `!a*`.
    ///   Thus `abc` doesn't match the list of globs.
    /// - `a` matches the first glob `*` and is excluded by the exception `!a*`.
    ///   However, it is included again by the last glob `a`.
    ///   Thus `a` matches the list of globs.
    ///
    pub fn matches_with_exceptions<'b, I>(&self, globs: I) -> bool
    where
        I: IntoIterator<Item = &'b RestrictedGlob>,
        I::IntoIter: DoubleEndedIterator,
    {
        // Iterate in reverse order to avoid unnecessary glob matching.
        for glob in globs.into_iter().rev() {
            if glob.is_raw_match_candidate(self) {
                return !glob.is_negated();
            }
        }
        false
    }
}

#[derive(Debug)]
pub enum RestrictedGlobError {
    Regular {
        kind: RestrictedGlobErrorKind,
        index: u32,
    },
    /// Error caused by a a third-party module.
    Generic(Box<str>),
}
impl RestrictedGlobError {
    /// Returns the index in the glob where the error is located.
    pub fn index(&self) -> Option<u32> {
        match self {
            Self::Regular { index, .. } => Some(*index),
            Self::Generic(_) => None,
        }
    }
}
impl std::error::Error for RestrictedGlobError {}
impl std::fmt::Display for RestrictedGlobError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Regular { kind, .. } => write!(f, "{kind}"),
            Self::Generic(desc) => write!(f, "{desc}"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum RestrictedGlobErrorKind {
    /// Occurs when an unescaped '\' is found at the end of a glob.
    DanglingEscape,
    /// Occurs when an invalid escape is found.
    /// If the character is not set, then it is an invalid UTF-8 character.
    InvalidEscape(char),
    UnsupportedAlternates,
    UnsupportedCharacterClass,
    UnsupportedAnyCharacter,
}
impl std::fmt::Display for RestrictedGlobErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            Self::DanglingEscape => "Unterminated escape sequence.",
            Self::InvalidEscape(c) => {
                return write!(f, "The escape sequence `\\{c}` is not supported.");
            }
            Self::UnsupportedAlternates => {
                r"Alternates `{}` are not supported. Use `\{` and `\}` to escape the characters."
            }
            Self::UnsupportedCharacterClass => {
                r"Character class `[]` are not supported. Use `\[` and `\]` to escape the characters."
            }
            Self::UnsupportedAnyCharacter => {
                r"`?` matcher is not supported. Use `\?` to escape the character."
            }
        };
        write!(f, "{desc}")
    }
}

/// Returns an error if `pattern` doesn't follow the restricted glob syntax.
fn validate_restricted_glob(pattern: &str) -> Result<(), RestrictedGlobError> {
    let mut it = pattern.bytes().enumerate();
    while let Some((i, c)) = it.next() {
        match c {
            b'\\' => {
                // Accept a restrictive set of escape sequence
                if let Some((j, c)) = it.next() {
                    if !matches!(c, b'!' | b'*' | b'?' | b'{' | b'}' | b'[' | b']' | b'\\') {
                        return Err(RestrictedGlobError::Regular {
                            kind: RestrictedGlobErrorKind::InvalidEscape(
                                // SAFETY: the index `j` starts a new character
                                // because it is preceded by the character `\\`.
                                pattern[j..].chars().next().expect("valid character"),
                            ),
                            index: i as u32,
                        });
                    }
                } else {
                    return Err(RestrictedGlobError::Regular {
                        kind: RestrictedGlobErrorKind::DanglingEscape,
                        index: i as u32,
                    });
                }
            }
            b'?' => {
                return Err(RestrictedGlobError::Regular {
                    kind: RestrictedGlobErrorKind::UnsupportedAnyCharacter,
                    index: i as u32,
                });
            }
            b'[' | b']' => {
                return Err(RestrictedGlobError::Regular {
                    kind: RestrictedGlobErrorKind::UnsupportedCharacterClass,
                    index: i as u32,
                });
            }
            b'{' | b'}' => {
                return Err(RestrictedGlobError::Regular {
                    kind: RestrictedGlobErrorKind::UnsupportedAlternates,
                    index: i as u32,
                });
            }
            _ => {}
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_validate_restricted_glob() {
        assert!(validate_restricted_glob("*.[jt]s").is_err());
        assert!(validate_restricted_glob("*.{js,ts}").is_err());
        assert!(validate_restricted_glob("?*.js").is_err());
        assert!(validate_restricted_glob(r"\").is_err());
        assert!(validate_restricted_glob(r"\n").is_err());
        assert!(validate_restricted_glob(r"\😀").is_err());

        assert!(validate_restricted_glob("!*.js").is_ok());
        assert!(validate_restricted_glob("!").is_ok());
        assert!(validate_restricted_glob("*.js").is_ok());
        assert!(validate_restricted_glob("**/*.js").is_ok());
        assert!(validate_restricted_glob(r"\*").is_ok());
        assert!(validate_restricted_glob(r"\!").is_ok());
    }

    #[test]
    fn test_restricted_regex() {
        assert!(!"*.js"
            .parse::<RestrictedGlob>()
            .unwrap()
            .is_match("file/path.js"));

        assert!("**/*.js"
            .parse::<RestrictedGlob>()
            .unwrap()
            .is_match("file/path.js"));
    }

    #[test]
    fn test_match_with_exceptions() {
        let a = CandidatePath::new(&"a");

        assert!(a.matches_with_exceptions(&[
            RestrictedGlob::from_str("*").unwrap(),
            RestrictedGlob::from_str("!b").unwrap(),
        ]));
        assert!(!a.matches_with_exceptions(&[
            RestrictedGlob::from_str("*").unwrap(),
            RestrictedGlob::from_str("!a*").unwrap(),
        ]));
        assert!(a.matches_with_exceptions(&[
            RestrictedGlob::from_str("*").unwrap(),
            RestrictedGlob::from_str("!a*").unwrap(),
            RestrictedGlob::from_str("a").unwrap(),
        ]));
    }

    #[test]
    fn test_to_string() {
        assert_eq!(
            RestrictedGlob::from_str("**/*.js").unwrap().to_string(),
            "**/*.js"
        );
        assert_eq!(
            RestrictedGlob::from_str("!**/*.js").unwrap().to_string(),
            "!**/*.js"
        );
    }
}
