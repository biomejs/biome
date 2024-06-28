use crate::matcher::pattern::CharSpecifier::{CharRange, SingleChar};
use crate::matcher::pattern::MatchResult::{
    EntirePatternDoesntMatch, Match, SubPatternDoesntMatch,
};
use crate::matcher::pattern::PatternToken::{
    AnyChar, AnyExcept, AnyPattern, AnyRecursiveSequence, AnySequence, AnyWithin, Char,
};
use std::error::Error;
use std::path::Path;
use std::str::FromStr;
use std::{fmt, path};

/// A pattern parsing error.
#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct PatternError {
    /// The approximate character index of where the error occurred.
    pub pos: usize,

    /// A message describing the error.
    pub msg: &'static str,
}

impl Error for PatternError {
    fn description(&self) -> &str {
        self.msg
    }
}

impl fmt::Display for PatternError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Pattern syntax error near position {}: {}",
            self.pos, self.msg
        )
    }
}

/// A compiled Unix shell style pattern.
///
/// - `?` matches any single character.
///
/// - `*` matches any (possibly empty) sequence of characters.
///
/// - `**` matches the current directory and arbitrary subdirectories. This
///   sequence **must** form a single path component, so both `**a` and `b**`
///   are invalid and will result in an error.  A sequence of more than two
///   consecutive `*` characters is also invalid.
///
/// - `[...]` matches any character inside the brackets.  Character sequences
///   can also specify ranges of characters, as ordered by Unicode, so e.g.
///   `[0-9]` specifies any character between 0 and 9 inclusive. An unclosed
///   bracket is invalid.
///
/// - `[!...]` is the negation of `[...]`, i.e. it matches any characters
///   **not** in the brackets.
///
/// - The metacharacters `?`, `*`, `[`, `]` can be matched by using brackets
///   (e.g. `[?]`).  When a `]` occurs immediately following `[` or `[!` then it
///   is interpreted as being part of, rather then ending, the character set, so
///   `]` and NOT `]` can be matched by `[]]` and `[!]]` respectively.  The `-`
///   character can be specified inside a character sequence pattern by placing
///   it at the start or the end, e.g. `[abc-]`.
///
/// - `{...}` can be used to specify multiple patterns separated by commas. For
///   example, `a/{b,c}/d` will match `a/b/d` and `a/c/d`.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct Pattern {
    /// The original glob pattern that was parsed to create this `Pattern`.
    original: String,
    tokens: Vec<PatternToken>,
    is_recursive: bool,
    /// Did this pattern come from an `.editorconfig` file?
    ///
    /// TODO: Remove this flag and support `{a,b}` globs in Biome 2.0
    is_editorconfig: bool,
}

/// Show the original glob pattern.
impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.original.fmt(f)
    }
}

impl FromStr for Pattern {
    type Err = PatternError;

    fn from_str(s: &str) -> Result<Self, PatternError> {
        Self::new(s)
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum PatternToken {
    Char(char),
    AnyChar,
    AnySequence,
    AnyRecursiveSequence,
    AnyWithin(Vec<CharSpecifier>),
    AnyExcept(Vec<CharSpecifier>),
    /// A set of patterns that at least one of them must match
    AnyPattern(Vec<Pattern>),
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum CharSpecifier {
    SingleChar(char),
    CharRange(char, char),
}

#[derive(Copy, Clone, PartialEq)]
enum MatchResult {
    Match,
    SubPatternDoesntMatch,
    EntirePatternDoesntMatch,
}

const ERROR_WILDCARDS: &str = "wildcards are either regular `*` or recursive `**`";
const ERROR_RECURSIVE_WILDCARDS: &str = "recursive wildcards must form a single path \
                                         component";
const ERROR_INVALID_RANGE: &str = "invalid range pattern";

impl Pattern {
    /// This function compiles Unix shell style patterns.
    ///
    /// An invalid glob pattern will yield a `PatternError`.
    pub fn new(pattern: &str) -> Result<Self, PatternError> {
        Self::parse(pattern, false)
    }

    /// This function compiles Unix shell style patterns.
    ///
    /// An invalid glob pattern will yield a `PatternError`.
    pub fn parse(pattern: &str, is_editorconfig: bool) -> Result<Self, PatternError> {
        let chars = pattern.chars().collect::<Vec<_>>();
        let mut tokens = Vec::new();
        let mut is_recursive = false;
        let mut i = 0;

        // A pattern is relative if it starts with "." followed by a separator,
        // eg. "./test" or ".\test"
        let is_relative = matches!(chars.get(..2), Some(['.', sep]) if path::is_separator(*sep));
        if is_relative {
            // If a pattern starts with a relative prefix, strip it from the
            // pattern and replace it with a "**" sequence
            i += 2;
            tokens.push(AnyRecursiveSequence);
        } else {
            // A pattern is absolute if it starts with a path separator, eg. "/home" or "\\?\C:\Users"
            let mut is_absolute = chars.first().map_or(false, |c| path::is_separator(*c));

            // On windows a pattern may also be absolute if it starts with a
            // drive letter, a colon and a separator, eg. "c:/Users" or "G:\Users"
            if cfg!(windows) && !is_absolute {
                is_absolute = matches!(chars.get(..3), Some(['a'..='z' | 'A'..='Z', ':', sep]) if path::is_separator(*sep));
            }

            // If a pattern is not absolute, insert a "**" sequence in front
            if !is_absolute {
                tokens.push(AnyRecursiveSequence);
            }
        }

        while i < chars.len() {
            match chars[i] {
                '?' => {
                    tokens.push(AnyChar);
                    i += 1;
                }
                '*' => {
                    let old = i;

                    while i < chars.len() && chars[i] == '*' {
                        i += 1;
                    }

                    let count = i - old;

                    match count {
                        count if count > 2 => {
                            return Err(PatternError {
                                pos: old + 2,
                                msg: ERROR_WILDCARDS,
                            });
                        }
                        count if count == 2 => {
                            // ** can only be an entire path component
                            // i.e. a/**/b is valid, but a**/b or a/**b is not
                            // invalid matches are treated literally
                            let is_valid = if i == 2 || path::is_separator(chars[i - count - 1]) {
                                // it ends in a '/'
                                if i < chars.len() && path::is_separator(chars[i]) {
                                    i += 1;
                                    true
                                    // or the pattern ends here
                                    // this enables the existing globbing mechanism
                                } else if i == chars.len() {
                                    true
                                    // `**` ends in non-separator
                                } else {
                                    return Err(PatternError {
                                        pos: i,
                                        msg: ERROR_RECURSIVE_WILDCARDS,
                                    });
                                }
                                // `**` begins with non-separator
                            } else {
                                return Err(PatternError {
                                    pos: old - 1,
                                    msg: ERROR_RECURSIVE_WILDCARDS,
                                });
                            };

                            if is_valid {
                                // collapse consecutive AnyRecursiveSequence to a
                                // single one

                                let tokens_len = tokens.len();

                                if !(tokens_len > 1
                                    && tokens[tokens_len - 1] == AnyRecursiveSequence)
                                {
                                    is_recursive = true;
                                    tokens.push(AnyRecursiveSequence);
                                }
                            }
                        }
                        _ => {
                            tokens.push(AnySequence);
                        }
                    }
                }
                '[' => {
                    if i + 4 <= chars.len() && chars[i + 1] == '!' {
                        match chars[i + 3..].iter().position(|x| *x == ']') {
                            None => (),
                            Some(j) => {
                                let chars = &chars[i + 2..i + 3 + j];
                                let cs = parse_char_specifiers(chars);
                                tokens.push(AnyExcept(cs));
                                i += j + 4;
                                continue;
                            }
                        }
                    } else if i + 3 <= chars.len() && chars[i + 1] != '!' {
                        match chars[i + 2..].iter().position(|x| *x == ']') {
                            None => (),
                            Some(j) => {
                                let cs = parse_char_specifiers(&chars[i + 1..i + 2 + j]);
                                tokens.push(AnyWithin(cs));
                                i += j + 3;
                                continue;
                            }
                        }
                    }

                    // if we get here then this is not a valid range pattern
                    return Err(PatternError {
                        pos: i,
                        msg: ERROR_INVALID_RANGE,
                    });
                }
                '{' if is_editorconfig => {
                    let mut depth = 1;
                    let mut j = i + 1;
                    while j < chars.len() {
                        match chars[j] {
                            '{' => depth += 1,
                            '}' => depth -= 1,
                            _ => (),
                        }
                        if depth > 1 {
                            return Err(PatternError {
                                pos: j,
                                msg: "nested '{' in '{...}' is not allowed",
                            });
                        }
                        if depth == 0 {
                            break;
                        }
                        j += 1;
                    }

                    if depth != 0 {
                        return Err(PatternError {
                            pos: i,
                            msg: "unmatched '{'",
                        });
                    }

                    let mut subpatterns = Vec::new();
                    for subpattern in pattern[i + 1..j].split(',') {
                        let mut pattern = Pattern::new(subpattern)?;
                        // HACK: remove the leading '**' if it exists
                        if pattern.tokens.first() == Some(&PatternToken::AnyRecursiveSequence) {
                            pattern.tokens.remove(0);
                        }
                        subpatterns.push(pattern);
                    }
                    tokens.push(AnyPattern(subpatterns));
                    i = j + 1;
                }
                c => {
                    tokens.push(Char(c));
                    i += 1;
                }
            }
        }

        Ok(Self {
            tokens,
            original: pattern.to_string(),
            is_recursive,
            is_editorconfig,
        })
    }

    fn from_tokens(tokens: Vec<PatternToken>, original: String, is_recursive: bool) -> Self {
        Self {
            tokens,
            original,
            is_recursive,
            is_editorconfig: false,
        }
    }

    /// Escape metacharacters within the given string by surrounding them in
    /// brackets. The resulting string will, when compiled into a `Pattern`,
    /// match the input string and nothing else.
    pub fn escape(s: &str) -> String {
        let mut escaped = String::new();
        for c in s.chars() {
            match c {
                // note that ! does not need escaping because it is only special
                // inside brackets
                '?' | '*' | '[' | ']' => {
                    escaped.push('[');
                    escaped.push(c);
                    escaped.push(']');
                }
                c => {
                    escaped.push(c);
                }
            }
        }
        escaped
    }

    /// Return if the given `str` matches this `Pattern` using the default
    /// match options (i.e. `MatchOptions::new()`).
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use crate::Pattern;
    ///
    /// assert!(Pattern::new("c?t").unwrap().matches("cat"));
    /// assert!(Pattern::new("k[!e]tteh").unwrap().matches("kitteh"));
    /// assert!(Pattern::new("d*g").unwrap().matches("doog"));
    /// ```
    pub fn matches(&self, str: &str) -> bool {
        self.matches_with(str, MatchOptions::new())
    }

    /// Return if the given `Path`, when converted to a `str`, matches this
    /// `Pattern` using the default match options (i.e. `MatchOptions::new()`).
    pub fn matches_path(&self, path: &Path) -> bool {
        // FIXME (#9639): This needs to handle non-utf8 paths
        path.to_str().map_or(false, |s| self.matches(s))
    }

    /// Return if the given `str` matches this `Pattern` using the specified
    /// match options.
    pub fn matches_with(&self, str: &str, options: MatchOptions) -> bool {
        self.matches_from(true, str.chars(), 0, options) == Match
    }

    /// Return if the given `Path`, when converted to a `str`, matches this
    /// `Pattern` using the specified match options.
    pub fn matches_path_with(&self, path: &Path, options: MatchOptions) -> bool {
        // FIXME (#9639): This needs to handle non-utf8 paths
        path.to_str()
            .map_or(false, |s| self.matches_with(s, options))
    }

    /// Access the original glob pattern.
    pub fn as_str(&self) -> &str {
        &self.original
    }

    fn matches_from(
        &self,
        mut follows_separator: bool,
        mut file: std::str::Chars,
        i: usize,
        options: MatchOptions,
    ) -> MatchResult {
        for (ti, token) in self.tokens[i..].iter().enumerate() {
            match token {
                AnySequence | AnyRecursiveSequence => {
                    // ** must be at the start.
                    debug_assert!(match *token {
                        AnyRecursiveSequence => follows_separator,
                        _ => true,
                    });

                    // Empty match
                    match self.matches_from(follows_separator, file.clone(), i + ti + 1, options) {
                        SubPatternDoesntMatch => (), // keep trying
                        m => return m,
                    };

                    while let Some(c) = file.next() {
                        if follows_separator && options.require_literal_leading_dot && c == '.' {
                            return SubPatternDoesntMatch;
                        }
                        follows_separator = path::is_separator(c);
                        match *token {
                            AnyRecursiveSequence if !follows_separator => continue,
                            AnySequence
                                if options.require_literal_separator && follows_separator =>
                            {
                                return SubPatternDoesntMatch
                            }
                            _ => (),
                        }
                        match self.matches_from(
                            follows_separator,
                            file.clone(),
                            i + ti + 1,
                            options,
                        ) {
                            SubPatternDoesntMatch => (), // keep trying
                            m => return m,
                        }
                    }
                }
                AnyPattern(patterns) => {
                    for pattern in patterns.iter() {
                        let mut tokens = pattern.tokens.clone();
                        tokens.extend_from_slice(&self.tokens[(i + ti + 1)..]);
                        let new_pattern = Pattern::from_tokens(
                            tokens,
                            pattern.original.clone(),
                            pattern.is_recursive,
                        );
                        if new_pattern.matches_from(follows_separator, file.clone(), 0, options)
                            == Match
                        {
                            return Match;
                        }
                    }
                    return SubPatternDoesntMatch;
                }
                _ => {
                    let c = match file.next() {
                        Some(c) => c,
                        None => return EntirePatternDoesntMatch,
                    };

                    let is_sep = path::is_separator(c);

                    if !match *token {
                        AnyChar | AnyWithin(..) | AnyExcept(..)
                            if (options.require_literal_separator && is_sep)
                                || (follows_separator
                                    && options.require_literal_leading_dot
                                    && c == '.') =>
                        {
                            false
                        }
                        AnyChar => true,
                        AnyWithin(ref specifiers) => in_char_specifiers(specifiers, c, options),
                        AnyExcept(ref specifiers) => !in_char_specifiers(specifiers, c, options),
                        Char(c2) => chars_eq(c, c2, options.case_sensitive),
                        AnySequence | AnyRecursiveSequence | AnyPattern(_) => unreachable!(),
                    } {
                        return SubPatternDoesntMatch;
                    }
                    follows_separator = is_sep;
                }
            }
        }

        // Iter is fused.
        if file.next().is_none() {
            Match
        } else {
            SubPatternDoesntMatch
        }
    }
}

fn parse_char_specifiers(s: &[char]) -> Vec<CharSpecifier> {
    let mut cs = Vec::new();
    let mut i = 0;
    while i < s.len() {
        if i + 3 <= s.len() && s[i + 1] == '-' {
            cs.push(CharRange(s[i], s[i + 2]));
            i += 3;
        } else {
            cs.push(SingleChar(s[i]));
            i += 1;
        }
    }
    cs
}

fn in_char_specifiers(specifiers: &[CharSpecifier], c: char, options: MatchOptions) -> bool {
    for &specifier in specifiers.iter() {
        match specifier {
            SingleChar(sc) => {
                if chars_eq(c, sc, options.case_sensitive) {
                    return true;
                }
            }
            CharRange(start, end) => {
                // FIXME: work with non-ascii chars properly (issue #1347)
                if !options.case_sensitive && c.is_ascii() && start.is_ascii() && end.is_ascii() {
                    let start = start.to_ascii_lowercase();
                    let end = end.to_ascii_lowercase();

                    let start_up = start.to_uppercase().next().unwrap();
                    let end_up = end.to_uppercase().next().unwrap();

                    // only allow case insensitive matching when
                    // both start and end are within a-z or A-Z
                    if start != start_up && end != end_up {
                        let c = c.to_ascii_lowercase();
                        if c >= start && c <= end {
                            return true;
                        }
                    }
                }

                if c >= start && c <= end {
                    return true;
                }
            }
        }
    }

    false
}

/// A helper function to determine if two chars are (possibly case-insensitively) equal.
fn chars_eq(a: char, b: char, case_sensitive: bool) -> bool {
    if cfg!(windows) && path::is_separator(a) && path::is_separator(b) {
        true
    } else if !case_sensitive && a.is_ascii() && b.is_ascii() {
        // FIXME: work with non-ascii chars properly (issue #9084)
        a.to_ascii_lowercase() == b.to_ascii_lowercase()
    } else {
        a == b
    }
}

/// Configuration options to modify the behaviour of `Pattern::matches_with(..)`.
#[allow(missing_copy_implementations)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MatchOptions {
    /// Whether or not patterns should be matched in a case-sensitive manner.
    /// This currently only considers upper/lower case relationships between
    /// ASCII characters, but in future this might be extended to work with
    /// Unicode.
    pub case_sensitive: bool,

    /// Whether or not path-component separator characters (e.g. `/` on
    /// Posix) must be matched by a literal `/`, rather than by `*` or `?` or
    /// `[...]`.
    pub require_literal_separator: bool,

    /// Whether or not paths that contain components that start with a `.`
    /// will require that `.` appears literally in the pattern; `*`, `?`, `**`,
    /// or `[...]` will not match. This is useful because such files are
    /// conventionally considered hidden on Unix systems and it might be
    /// desirable to skip them when listing files.
    pub require_literal_leading_dot: bool,
}

impl MatchOptions {
    /// Constructs a new `MatchOptions` with default field values. This is used
    /// when calling functions that do not take an explicit `MatchOptions`
    /// parameter.
    ///
    /// This function always returns this value:
    ///
    /// ```rust,ignore
    /// MatchOptions {
    ///     case_sensitive: true,
    ///     require_literal_separator: false,
    ///     require_literal_leading_dot: false
    /// }
    /// ```
    pub fn new() -> Self {
        Self {
            case_sensitive: true,
            require_literal_separator: false,
            require_literal_leading_dot: false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{MatchOptions, Pattern};
    use std::path::Path;

    #[test]
    fn test_pattern_from_str() {
        assert!("a*b".parse::<Pattern>().unwrap().matches("a_b"));
        assert!("a/**b".parse::<Pattern>().unwrap_err().pos == 4);
    }

    #[test]
    fn test_wildcard_errors() {
        assert!(Pattern::new("a/**b").unwrap_err().pos == 4);
        assert!(Pattern::new("a/bc**").unwrap_err().pos == 3);
        assert!(Pattern::new("a/*****").unwrap_err().pos == 4);
        assert!(Pattern::new("a/b**c**d").unwrap_err().pos == 2);
        assert!(Pattern::new("a**b").unwrap_err().pos == 0);
    }

    #[test]
    fn test_unclosed_bracket_errors() {
        assert!(Pattern::new("abc[def").unwrap_err().pos == 3);
        assert!(Pattern::new("abc[!def").unwrap_err().pos == 3);
        assert!(Pattern::new("abc[").unwrap_err().pos == 3);
        assert!(Pattern::new("abc[!").unwrap_err().pos == 3);
        assert!(Pattern::new("abc[d").unwrap_err().pos == 3);
        assert!(Pattern::new("abc[!d").unwrap_err().pos == 3);
        assert!(Pattern::new("abc[]").unwrap_err().pos == 3);
        assert!(Pattern::new("abc[!]").unwrap_err().pos == 3);
    }

    #[test]
    fn test_wildcards() {
        assert!(Pattern::new("a*b").unwrap().matches("a_b"));
        assert!(Pattern::new("a*b*c").unwrap().matches("abc"));
        assert!(!Pattern::new("a*b*c").unwrap().matches("abcd"));
        assert!(Pattern::new("a*b*c").unwrap().matches("a_b_c"));
        assert!(Pattern::new("a*b*c").unwrap().matches("a___b___c"));
        assert!(Pattern::new("abc*abc*abc")
            .unwrap()
            .matches("abcabcabcabcabcabcabc"));
        assert!(!Pattern::new("abc*abc*abc")
            .unwrap()
            .matches("abcabcabcabcabcabcabca"));
        assert!(Pattern::new("a*a*a*a*a*a*a*a*a")
            .unwrap()
            .matches("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
        assert!(Pattern::new("a*b[xyz]c*d").unwrap().matches("abxcdbxcddd"));
    }

    #[test]
    fn test_recursive_wildcards() {
        let pat = Pattern::new("some/**/needle.txt").unwrap();
        assert!(pat.matches("some/needle.txt"));
        assert!(pat.matches("some/one/needle.txt"));
        assert!(pat.matches("some/one/two/needle.txt"));
        assert!(pat.matches("some/other/needle.txt"));
        assert!(!pat.matches("some/other/notthis.txt"));

        // a single ** should be valid, for globs
        // Should accept anything
        let pat = Pattern::new("**").unwrap();
        assert!(pat.is_recursive);
        assert!(pat.matches("abcde"));
        assert!(pat.matches(""));
        assert!(pat.matches(".asdf"));
        assert!(pat.matches("/x/.asdf"));

        // collapse consecutive wildcards
        let pat = Pattern::new("some/**/**/needle.txt").unwrap();
        assert!(pat.matches("some/needle.txt"));
        assert!(pat.matches("some/one/needle.txt"));
        assert!(pat.matches("some/one/two/needle.txt"));
        assert!(pat.matches("some/other/needle.txt"));
        assert!(!pat.matches("some/other/notthis.txt"));

        // ** can begin the pattern
        let pat = Pattern::new("**/test").unwrap();
        assert!(pat.matches("one/two/test"));
        assert!(pat.matches("one/test"));
        assert!(pat.matches("test"));

        // /** can begin the pattern
        let pat = Pattern::new("/**/test").unwrap();
        assert!(pat.matches("/one/two/test"));
        assert!(pat.matches("/one/test"));
        assert!(pat.matches("/test"));
        assert!(!pat.matches("/one/notthis"));
        assert!(!pat.matches("/notthis"));

        // Only start sub-patterns on start of path segment.
        let pat = Pattern::new("**/.*").unwrap();
        assert!(pat.matches(".abc"));
        assert!(pat.matches("abc/.abc"));
        assert!(!pat.matches("ab.c"));
        assert!(!pat.matches("abc/ab.c"));
    }

    #[test]
    fn test_range_pattern() {
        let pat = Pattern::new("a[0-9]b").unwrap();
        for i in 0..10 {
            assert!(pat.matches(&format!("a{i}b")));
        }
        assert!(!pat.matches("a_b"));

        let pat = Pattern::new("a[!0-9]b").unwrap();
        for i in 0..10 {
            assert!(!pat.matches(&format!("a{i}b")));
        }
        assert!(pat.matches("a_b"));

        let pats = ["[a-z123]", "[1a-z23]", "[123a-z]"];
        for &p in pats.iter() {
            let pat = Pattern::new(p).unwrap();
            for c in "abcdefghijklmnopqrstuvwxyz".chars() {
                assert!(pat.matches(&c.to_string()));
            }
            for c in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
                let options = MatchOptions {
                    case_sensitive: false,
                    ..MatchOptions::new()
                };
                assert!(pat.matches_with(&c.to_string(), options));
            }
            assert!(pat.matches("1"));
            assert!(pat.matches("2"));
            assert!(pat.matches("3"));
        }

        let pats = ["[abc-]", "[-abc]", "[a-c-]"];
        for &p in pats.iter() {
            let pat = Pattern::new(p).unwrap();
            assert!(pat.matches("a"));
            assert!(pat.matches("b"));
            assert!(pat.matches("c"));
            assert!(pat.matches("-"));
            assert!(!pat.matches("d"));
        }

        let pat = Pattern::new("[2-1]").unwrap();
        assert!(!pat.matches("1"));
        assert!(!pat.matches("2"));

        assert!(Pattern::new("[-]").unwrap().matches("-"));
        assert!(!Pattern::new("[!-]").unwrap().matches("-"));
    }

    #[test]
    fn test_pattern_matches() {
        let txt_pat = Pattern::new("*hello.txt").unwrap();
        assert!(txt_pat.matches("hello.txt"));
        assert!(txt_pat.matches("gareth_says_hello.txt"));
        assert!(txt_pat.matches("some/path/to/hello.txt"));
        assert!(txt_pat.matches("some\\path\\to\\hello.txt"));
        assert!(txt_pat.matches("/an/absolute/path/to/hello.txt"));
        assert!(!txt_pat.matches("hello.txt-and-then-some"));
        assert!(!txt_pat.matches("goodbye.txt"));

        let dir_pat = Pattern::new("*some/path/to/hello.txt").unwrap();
        assert!(dir_pat.matches("some/path/to/hello.txt"));
        assert!(dir_pat.matches("a/bigger/some/path/to/hello.txt"));
        assert!(!dir_pat.matches("some/path/to/hello.txt-and-then-some"));
        assert!(!dir_pat.matches("some/other/path/to/hello.txt"));
    }

    #[test]
    fn test_pattern_escape() {
        let s = "_[_]_?_*_!_";
        assert_eq!(Pattern::escape(s), "_[[]_[]]_[?]_[*]_!_".to_string());
        assert!(Pattern::new(&Pattern::escape(s)).unwrap().matches(s));
    }

    #[test]
    fn test_pattern_matches_case_insensitive() {
        let pat = Pattern::new("aBcDeFg").unwrap();
        let options = MatchOptions {
            case_sensitive: false,
            require_literal_separator: false,
            require_literal_leading_dot: false,
        };

        assert!(pat.matches_with("aBcDeFg", options));
        assert!(pat.matches_with("abcdefg", options));
        assert!(pat.matches_with("ABCDEFG", options));
        assert!(pat.matches_with("AbCdEfG", options));
    }

    #[test]
    fn test_pattern_matches_case_insensitive_range() {
        let pat_within = Pattern::new("[a]").unwrap();
        let pat_except = Pattern::new("[!a]").unwrap();

        let options_case_insensitive = MatchOptions {
            case_sensitive: false,
            require_literal_separator: false,
            require_literal_leading_dot: false,
        };
        let options_case_sensitive = MatchOptions {
            case_sensitive: true,
            require_literal_separator: false,
            require_literal_leading_dot: false,
        };

        assert!(pat_within.matches_with("a", options_case_insensitive));
        assert!(pat_within.matches_with("A", options_case_insensitive));
        assert!(!pat_within.matches_with("A", options_case_sensitive));

        assert!(!pat_except.matches_with("a", options_case_insensitive));
        assert!(!pat_except.matches_with("A", options_case_insensitive));
        assert!(pat_except.matches_with("A", options_case_sensitive));
    }

    #[test]
    fn test_pattern_matches_require_literal_separator() {
        let options_require_literal = MatchOptions {
            case_sensitive: true,
            require_literal_separator: true,
            require_literal_leading_dot: false,
        };
        let options_not_require_literal = MatchOptions {
            case_sensitive: true,
            require_literal_separator: false,
            require_literal_leading_dot: false,
        };

        assert!(Pattern::new("abc/def")
            .unwrap()
            .matches_with("abc/def", options_require_literal));
        assert!(!Pattern::new("abc?def")
            .unwrap()
            .matches_with("abc/def", options_require_literal));
        assert!(!Pattern::new("abc*def")
            .unwrap()
            .matches_with("abc/def", options_require_literal));
        assert!(!Pattern::new("abc[/]def")
            .unwrap()
            .matches_with("abc/def", options_require_literal));

        assert!(Pattern::new("abc/def")
            .unwrap()
            .matches_with("abc/def", options_not_require_literal));
        assert!(Pattern::new("abc?def")
            .unwrap()
            .matches_with("abc/def", options_not_require_literal));
        assert!(Pattern::new("abc*def")
            .unwrap()
            .matches_with("abc/def", options_not_require_literal));
        assert!(Pattern::new("abc[/]def")
            .unwrap()
            .matches_with("abc/def", options_not_require_literal));
    }

    #[test]
    fn test_pattern_matches_require_literal_leading_dot() {
        let options_require_literal_leading_dot = MatchOptions {
            case_sensitive: true,
            require_literal_separator: false,
            require_literal_leading_dot: true,
        };
        let options_not_require_literal_leading_dot = MatchOptions {
            case_sensitive: true,
            require_literal_separator: false,
            require_literal_leading_dot: false,
        };

        let f = |options| {
            Pattern::new("*.txt")
                .unwrap()
                .matches_with(".hello.txt", options)
        };
        assert!(f(options_not_require_literal_leading_dot));
        assert!(!f(options_require_literal_leading_dot));

        let f = |options| {
            Pattern::new(".*.*")
                .unwrap()
                .matches_with(".hello.txt", options)
        };
        assert!(f(options_not_require_literal_leading_dot));
        assert!(f(options_require_literal_leading_dot));

        let f = |options| {
            Pattern::new("aaa/bbb/*")
                .unwrap()
                .matches_with("aaa/bbb/.ccc", options)
        };
        assert!(f(options_not_require_literal_leading_dot));
        assert!(!f(options_require_literal_leading_dot));

        let f = |options| {
            Pattern::new("aaa/bbb/*")
                .unwrap()
                .matches_with("aaa/bbb/c.c.c.", options)
        };
        assert!(f(options_not_require_literal_leading_dot));
        assert!(f(options_require_literal_leading_dot));

        let f = |options| {
            Pattern::new("aaa/bbb/.*")
                .unwrap()
                .matches_with("aaa/bbb/.ccc", options)
        };
        assert!(f(options_not_require_literal_leading_dot));
        assert!(f(options_require_literal_leading_dot));

        let f = |options| {
            Pattern::new("aaa/?bbb")
                .unwrap()
                .matches_with("aaa/.bbb", options)
        };
        assert!(f(options_not_require_literal_leading_dot));
        assert!(!f(options_require_literal_leading_dot));

        let f = |options| {
            Pattern::new("aaa/[.]bbb")
                .unwrap()
                .matches_with("aaa/.bbb", options)
        };
        assert!(f(options_not_require_literal_leading_dot));
        assert!(!f(options_require_literal_leading_dot));

        let f = |options| Pattern::new("**/*").unwrap().matches_with(".bbb", options);
        assert!(f(options_not_require_literal_leading_dot));
        assert!(!f(options_require_literal_leading_dot));
    }

    #[test]
    fn test_matches_path() {
        // on windows, (Path::new("a/b").as_str().unwrap() == "a\\b"), so this
        // tests that / and \ are considered equivalent on windows
        assert!(Pattern::new("a/b").unwrap().matches_path(Path::new("a/b")));
    }

    #[test]
    fn test_path_join() {
        let pattern = Path::new("one").join(Path::new("**/*.rs"));
        assert!(Pattern::new(pattern.to_str().unwrap()).is_ok());
    }

    #[test]
    fn test_pattern_relative() {
        assert!(Pattern::new("./b").unwrap().matches_path(Path::new("a/b")));
        assert!(Pattern::new("b").unwrap().matches_path(Path::new("a/b")));

        if cfg!(windows) {
            assert!(Pattern::new(".\\b")
                .unwrap()
                .matches_path(Path::new("a\\b")));
            assert!(Pattern::new("b").unwrap().matches_path(Path::new("a\\b")));
        }
    }

    #[test]
    fn test_pattern_absolute() {
        assert!(Pattern::new("/a/b")
            .unwrap()
            .matches_path(Path::new("/a/b")));

        if cfg!(windows) {
            assert!(Pattern::new("c:/a/b")
                .unwrap()
                .matches_path(Path::new("c:/a/b")));
            assert!(Pattern::new("C:\\a\\b")
                .unwrap()
                .matches_path(Path::new("C:\\a\\b")));

            assert!(Pattern::new("\\\\?\\c:\\a\\b")
                .unwrap()
                .matches_path(Path::new("\\\\?\\c:\\a\\b")));
            assert!(Pattern::new("\\\\?\\C:/a/b")
                .unwrap()
                .matches_path(Path::new("\\\\?\\C:/a/b")));
        }
    }

    #[test]
    fn test_pattern_glob() {
        assert!(Pattern::new("*.js")
            .unwrap()
            .matches_path(Path::new("b/c.js")));

        assert!(Pattern::new("**/*.js")
            .unwrap()
            .matches_path(Path::new("b/c.js")));

        assert!(Pattern::new("*.js")
            .unwrap()
            .matches_path(Path::new("/a/b/c.js")));

        assert!(Pattern::new("**/*.js")
            .unwrap()
            .matches_path(Path::new("/a/b/c.js")));

        if cfg!(windows) {
            assert!(Pattern::new("*.js")
                .unwrap()
                .matches_path(Path::new("C:\\a\\b\\c.js")));

            assert!(Pattern::new("**/*.js")
                .unwrap()
                .matches_path(Path::new("\\\\?\\C:\\a\\b\\c.js")));
        }
    }

    #[test]
    fn test_pattern_glob_brackets() {
        let pattern = Pattern::parse("{foo.js,bar.js}", true).unwrap();
        assert!(pattern.matches_path(Path::new("foo.js")));
        assert!(pattern.matches_path(Path::new("bar.js")));
        assert!(!pattern.matches_path(Path::new("baz.js")));

        let pattern = Pattern::parse("{foo,bar}.js", true).unwrap();
        assert!(pattern.matches_path(Path::new("foo.js")));
        assert!(pattern.matches_path(Path::new("bar.js")));
        assert!(!pattern.matches_path(Path::new("baz.js")));

        assert!(Pattern::parse("**/{foo,bar}.js", true)
            .unwrap()
            .matches_path(Path::new("a/b/foo.js")));

        let pattern = Pattern::parse("src/{a/foo,bar}.js", true).unwrap();
        assert!(pattern.matches_path(Path::new("src/a/foo.js")));
        assert!(pattern.matches_path(Path::new("src/bar.js")));
        assert!(!pattern.matches_path(Path::new("src/a/b/foo.js")));
        assert!(!pattern.matches_path(Path::new("src/a/bar.js")));

        let pattern = Pattern::parse("src/{a,b}/{c,d}/foo.js", true).unwrap();
        assert!(pattern.matches_path(Path::new("src/a/c/foo.js")));
        assert!(pattern.matches_path(Path::new("src/a/d/foo.js")));
        assert!(pattern.matches_path(Path::new("src/b/c/foo.js")));
        assert!(pattern.matches_path(Path::new("src/b/d/foo.js")));
        assert!(!pattern.matches_path(Path::new("src/bar/foo.js")));

        let _ = Pattern::parse("{{foo,bar},baz}", true)
            .expect_err("should not allow curly brackets more than 1 level deep");
    }

    #[test]
    fn test_pattern_glob_brackets_not_available_by_default() {
        // RODO: Remove this test when we make brackets available by default in Biome 2.0
        let pattern = Pattern::parse("{foo.js,bar.js}", false).unwrap();
        assert!(!pattern.matches_path(Path::new("foo.js")));
        assert!(!pattern.matches_path(Path::new("bar.js")));
        assert!(!pattern.matches_path(Path::new("baz.js")));
    }
}
