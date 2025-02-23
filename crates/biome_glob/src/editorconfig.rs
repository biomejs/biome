//! `biome_glob::editorconfig` provides globbing functionality that follows:
//!
//! - the [`.editorconfig` specification](https://editorconfig.org/#wildcards), and
//! - the [relative test suite](https://github.com/editorconfig/editorconfig-core-test/tree/master/glob).
//!
//! The current implementation has some limitations:
//!
//! - Alternates (choices) are limited to one level, which means nested alternates are not supported.
//!   For example, `{a,b{a,b},d}` is not supported
//! - Empty choice inside alternates such as `{a,b,}` is not supported
//! - `.editorconfig` allows globstars `**` at any position.
//!   It matches any character including `/`.
//!   Internally, Biome translates `**` to `*/**/*`.
//!   Thus Biome rejects some valid cases.
//!   For example `a**b` doesn't match `ab` or `axb`.
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
//! use biome_glob::editorconfig::EditorconfigGlob;
//!
//! let glob: EditorconfigGlob = "*.rs".parse().expect("correct glob");
//! assert!(glob.is_match("lib.rs"));
//! assert!(glob.is_match("src/lib.rs"));
//! ```
//!
//! ## Matching against multiple globs
//!
//! When a path is expected to be matched against several globs,
//! you should compile the path into a [crate::CandidatePath] using [crate::CandidatePath::new].
//! [crate::CandidatePath] may speed up matching against several globs.
//! To get advantage of the speed-up, you have to use the [EditorconfigGlob::is_match] method instead of [EditorconfigGlob::is_match].
//!
//! In the following example, we create a list of two globs and we match them against a path compiled into a candidate path.
//! The path matches the second glob of the list.
//!
//! ```
//! use biome_glob::{CandidatePath, editorconfig::EditorconfigGlob};
//!
//! let globs: &[EditorconfigGlob] = &[
//!     "**/*.rs".parse().expect("correct glob"),
//!     "**/*.txt".parse().expect("correct glob"),
//! ];
//!
//! let path = CandidatePath::new(&"a/path/to/file.txt");
//!
//! assert!(globs.iter().any(|glob| glob.is_match_candidate(&path)));
//! ```
//!

/// Am Editorconfig glob pattern.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct EditorconfigGlob {
    glob: globset::GlobMatcher,
}
impl EditorconfigGlob {
    /// Tests whether the given path matches this pattern.
    pub fn is_match(&self, path: impl AsRef<std::path::Path>) -> bool {
        self.glob.is_match(path)
    }

    /// Tests whether the given path matches this pattern.
    pub fn is_match_candidate(&self, path: &crate::CandidatePath<'_>) -> bool {
        self.glob.is_match_candidate(&path.0)
    }
}
impl PartialEq for EditorconfigGlob {
    fn eq(&self, other: &Self) -> bool {
        self.glob.glob() == other.glob.glob()
    }
}
impl Eq for EditorconfigGlob {}
impl std::hash::Hash for EditorconfigGlob {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.glob.glob().hash(state);
    }
}
impl std::fmt::Display for EditorconfigGlob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.glob.glob())
    }
}
impl std::fmt::Debug for EditorconfigGlob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
impl From<EditorconfigGlob> for String {
    fn from(value: EditorconfigGlob) -> Self {
        value.to_string()
    }
}
impl std::str::FromStr for EditorconfigGlob {
    type Err = EditorconfigGlobError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        value.to_string().try_into()
    }
}
impl TryFrom<String> for EditorconfigGlob {
    type Error = EditorconfigGlobError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = convert_glob(value)?;
        let mut glob_builder = globset::GlobBuilder::new(&value);
        // Allow escaping with `\` on all platforms.
        glob_builder.backslash_escape(true);
        // Only `**` can match `/`
        glob_builder.literal_separator(true);
        match glob_builder.build() {
            Ok(glob) => Ok(EditorconfigGlob {
                glob: glob.compile_matcher(),
            }),
            Err(error) => Err(EditorconfigGlobError::Generic(
                error.kind().to_string().into_boxed_str(),
            )),
        }
    }
}
// We use a custom impl to precisely report the location of the error.
#[cfg(feature = "biome_deserialize")]
impl biome_deserialize::Deserializable for EditorconfigGlob {
    fn deserialize(
        ctx: &mut impl biome_deserialize::DeserializationContext,
        value: &impl biome_deserialize::DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        let glob = String::deserialize(ctx, value, name)?;
        match glob.try_into() {
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
impl schemars::JsonSchema for EditorconfigGlob {
    fn schema_name() -> String {
        "Glob".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum EditorconfigGlobError {
    Regular {
        kind: EditorconfigGlobErrorKind,
        index: u32,
    },
    /// Error caused by a third-party module.
    Generic(Box<str>),
}
impl EditorconfigGlobError {
    /// Returns the index in the glob where the error is located.
    pub fn index(&self) -> Option<u32> {
        match self {
            Self::Regular { index, .. } => Some(*index),
            Self::Generic(_) => None,
        }
    }
}
impl std::error::Error for EditorconfigGlobError {}
impl std::fmt::Display for EditorconfigGlobError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Regular { kind, .. } => write!(f, "{kind}"),
            Self::Generic(desc) => write!(f, "{desc}"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum EditorconfigGlobErrorKind {
    /// Occurs when an unescaped '\' is found at the end of a glob.
    DanglingEscape,
    /// Nested `{}` are not supported.
    UnsupportedNestedAlternates,
    /// Reversed numeric range such as `{1..0}` are not supported.
    UnsupportedReversedNumericRange,
}
impl std::fmt::Display for EditorconfigGlobErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            Self::DanglingEscape => "Unterminated escape sequence.",
            Self::UnsupportedNestedAlternates => {
                r"Nested alternates `{}` are not supported. Use a separate glob for each nested alternate."
            }
            Self::UnsupportedReversedNumericRange => {
                r"Reversed numeric ranges such as `{1..0}` are not supported. Flip the numbers to obtain a valid numeric range."
            }
        };
        write!(f, "{desc}")
    }
}

/// Convert an Editorconfig glob into a gloset's glob
/// See https://github.com/editorconfig/editorconfig/issues/528#issuecomment-2495721612
fn convert_glob(mut pattern: String) -> Result<String, EditorconfigGlobError> {
    // Rust doesn't allow us to iterate and mutate a string at the same time.
    // Thus, we first gather all changes (operations) to perform on the pattern and
    // then we perform them in a second step.
    enum StrOp {
        Insert {
            index: usize,
            content: &'static str,
        },
        ConvertNumericRange {
            indexes: std::ops::Range<usize>,
            range: std::ops::Range<usize>,
        },
    }
    let mut it = pattern.bytes().enumerate();
    let mut changes = Vec::new();
    let mut has_no_slashes = true;
    let mut prev_is_slash = true;
    let mut alternation_start = None;
    let mut alternation_has_coma = false;
    while let Some((i, c)) = it.next() {
        match c {
            b'\\' => {
                // Eat the escaped character
                if it.next().is_none() {
                    return Err(EditorconfigGlobError::Regular {
                        kind: EditorconfigGlobErrorKind::DanglingEscape,
                        index: i as u32,
                    });
                }
            }
            b'/' => {
                has_no_slashes = false;
                prev_is_slash = true;
                continue;
            }
            b'*' => {
                let mut lookahead = it.clone();
                if matches!(lookahead.next(), Some((_, b'*'))) {
                    if !prev_is_slash {
                        changes.push(StrOp::Insert {
                            index: i,
                            content: "*/",
                        });
                    }
                    match lookahead.next() {
                        None | Some((_, b'/')) => {}
                        Some((_, b'*')) => {
                            changes.push(StrOp::Insert {
                                index: i + 2,
                                content: "/",
                            });
                        }
                        _ => {
                            changes.push(StrOp::Insert {
                                index: i + 2,
                                content: "/*",
                            });
                        }
                    }
                    // Eat `*`
                    it.next();
                }
            }
            b'[' => {
                let mut lookahead = it.clone();
                loop {
                    match lookahead.next() {
                        Some((_, b'\\')) => {
                            // Consume escaped character.
                            lookahead.next();
                        }
                        None | Some((_, b'/')) => {
                            // `[/]` is treated as a literal
                            changes.push(StrOp::Insert {
                                index: i,
                                content: r"\",
                            });
                            break;
                        }
                        Some((j, b']')) => {
                            it.nth(j - i);
                            break;
                        }
                        _ => {}
                    }
                }
            }
            b'{' => {
                if alternation_start.is_some() {
                    return Err(EditorconfigGlobError::Regular {
                        kind: EditorconfigGlobErrorKind::UnsupportedNestedAlternates,
                        index: i as u32,
                    });
                }
                alternation_start = Some(i);
            }
            b',' if alternation_start.is_some() => {
                alternation_has_coma = true;
            }
            b'}' => {
                if let Some(alternation_start) = alternation_start.take() {
                    let range_candidate = &pattern[(alternation_start + 1)..i];
                    if let Some((start, end)) = range_candidate
                        .split_once("..")
                        .and_then(|(start, end)| Some((start.parse().ok()?, end.parse().ok()?)))
                    {
                        if start > end {
                            return Err(EditorconfigGlobError::Regular {
                                kind: EditorconfigGlobErrorKind::UnsupportedReversedNumericRange,
                                index: alternation_start as u32,
                            });
                        }
                        changes.push(StrOp::ConvertNumericRange {
                            indexes: (alternation_start + 1)..(i),
                            range: start..(end + 1),
                        });
                    } else if !alternation_has_coma {
                        // `{}` and `{single}` are not considered as alternations.
                        changes.push(StrOp::Insert {
                            index: alternation_start,
                            content: r"\",
                        });
                        changes.push(StrOp::Insert {
                            index: i,
                            content: r"\",
                        });
                        alternation_has_coma = false;
                    }
                } else {
                    changes.push(StrOp::Insert {
                        index: i,
                        content: r"\",
                    });
                }
            }
            _ => {}
        }
        prev_is_slash = false;
    }
    if let Some(alternation_start) = alternation_start {
        // If it is not closed, then it is not an alternation.
        changes.push(StrOp::Insert {
            index: alternation_start,
            content: r"\",
        });
    }
    for change in changes.into_iter().rev() {
        match change {
            StrOp::Insert { index, content } => {
                pattern.insert_str(index, content);
            }
            StrOp::ConvertNumericRange { indexes, range } => {
                let mut replacement = String::with_capacity(range.len() * 2);
                for (i, number) in range.enumerate() {
                    if i != 0 {
                        replacement.push(',');
                    }
                    // This should not fail because we write in a String.
                    // Thus, we ignore any error.
                    let _ = std::fmt::write(&mut replacement, format_args!("{}", number));
                }
                pattern.replace_range(indexes, &replacement);
            }
        }
    }
    if has_no_slashes && !pattern.is_empty() && !pattern.starts_with("**") {
        pattern.insert_str(0, "**/");
    }
    Ok(pattern)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_matching(glob: &EditorconfigGlob, path: &str) -> bool {
        glob.is_match_candidate(&crate::CandidatePath::new(path))
    }

    #[test]
    fn test_convert_glob() {
        fn convert_valid_glob(pattern: impl Into<String>) -> String {
            super::convert_glob(pattern.into()).unwrap()
        }

        assert_eq!(convert_valid_glob(""), "");
        assert_eq!(convert_valid_glob("**"), "**");
        assert_eq!(convert_valid_glob("a/**"), "a/**");
        assert_eq!(convert_valid_glob("**/a"), "**/a");
        assert_eq!(convert_valid_glob("a**b"), "**/a*/**/*b");

        assert_eq!(convert_valid_glob("a**"), "**/a*/**");
        assert_eq!(convert_valid_glob("**a"), "**/*a");
        assert_eq!(convert_valid_glob("***"), "**/*");

        assert_eq!(convert_valid_glob(r"[\]a]"), r"**/[\]a]");
        assert_eq!(convert_valid_glob("[a/b]"), r"\[a/b]");

        assert_eq!(convert_valid_glob("{}"), r"**/\{\}");
        assert_eq!(convert_valid_glob("[a/b]{}"), r"\[a/b]\{\}");
        assert_eq!(convert_valid_glob("{a}"), r"**/\{a\}");
        assert_eq!(convert_valid_glob("{a,b"), r"**/\{a,b");

        assert_eq!(convert_valid_glob("{a,b}"), "**/{a,b}");
        assert_eq!(convert_valid_glob("{0,1}"), "**/{0,1}");
        assert_eq!(convert_valid_glob("{a,0..1}"), "**/{a,0..1}");

        assert_eq!(convert_valid_glob("{0..1}"), "**/{0,1}");
        assert_eq!(convert_valid_glob("{0..9}"), "**/{0,1,2,3,4,5,6,7,8,9}");
        assert_eq!(convert_valid_glob("{+1..+8}"), "**/{1,2,3,4,5,6,7,8}");
        assert_eq!(convert_valid_glob("{0..0}"), "**/{0}");

        assert_eq!(convert_valid_glob("{10..12}"), "**/{10,11,12}");
        assert_eq!(convert_valid_glob("{10..10}"), "**/{10}");

        assert_eq!(convert_valid_glob(r"\{0..0}"), r"**/\{0..0\}");
        assert_eq!(convert_valid_glob("{a..b}"), r"**/\{a..b\}");
    }

    // Editorconfig glob tests are ported from https://github.com/editorconfig/editorconfig-core-test/tree/master/glob

    #[test]
    fn test_editorconfig_glob_star() {
        let mut glob: EditorconfigGlob;

        // matches a single characters
        glob = "a*e.c".parse().unwrap();
        assert!(is_matching(&glob, "ace.c"));
        // matches zero characters
        assert!(is_matching(&glob, "ae.c"));
        // matches multiple characters
        assert!(is_matching(&glob, "abcde.c"));
        // does not match path separator
        assert!(!is_matching(&glob, "a/e.c"));

        // star after a slash
        glob = "Bar/*".parse().unwrap();
        assert!(is_matching(&glob, "Bar/foo.txt"));
        // star matches a dot file after slash
        assert!(is_matching(&glob, "Bar/.editorconfig"));
        // Slash makes the pattern not match in subdirectories
        assert!(!is_matching(&glob, "bat/Bar/foo.txt"));
        // Slash makes the pattern not match dotfiles in subdirectories
        assert!(!is_matching(&glob, "bat/Bar/.editorconfig"));

        // star matches a dot file
        glob = "*".parse().unwrap();
        assert!(is_matching(&glob, ".editorconfig"));
    }

    #[test]
    fn test_editorconfig_glob_question() {
        // matches a single characters
        let glob: EditorconfigGlob = "som?.c".parse().unwrap();
        assert!(is_matching(&glob, "some.c"));
        // does not match zero characters
        assert!(!is_matching(&glob, "som.c"));
        // does not match multiple characters
        assert!(!is_matching(&glob, "something.c"));
        // does not match slash
        assert!(!is_matching(&glob, "som/.c"));
    }

    #[test]
    fn test_editorconfig_glob_brackets() {
        // commented tests are tests that should pass and we don't support yet.
        let mut glob: EditorconfigGlob;

        // close bracket outside
        glob = r"[ab]].g".parse().unwrap();
        assert!(is_matching(&glob, "b].g"));

        // negative close bracket outside
        glob = r"[!ab]].g".parse().unwrap();
        assert!(is_matching(&glob, "c].g"));

        // character choice
        glob = "[ab].a".parse().unwrap();
        assert!(is_matching(&glob, "a.a"));
        assert!(is_matching(&glob, "b.a"));
        assert!(!is_matching(&glob, "c.a"));

        // negative character choice
        glob = "[!ab].b".parse().unwrap();
        assert!(is_matching(&glob, "c.b"));
        assert!(!is_matching(&glob, "a.b"));

        // character range
        glob = "[d-g].c".parse().unwrap();
        assert!(is_matching(&glob, "f.c"));
        assert!(!is_matching(&glob, "h.c"));

        // negative character range
        glob = "[!d-g].d".parse().unwrap();
        assert!(is_matching(&glob, "h.d"));
        assert!(!is_matching(&glob, "f.d"));

        // range and choice
        glob = "[abd-g].e".parse().unwrap();
        assert!(is_matching(&glob, "e.e"));

        // character choice with a dash
        glob = "[-ab].f".parse().unwrap();
        assert!(is_matching(&glob, "-.f"));

        // slash inside brackets
        glob = "ab[e/]cd.i".parse().unwrap();
        assert!(!is_matching(&glob, "ab/cd.i"));
        assert!(!is_matching(&glob, "abecd.i"));
        assert!(is_matching(&glob, "ab[e/]cd.i"));
        // slash after an half-open bracket
        glob = "ab[/c".parse().unwrap();
        assert!(is_matching(&glob, "ab[/c"));
    }

    #[test]
    #[ignore = "tests that should pass and we don't support yet"]
    fn test_editorconfig_glob_brackets_2() {
        let mut glob: EditorconfigGlob;

        // close bracket inside
        glob = r"[\]ab].g".parse().unwrap();
        assert!(is_matching(&glob, "].g"));

        // negative close bracket inside
        glob = r"[!\]ab].g".parse().unwrap();
        assert!(is_matching(&glob, "c.g"));
    }

    #[test]
    fn test_editorconfig_glob_alternates() {
        let mut glob: EditorconfigGlob;

        // word choices
        glob = "*.{py,js,html}".parse().unwrap();
        assert!(is_matching(&glob, "test.py"));
        assert!(is_matching(&glob, "test.js"));
        assert!(is_matching(&glob, "test.html"));
        assert!(!is_matching(&glob, "test.pyrc"));

        // single choice
        glob = "{single}.b".parse().unwrap();
        assert!(is_matching(&glob, "{single}.b"));
        assert!(!is_matching(&glob, ".b"));

        // empty choice
        glob = "{}.c".parse().unwrap();
        assert!(is_matching(&glob, "{}.c"));
        assert!(!is_matching(&glob, ".c"));

        // choice with empty words
        glob = "a{,b,c,}.d".parse().unwrap();
        //assert!(is_matching(&glob, "a.d"));
        assert!(is_matching(&glob, "ab.d"));
        assert!(is_matching(&glob, "ac.d"));
        assert!(!is_matching(&glob, "a,.d"));

        // no closing brace
        glob = "{.f".parse().unwrap();
        assert!(is_matching(&glob, "{.f"));
        assert!(!is_matching(&glob, ".f"));

        // closing inside beginning
        glob = "{},b}.h".parse().unwrap();
        assert!(is_matching(&glob, "{},b}.h"));

        // escaped comma
        glob = r"{a\,b,cd}.txt".parse().unwrap();
        assert!(is_matching(&glob, "a,b.txt"));
        assert!(!is_matching(&glob, "a.txt"));
        assert!(is_matching(&glob, "cd.txt"));

        // escaped closing brace
        glob = r"{e,\},f}.txt".parse().unwrap();
        assert!(is_matching(&glob, "e.txt"));
        assert!(is_matching(&glob, "}.txt"));
        assert!(is_matching(&glob, "f.txt"));

        // escaped backslash
        glob = r"{g,\\,i}.txt".parse().unwrap();
        assert!(is_matching(&glob, "g.txt"));
        assert!(is_matching(&glob, "i.txt"));

        // numeric brace range
        glob = "{3..120}".parse().unwrap();
        assert!(!is_matching(&glob, "1"));
        assert!(is_matching(&glob, "3"));
        assert!(is_matching(&glob, "15"));
        assert!(is_matching(&glob, "60"));
        assert!(!is_matching(&glob, "5a"));
        assert!(is_matching(&glob, "120"));
        assert!(!is_matching(&glob, "121"));
        assert!(!is_matching(&glob, "060"));

        // alphabetical brace range: letters should not be considered for ranges
        glob = "{aardvark..antelope}".parse().unwrap();
        assert!(is_matching(&glob, "{aardvark..antelope}"));
        assert!(!is_matching(&glob, "a"));
        assert!(!is_matching(&glob, "aardvark"));
        assert!(!is_matching(&glob, "agreement"));
        assert!(!is_matching(&glob, "antelope"));
        assert!(!is_matching(&glob, "antimatter"));
    }

    #[test]
    #[ignore = "tests that should pass and we don't support yet"]
    fn test_editorconfig_glob_alternates_2() {
        let mut glob: EditorconfigGlob;

        // nested braces
        glob = "{word,{also},this}.g".parse().unwrap();
        assert!(!is_matching(&glob, "word,this}.g"));
        assert!(!is_matching(&glob, "{also,this}.g"));
        assert!(is_matching(&glob, "word.g"));
        assert!(is_matching(&glob, "{also}.g"));
        assert!(is_matching(&glob, "this.g"));

        // nested braces, adjacent at start
        glob = "{{a,b},c}.k".parse().unwrap();
        assert!(!is_matching(&glob, "{{a,b},c}.k"));
        assert!(!is_matching(&glob, "{a,b}.k"));
        assert!(is_matching(&glob, "a.k"));
        assert!(is_matching(&glob, "b.k"));
        assert!(is_matching(&glob, "c.k"));

        // nested braces, adjacent at end
        glob = "{a,{b,c}}.l".parse().unwrap();
        assert!(!is_matching(&glob, "{a,{b,c}}.l"));
        assert!(!is_matching(&glob, "{b,c}.l"));
        assert!(is_matching(&glob, "a.l"));
        assert!(is_matching(&glob, "b.l"));
        assert!(is_matching(&glob, "c.l"));

        // missing closing braces
        glob = "{{,b,c{d}.i".parse().unwrap();
        assert!(is_matching(&glob, "{{,b,c{d}.i"));
        assert!(!is_matching(&glob, "{.i"));
        assert!(!is_matching(&glob, "b.i"));
        assert!(!is_matching(&glob, "c{d.i"));
        assert!(!is_matching(&glob, ".i"));

        // patterns nested in braces
        glob = "{some,a{*c,b}[ef]}.j".parse().unwrap();
        assert!(is_matching(&glob, "some.j"));
        assert!(is_matching(&glob, "abe.j"));
        assert!(is_matching(&glob, "abf.j"));
        assert!(!is_matching(&glob, "abg.j"));
        assert!(is_matching(&glob, "ace.j"));
        assert!(is_matching(&glob, "acf.j"));
        assert!(!is_matching(&glob, "acg.j"));
        assert!(is_matching(&glob, "abce.j"));
        assert!(is_matching(&glob, "abcf.j"));
        assert!(!is_matching(&glob, "abcg.j"));
        assert!(!is_matching(&glob, "ae.j"));
        assert!(!is_matching(&glob, ".j"));
    }

    #[test]
    fn test_editorconfig_glob_star_star() {
        // commented tests are tests that should pass and we don't support yet.
        let mut glob: EditorconfigGlob;

        // test EditorConfig files with UTF-8 characters larger than 127
        glob = "中文.txt".parse().unwrap();
        assert!(is_matching(&glob, "中文.txt"));

        // matches over path separator
        glob = "a**z.c".parse().unwrap();
        assert!(is_matching(&glob, "a/z.c"));
        //assert!(is_matching(&glob, "amnz.c"));
        assert!(is_matching(&glob, "am/nz.c"));
        assert!(is_matching(&glob, "a/mnz.c"));
        assert!(is_matching(&glob, "amn/z.c"));
        assert!(is_matching(&glob, "a/mn/z.c"));

        glob = "b/**z.c".parse().unwrap();
        assert!(is_matching(&glob, "b/z.c"));
        assert!(is_matching(&glob, "b/mnz.c"));
        assert!(is_matching(&glob, "b/mn/z.c"));
        assert!(!is_matching(&glob, "bmnz.c"));
        assert!(!is_matching(&glob, "bm/nz.c"));
        assert!(!is_matching(&glob, "bmn/z.c"));

        glob = "c**/z.c".parse().unwrap();
        assert!(is_matching(&glob, "c/z.c"));
        assert!(is_matching(&glob, "cmn/z.c"));
        assert!(is_matching(&glob, "c/mn/z.c"));
        assert!(!is_matching(&glob, "cmnz.c"));
        assert!(!is_matching(&glob, "cm/nz.c"));
        assert!(!is_matching(&glob, "c/mnz.c"));

        glob = "d/**/z.c".parse().unwrap();
        assert!(is_matching(&glob, "d/z.c"));
        assert!(is_matching(&glob, "d/mn/z.c"));
        assert!(!is_matching(&glob, "dmnz.c"));
        assert!(!is_matching(&glob, "dm/nz.c"));
        assert!(!is_matching(&glob, "d/mnz.c"));
        assert!(!is_matching(&glob, "dmn/z.c"));
    }
}
