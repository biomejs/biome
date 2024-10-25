use biome_rowan::{TextRange, TextSize};

/// A restricted glob pattern only supports the following syntaxes:
///
/// - star `*` that matches zero or more characters inside a path segment
/// - globstar `**` that matches zero or more path segments
/// - Use `\*` to escape `*`
/// - `?`, `[`, `]`, `{`, and `}` must be escaped using `\`.
///   These characters are reserved for future use.
/// - `!` must be escaped if it is the first character of the pattern
///
/// A path segment is delimited by path separator `/` or the start/end of the path.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct RestrictedGlob(globset::GlobMatcher);
impl RestrictedGlob {
    /// Tests whether the given path matches this pattern or not.
    pub fn is_match(&self, path: impl AsRef<std::path::Path>) -> bool {
        self.0.is_match(path)
    }

    /// Tests whether the given path matches this pattern or not.
    pub fn is_match_candidate(&self, path: &CandidatePath<'_>) -> bool {
        self.0.is_match_candidate(&path.0)
    }
}
impl std::fmt::Display for RestrictedGlob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = self.0.glob().to_string();
        f.write_str(&repr)
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
        validate_restricted_glob(value)?;
        let mut glob_builder = globset::GlobBuilder::new(value);
        // Allow escaping with `\` on all platforms.
        glob_builder.backslash_escape(true);
        // Only `**` can match `/`
        glob_builder.literal_separator(true);
        match glob_builder.build() {
            Ok(glob) => Ok(RestrictedGlob(glob.compile_matcher())),
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
pub struct CandidatePath<'a>(globset::Candidate<'a>);
impl<'a> CandidatePath<'a> {
    /// Create a new candidate for matching from the given path.
    pub fn new(path: &'a impl AsRef<std::path::Path>) -> Self {
        Self(globset::Candidate::new(path))
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
    UnsupportedNegation,
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
            Self::UnsupportedNegation => {
                r"Negated globs `!` are not supported. Use `\!` to escape the character."
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
            b'!' if i == 0 => {
                return Err(RestrictedGlobError::Regular {
                    kind: RestrictedGlobErrorKind::UnsupportedNegation,
                    index: i as u32,
                });
            }
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
    use super::*;

    #[test]
    fn test_validate_restricted_glob() {
        assert!(validate_restricted_glob("!*.js").is_err());
        assert!(validate_restricted_glob("*.[jt]s").is_err());
        assert!(validate_restricted_glob("*.{js,ts}").is_err());
        assert!(validate_restricted_glob("?*.js").is_err());
        assert!(validate_restricted_glob(r"\").is_err());
        assert!(validate_restricted_glob(r"\n").is_err());
        assert!(validate_restricted_glob(r"\😀").is_err());
        assert!(validate_restricted_glob("!").is_err());

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
}
