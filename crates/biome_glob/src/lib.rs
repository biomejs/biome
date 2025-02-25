//! `biome_glob` provides globbing functionality. When listing the globs to match,
//! it is also possible to provide globs that function as "exceptions" by prefixing the globs with `!`.
//!
//! A glob is primarily used to select or filter a set of file paths by matching every file paths against the glob.
//! A file path either matches or doesn't match a glob.
//! For example, the path `lib.rs` matches the glob `*.rs`.
//!
//! Biome globs are case-sensitive. This means that `lib.RS` doesn't match `*.rs`.
//!
//! You have to understand the structure of a path to understand which path match a glob.
//! A path is divided in path segments.
//! Every path segment is delimited by the path separator `/` or the start/end of the path.
//! For instance `src/lib.rs` consists of two path segments: `src` and `lib.rs`.
//! A Biome glob supports the following patterns:
//!
//! - star `*` that matches zero or more characters inside a path segment
//!
//!   `lib.rs` matches `*.rs`.
//!   Conversely, `src/lib.rs` doesn't match `*.rs`
//!
//! - globstar `**` that matches zero or more path segments
//!   `**` must be enclosed by path separators `/` or the start/end of the glob.
//!   For example, `**a` is not a valid glob.
//!   Also, `**` must not be followed by another globstar.
//!   For example, `**/**` is not a valid glob.
//!
//!   `lib.rs` and `src/lib.rs` match `**` and `**/*.rs`
//!   Conversely, `README.txt` doesn't match `**/*.rs` because the pat ends with `.txt`.
//!
//! - Use `\*` to escape `*`
//!
//!   the path `*` matches `\*`.
//!
//! - `?`, `[`, `]`, `{`, and `}` must be escaped using `\`.
//!   These characters are reserved for possible future use.
//!
//! - Use `!` as first character to negate a glob
//!
//!   `README.txt` matches `!*.rs`.
//!   `src/lib.rs` matches `!*.rs` because the path contains several segments.
//!
//! ## Matching a path against a glob
//!
//! You can create a glob from a string using the `parse` method.
//! Use [Glob::is_match] to match against anything that can be turned into a [std::path::Path], such as a string.
//!
//! In the following example we parse the string `"*.rs"` into a glob, and we match against two strings.
//! `lib.rs` matches the glob because the path has a single path segment that ends with `.rs`.
//! Conversely, `src/lib.rs` doesn't match because it has two path segments (`src` and `lib.rs`).
//!
//! ```
//! use biome_glob::Glob;
//!
//! let glob: Glob = "*.rs".parse().expect("correct glob");
//! assert!(glob.is_match("lib.rs"));
//! assert!(!glob.is_match("src/lib.rs"));
//! ```
//!
//! ## Matching against multiple globs
//!
//! When a path is expected to be matched against several globs,
//! you should compile the path into a [CandidatePath] using [CandidatePath::new].
//! [CandidatePath] may speed up matching against several globs.
//! To get advantage of the speed-up, you have to use the [CandidatePath::matches] method instead of [Glob::is_match].
//!
//! In the following example, we create a list of two globs and we match them against a path compiled into a candidate path.
//! The path matches the second glob of the list.
//!
//! ```
//! use biome_glob::{CandidatePath, Glob};
//!
//! let globs: &[Glob] = &[
//!     "**/*.rs".parse().expect("correct glob"),
//!     "**/*.txt".parse().expect("correct glob"),
//! ];
//!
//! let path = CandidatePath::new(&"a/path/to/file.txt");
//!
//! assert!(globs.iter().any(|glob| path.matches(glob)));
//! ```
//!
//! ## Matching against multiple globs and exceptions
//!
//! Negated globs are particularly useful to denote exceptions in a list of globs.
//! To interpret negated globs as exceptions, use [CandidatePath::matches_with_exceptions].
//! This semantic is taken from the [.gitignore](https://git-scm.com/docs/gitignore#_pattern_format) format.
//!
//! In the following example we accept all files in the `src` directory, except the ones ending with the `txt` extension.
//!
//! ```
//! use biome_glob::{CandidatePath, Glob};
//!
//! let globs: &[Glob] = &[
//!     "**/*.rs".parse().expect("correct glob"),
//!     "!**/*.txt".parse().expect("correct glob"),
//! ];
//!
//! let path = CandidatePath::new(&"a/path/to/file.txt");
//!
//! assert!(!path.matches_with_exceptions(globs));
//! ```
//!
//! ## Matching a directory path against multiple globs and exceptions
//!
//! Taking the previous example, the directory path `a/path` doesn't match `**/*.rs` the list of glob,
//! because the path doesn't end with the `.rs` extension.
//! This behavior is problematic when you write a file crawler that traverse the file hierarchy and
//! ignore directories with files that never match the list of globs.
//! Biome provides a dedicated method [CandidatePath::matches_directory_with_exceptions] for this purpose.
//! The method only check if the directory is not excluded by an exception.
//!
//! In the following example, `dir1` matches the list of globs, while `dir2` doesn't.
//!
//! ```
//! use biome_glob::{CandidatePath, Glob};
//!
//! let globs: &[Glob] = &[
//!     "**/*.rs".parse().expect("correct glob"),
//!     "!test".parse().expect("correct glob"),
//! ];
//!
//! let dir1 = CandidatePath::new(&"src");
//! let dir2 = CandidatePath::new(&"test");
//!
//! assert!(dir1.matches_directory_with_exceptions(globs));
//! assert!(!dir2.matches_directory_with_exceptions(globs));
//! ```
//!

pub mod editorconfig;

/// A Biome glob pattern.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct Glob {
    is_negated: bool,
    glob: globset::GlobMatcher,
}
impl Glob {
    /// Returns `true` if this glob is negated.
    ///
    /// ```
    /// let glob = "!*.js".parse::<biome_glob::Glob>().unwrap();
    /// assert!(glob.is_negated());
    ///
    /// let glob = "*.js".parse::<biome_glob::Glob>().unwrap();
    /// assert!(!glob.is_negated());
    /// ```
    pub fn is_negated(&self) -> bool {
        self.is_negated
    }

    /// Returns the negated version of this glob.
    ///
    /// ```
    /// let glob = "!*.js".parse::<biome_glob::Glob>().unwrap();
    /// assert!(!glob.negated().is_negated());
    ///
    /// let glob = "*.js".parse::<biome_glob::Glob>().unwrap();
    /// assert!(glob.negated().is_negated());
    /// ```
    pub fn negated(self) -> Self {
        Self {
            is_negated: !self.is_negated,
            glob: self.glob,
        }
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
    fn is_match_candidate(&self, path: &CandidatePath<'_>) -> bool {
        self.is_raw_match_candidate(path) != self.is_negated
    }

    /// Tests whether the given path matches this pattern, ignoring the negation.
    fn is_raw_match_candidate(&self, path: &CandidatePath<'_>) -> bool {
        self.glob.is_match_candidate(&path.0)
    }
}
impl PartialEq for Glob {
    fn eq(&self, other: &Self) -> bool {
        self.is_negated == other.is_negated && self.glob.glob() == other.glob.glob()
    }
}
impl Eq for Glob {}
impl std::hash::Hash for Glob {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.is_negated.hash(state);
        self.glob.glob().hash(state);
    }
}
impl std::fmt::Display for Glob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = self.glob.glob();
        let negation = if self.is_negated { "!" } else { "" };
        write!(f, "{negation}{repr}")
    }
}
impl std::fmt::Debug for Glob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
impl From<Glob> for String {
    fn from(value: Glob) -> Self {
        value.to_string()
    }
}
impl std::str::FromStr for Glob {
    type Err = GlobError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (is_negated, value) = if let Some(stripped) = value.strip_prefix('!') {
            (true, stripped)
        } else {
            (false, value)
        };
        validate_glob(value)?;
        let mut glob_builder = globset::GlobBuilder::new(value);
        // Allow escaping with `\` on all platforms.
        glob_builder.backslash_escape(true);
        // Only `**` can match `/`
        glob_builder.literal_separator(true);
        match glob_builder.build() {
            Ok(glob) => Ok(Glob {
                is_negated,
                glob: glob.compile_matcher(),
            }),
            Err(error) => Err(GlobError::Generic(
                error.kind().to_string().into_boxed_str(),
            )),
        }
    }
}
impl TryFrom<String> for Glob {
    type Error = GlobError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
// We use a custom impl to precisely report the location of the error.
#[cfg(feature = "biome_deserialize")]
impl biome_deserialize::Deserializable for Glob {
    fn deserialize(
        ctx: &mut impl biome_deserialize::DeserializationContext,
        value: &impl biome_deserialize::DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        let glob = String::deserialize(ctx, value, name)?;
        match glob.parse() {
            Ok(glob) => Some(glob),
            Err(error) => {
                let range = value.range();
                let range = error.index().map_or(range, |index| {
                    biome_text_size::TextRange::at(
                        range.start() + biome_text_size::TextSize::from(1 + index),
                        1u32.into(),
                    )
                });
                ctx.report(
                    biome_deserialize::DeserializationDiagnostic::new(format_args!("{error}"))
                        .with_range(range),
                );
                None
            }
        }
    }
}
#[cfg(feature = "schema")]
impl schemars::JsonSchema for Glob {
    fn schema_name() -> String {
        "Glob".to_string()
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
    pub fn new<P: AsRef<std::path::Path> + ?Sized>(path: &'a P) -> Self {
        Self(globset::Candidate::new(path))
    }

    /// Tests whether the current path matches `glob`.
    pub fn matches(&self, glob: &Glob) -> bool {
        glob.is_match_candidate(self)
    }

    /// Match against a list of globs where negated globs are handled as exceptions.
    ///
    /// Let's take an example:
    ///
    /// ```
    /// use biome_glob::{CandidatePath, Glob};
    ///
    /// let globs: &[Glob] = &[
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
        I: IntoIterator<Item = &'b Glob>,
        I::IntoIter: DoubleEndedIterator,
    {
        self.matches_with_exceptions_or(false, globs)
    }

    /// Match against a list of globs where negated globs are handled as exceptions handling the path as a directory.
    ///
    /// This method is useful for implementing a file crawler that avoids traversing directories that should be ignored.
    ///
    /// The current implementation returns `false` only if at least one exception (negated glob) matches the path.
    ///
    ///
    /// ```
    /// use biome_glob::{CandidatePath, Glob};
    ///
    /// let globs: &[Glob] = &[
    ///     "a/path".parse().unwrap(),
    ///     "!b".parse().unwrap(),
    /// ];
    ///
    /// assert!(CandidatePath::new(&"a/path").matches_directory_with_exceptions(globs));
    /// assert!(CandidatePath::new(&"a").matches_directory_with_exceptions(globs));
    ///
    /// assert!(!CandidatePath::new(&"b").matches_directory_with_exceptions(globs));
    ///
    /// // Ideally, the following cases should not match.
    /// // The current implementation doesn't reject them.
    /// assert!(CandidatePath::new(&"c").matches_directory_with_exceptions(globs));
    /// assert!(CandidatePath::new(&"b/inner").matches_directory_with_exceptions(globs));
    /// ```
    pub fn matches_directory_with_exceptions<'b, I>(&self, globs: I) -> bool
    where
        I: IntoIterator<Item = &'b Glob>,
        I::IntoIter: DoubleEndedIterator,
    {
        self.matches_with_exceptions_or(true, globs)
    }

    /// Match against a list of globs where negated globs are handled as exceptions.
    /// Returns `default` if there is no globs that match.
    fn matches_with_exceptions_or<'b, I>(&self, default: bool, globs: I) -> bool
    where
        I: IntoIterator<Item = &'b Glob>,
        I::IntoIter: DoubleEndedIterator,
    {
        // Iterate in reverse order to avoid unnecessary glob matching.
        for glob in globs.into_iter().rev() {
            if glob.is_raw_match_candidate(self) {
                return !glob.is_negated();
            }
        }
        default
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum GlobError {
    Regular {
        kind: GlobErrorKind,
        index: u32,
    },
    /// Error caused by a third-party module.
    Generic(Box<str>),
}
impl GlobError {
    /// Returns the index in the glob where the error is located.
    pub fn index(&self) -> Option<u32> {
        match self {
            Self::Regular { index, .. } => Some(*index),
            Self::Generic(_) => None,
        }
    }
}
impl std::error::Error for GlobError {}
impl std::fmt::Display for GlobError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Regular { kind, .. } => write!(f, "{kind}"),
            Self::Generic(desc) => write!(f, "{desc}"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum GlobErrorKind {
    /// Occurs when an unescaped '\' is found at the end of a glob.
    DanglingEscape,
    /// Occurs when an invalid escape is found.
    InvalidEscape,
    /// Occurs when `**` isn't enclosed by the path separator `/` or the start/end of the glob.
    InvalidGlobStar,
    /// Nested `{}` are not supported.
    UnsupportedNestedAlternates,
    /// `[]` is not supported.
    UnsupportedCharacterClass,
    /// `?` is not supported.
    UnsupportedAnyCharacter,
}
impl std::fmt::Display for GlobErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            Self::DanglingEscape => "Unterminated escape sequence.",
            Self::InvalidEscape => "Invalid escape sequence.",
            Self::InvalidGlobStar => {
                r"`**` must be enclosed by the path separator `/`, or the start/end of the glob and mustn't be followed by `/**`."
            }
            Self::UnsupportedNestedAlternates => {
                r"Nested alternates `{}` are not supported. Use a separate glob for each nested alternate."
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

/// Returns an error if `pattern` doesn't follow the supported glob syntax.
fn validate_glob(pattern: &str) -> Result<(), GlobError> {
    let mut it = pattern.bytes().enumerate();
    let mut allow_globstar = true;
    let mut alternates_found = false;
    while let Some((i, c)) = it.next() {
        match c {
            b'*' => {
                let mut lookahead = it.clone();
                if matches!(lookahead.next(), Some((_, b'*'))) {
                    if !allow_globstar || !matches!(lookahead.next(), None | Some((_, b'/'))) {
                        return Err(GlobError::Regular {
                            kind: GlobErrorKind::InvalidGlobStar,
                            index: i as u32,
                        });
                    }
                    // Eat `*`
                    it.next();
                    // Eat `/`
                    it.next();
                }
            }
            b'\\' => {
                // Accept a restrictive set of escape sequence
                if let Some((_, c)) = it.next() {
                    if !matches!(c, b'!' | b'*' | b'?' | b'{' | b'}' | b'[' | b']' | b'\\') {
                        return Err(GlobError::Regular {
                            kind: GlobErrorKind::InvalidEscape,
                            index: i as u32,
                        });
                    }
                } else {
                    return Err(GlobError::Regular {
                        kind: GlobErrorKind::DanglingEscape,
                        index: i as u32,
                    });
                }
            }
            b'?' => {
                return Err(GlobError::Regular {
                    kind: GlobErrorKind::UnsupportedAnyCharacter,
                    index: i as u32,
                });
            }
            b'[' | b']' => {
                return Err(GlobError::Regular {
                    kind: GlobErrorKind::UnsupportedCharacterClass,
                    index: i as u32,
                });
            }
            b'{' => {
                if alternates_found {
                    return Err(GlobError::Regular {
                        kind: GlobErrorKind::UnsupportedNestedAlternates,
                        index: i as u32,
                    });
                } else {
                    alternates_found = true;
                }
            }
            b'}' => {
                alternates_found = false;
            }
            _ => {}
        }
        allow_globstar = c == b'/';
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_validate_glob() {
        assert_eq!(
            validate_glob("*.[jt]s"),
            Err(GlobError::Regular {
                kind: GlobErrorKind::UnsupportedCharacterClass,
                index: 2
            })
        );
        assert_eq!(
            validate_glob("?*.js"),
            Err(GlobError::Regular {
                kind: GlobErrorKind::UnsupportedAnyCharacter,
                index: 0
            })
        );
        assert_eq!(
            validate_glob(r"\"),
            Err(GlobError::Regular {
                kind: GlobErrorKind::DanglingEscape,
                index: 0
            })
        );
        assert_eq!(
            validate_glob(r"\n"),
            Err(GlobError::Regular {
                kind: GlobErrorKind::InvalidEscape,
                index: 0
            })
        );
        assert_eq!(
            validate_glob(r"\😀"),
            Err(GlobError::Regular {
                kind: GlobErrorKind::InvalidEscape,
                index: 0
            })
        );
        assert_eq!(
            validate_glob(r"***"),
            Err(GlobError::Regular {
                kind: GlobErrorKind::InvalidGlobStar,
                index: 0
            })
        );
        assert_eq!(
            validate_glob(r"a**"),
            Err(GlobError::Regular {
                kind: GlobErrorKind::InvalidGlobStar,
                index: 1
            })
        );
        assert_eq!(
            validate_glob(r"**a"),
            Err(GlobError::Regular {
                kind: GlobErrorKind::InvalidGlobStar,
                index: 0
            })
        );
        assert_eq!(
            validate_glob(r"**/**"),
            Err(GlobError::Regular {
                kind: GlobErrorKind::InvalidGlobStar,
                index: 3
            })
        );

        assert_eq!(
            validate_glob(r"file.{{spec,test}}"),
            Err(GlobError::Regular {
                kind: GlobErrorKind::UnsupportedNestedAlternates,
                index: 6
            })
        );

        assert!(validate_glob("!*.js").is_ok());
        assert!(validate_glob("!").is_ok());
        assert!(validate_glob("*.js").is_ok());
        assert!(validate_glob("**/*.js").is_ok());
        assert!(validate_glob(r"\*").is_ok());
        assert!(validate_glob(r"\!").is_ok());
        assert!(validate_glob(r"**").is_ok());
        assert!(validate_glob(r"/**/").is_ok());
        assert!(validate_glob(r"**/").is_ok());
        assert!(validate_glob(r"/**").is_ok());
        assert!(validate_glob(r"*.{js,jsx}").is_ok());
    }

    #[test]
    fn test_is_match() {
        assert!("*.rs".parse::<Glob>().unwrap().is_match("lib.rs"));
        assert!(!"*.rs".parse::<Glob>().unwrap().is_match("src/lib.rs"));
        assert!("**/*.rs".parse::<Glob>().unwrap().is_match("src/lib.rs"));
        assert!("file.{js,jsx}".parse::<Glob>().unwrap().is_match("file.js"));
        assert!("file.{js,jsx}"
            .parse::<Glob>()
            .unwrap()
            .is_match("file.jsx"));
    }

    #[test]
    fn test_matches_with_exceptions() {
        let a = CandidatePath::new(&"a");

        assert!(a.matches_with_exceptions(&[
            Glob::from_str("*").unwrap(),
            Glob::from_str("!b").unwrap(),
        ]));
        assert!(!a.matches_with_exceptions(&[
            Glob::from_str("*").unwrap(),
            Glob::from_str("!a*").unwrap(),
        ]));
        assert!(a.matches_with_exceptions(&[
            Glob::from_str("*").unwrap(),
            Glob::from_str("!a*").unwrap(),
            Glob::from_str("a").unwrap(),
        ]));
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Glob::from_str("**/*.rs").unwrap().to_string(), "**/*.rs");
        assert_eq!(Glob::from_str("!**/*.rs").unwrap().to_string(), "!**/*.rs");
        assert_eq!(
            Glob::from_str("file.{js,jsx}").unwrap().to_string(),
            "file.{js,jsx}"
        );
    }
}
