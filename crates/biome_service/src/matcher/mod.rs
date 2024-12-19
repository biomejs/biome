pub mod pattern;

use biome_configuration::BiomeDiagnostic;
use biome_console::markup;
use biome_diagnostics::Diagnostic;
use papaya::HashMap;
pub use pattern::{MatchOptions, Pattern, PatternError};
use rustc_hash::FxBuildHasher;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::WorkspaceError;

/// A data structure to use when there's need to match a string or a path a against
/// a unix shell style patterns
#[derive(Clone, Debug, Default)]
pub struct Matcher(Arc<Inner>);

impl Matcher {
    pub fn empty() -> Self {
        Self::default()
    }

    /// Creates a [Matcher] from a set of globs.
    ///
    /// ## Errors
    ///
    /// It can raise an error if the patterns aren't valid
    pub fn from_globs(
        working_directory: Option<PathBuf>,
        globs: Option<&[Box<str>]>,
    ) -> Result<Matcher, WorkspaceError> {
        let mut matcher = Inner::default();
        if let Some(working_directory) = working_directory {
            matcher.set_root(working_directory)
        }
        if let Some(string_set) = globs {
            for pattern in string_set {
                matcher.add_pattern(pattern).map_err(|err| {
                    BiomeDiagnostic::new_invalid_ignore_pattern(
                        pattern.to_string(),
                        err.msg.to_string(),
                    )
                })?;
            }
        }
        Ok(Self(Arc::new(matcher)))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Matches the given string against the stored patterns.
    ///
    /// Returns [true] if there's at least one match.
    pub fn matches(&self, source: &str) -> bool {
        self.0.matches(source)
    }

    /// Matches the given path against the stored patterns.
    ///
    /// Returns [true] if there's at least one match.
    pub fn matches_path(&self, source: &Path) -> bool {
        self.0.matches_path(source)
    }
}

#[derive(Clone, Debug, Default)]
struct Inner {
    root: Option<PathBuf>,
    patterns: Vec<Pattern>,
    /// Check [glob website](https://docs.rs/glob/latest/glob/struct.MatchOptions.html) for [MatchOptions]
    options: MatchOptions,
    /// Cached results for matches.
    already_checked: HashMap<String, bool, FxBuildHasher>,
}

impl Inner {
    /// Creates a new Matcher with given options.
    #[cfg(test)]
    fn new(options: MatchOptions) -> Self {
        Self {
            root: None,
            patterns: Vec::new(),
            options,
            already_checked: HashMap::default(),
        }
    }

    fn set_root(&mut self, root: PathBuf) {
        self.root = Some(root);
    }

    /// It adds a unix shell style pattern
    fn add_pattern(&mut self, pattern: &str) -> Result<(), PatternError> {
        let pattern = Pattern::new(pattern)?;
        self.patterns.push(pattern);
        Ok(())
    }

    /// It matches the given string against the stored patterns.
    ///
    /// It returns [true] if there's at least a match
    fn matches(&self, source: &str) -> bool {
        let already_checked = self.already_checked.pin();
        if let Some(matches) = already_checked.get(source) {
            return *matches;
        }
        for pattern in &self.patterns {
            if pattern.matches_with(source, self.options) || source.contains(pattern.as_str()) {
                already_checked.insert(source.to_string(), true);
                return true;
            }
        }
        already_checked.insert(source.to_string(), false);
        false
    }

    fn is_empty(&self) -> bool {
        self.patterns.is_empty()
    }

    /// It matches the given path against the stored patterns
    ///
    /// It returns [true] if there's at least one match
    fn matches_path(&self, source: &Path) -> bool {
        if self.is_empty() {
            return false;
        }
        let already_checked = self.already_checked.pin();
        let source_as_string = source.to_str();
        if let Some(source_as_string) = source_as_string {
            if let Some(matches) = already_checked.get(source_as_string) {
                return *matches;
            }
        }
        let matches = self.run_match(source);

        if let Some(source_as_string) = source_as_string {
            already_checked.insert(source_as_string.to_string(), matches);
        }

        matches
    }

    fn run_match(&self, source: &Path) -> bool {
        for pattern in &self.patterns {
            let matches = if pattern.matches_path_with(source, self.options) {
                true
            } else {
                // Here we cover cases where the user specifies single files inside the patterns.
                // The pattern library doesn't support single files, we here we just do a check
                // on contains
                //
                // Given the pattern `out`:
                // - `out/index.html` -> matches
                // - `out/` -> matches
                // - `layout.tsx` -> does not match
                // - `routes/foo.ts` -> does not match
                source
                    .ancestors()
                    .any(|ancestor| ancestor.ends_with(pattern.as_str()))
            };

            if matches {
                return true;
            }
        }
        false
    }
}

impl Diagnostic for PatternError {
    fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.msg)
    }

    fn message(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_markup(markup!({ self.msg }))
    }
}

#[cfg(test)]
mod test {
    use crate::matcher::pattern::MatchOptions;
    use crate::matcher::Inner;
    use std::env;

    #[test]
    fn matches() {
        let current = env::current_dir().unwrap();
        let dir = format!("{}/**/*.rs", current.display());
        let mut ignore = Inner::new(MatchOptions::default());
        ignore.add_pattern(&dir).unwrap();
        let path = env::current_dir().unwrap().join("src/workspace.rs");
        let result = ignore.matches(path.to_str().unwrap());

        assert!(result);
    }

    #[test]
    fn matches_path() {
        let current = env::current_dir().unwrap();
        let dir = format!("{}/**/*.rs", current.display());
        let mut ignore = Inner::new(MatchOptions::default());
        ignore.add_pattern(&dir).unwrap();
        let path = env::current_dir().unwrap().join("src/workspace.rs");
        let result = ignore.matches_path(path.as_path());

        assert!(result);
    }

    #[test]
    fn matches_path_for_single_file_or_directory_name() {
        let dir = "inv";
        let valid_test_dir = "valid/";
        let mut ignore = Inner::new(MatchOptions::default());
        ignore.add_pattern(dir).unwrap();
        ignore.add_pattern(valid_test_dir).unwrap();

        let path = env::current_dir().unwrap().join("tests").join("invalid");
        let result = ignore.matches_path(path.as_path());

        assert!(!result);

        let path = env::current_dir().unwrap().join("tests").join("valid");
        let result = ignore.matches_path(path.as_path());

        assert!(result);
    }

    #[test]
    fn matches_single_path() {
        let dir = "workspace.rs";
        let mut ignore = Inner::new(MatchOptions {
            require_literal_separator: true,
            case_sensitive: true,
            require_literal_leading_dot: true,
        });
        ignore.add_pattern(dir).unwrap();
        let path = env::current_dir().unwrap().join("src/workspace.rs");
        let result = ignore.matches(path.to_str().unwrap());

        assert!(result);
    }
}
