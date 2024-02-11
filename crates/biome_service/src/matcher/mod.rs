pub mod pattern;

use biome_console::markup;
use biome_diagnostics::Diagnostic;
pub use pattern::{MatchOptions, Pattern, PatternError};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

/// A data structure to use when there's need to match a string or a path a against
/// a unix shell style patterns
#[derive(Debug)]
pub struct Matcher {
    root: Option<PathBuf>,
    patterns: Vec<Pattern>,
    options: MatchOptions,
    /// Whether the string was already checked
    already_checked: RwLock<HashMap<String, bool>>,
}

impl Matcher {
    /// Creates a new Matcher with given options.
    ///
    /// Check [glob website](https://docs.rs/glob/latest/glob/struct.MatchOptions.html) for [MatchOptions]
    pub fn new(options: MatchOptions) -> Self {
        Self {
            root: None,
            patterns: Vec::new(),
            options,
            already_checked: RwLock::new(HashMap::default()),
        }
    }

    pub fn empty() -> Self {
        Self {
            root: None,
            patterns: Vec::new(),
            options: MatchOptions::default(),
            already_checked: RwLock::new(HashMap::default()),
        }
    }

    pub fn set_root(&mut self, root: PathBuf) {
        self.root = Some(root);
    }

    /// It adds a unix shell style pattern
    pub fn add_pattern(&mut self, pattern: &str) -> Result<(), PatternError> {
        let pattern = Pattern::new(pattern)?;
        self.patterns.push(pattern);
        Ok(())
    }

    /// It matches the given string against the stored patterns.
    ///
    /// It returns [true] if there's at least a match
    pub fn matches(&self, source: &str) -> bool {
        let mut already_ignored = self.already_checked.write().unwrap();
        if let Some(matches) = already_ignored.get(source) {
            return *matches;
        }
        for pattern in &self.patterns {
            if pattern.matches_with(source, self.options) || source.contains(pattern.as_str()) {
                already_ignored.insert(source.to_string(), true);
                return true;
            }
        }
        already_ignored.insert(source.to_string(), false);
        false
    }

    pub fn is_empty(&self) -> bool {
        self.patterns.is_empty()
    }

    /// It matches the given path against the stored patterns
    ///
    /// It returns [true] if there's at least one match
    pub fn matches_path(&self, source: &Path) -> bool {
        if self.is_empty() {
            return false;
        }
        let mut already_checked = self.already_checked.write().unwrap();
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
    use crate::matcher::Matcher;
    use std::env;

    #[test]
    fn matches() {
        let current = env::current_dir().unwrap();
        let dir = format!("{}/**/*.rs", current.display());
        let mut ignore = Matcher::new(MatchOptions::default());
        ignore.add_pattern(&dir).unwrap();
        let path = env::current_dir().unwrap().join("src/workspace.rs");
        let result = ignore.matches(path.to_str().unwrap());

        assert!(result);
    }

    #[test]
    fn matches_path() {
        let current = env::current_dir().unwrap();
        let dir = format!("{}/**/*.rs", current.display());
        let mut ignore = Matcher::new(MatchOptions::default());
        ignore.add_pattern(&dir).unwrap();
        let path = env::current_dir().unwrap().join("src/workspace.rs");
        let result = ignore.matches_path(path.as_path());

        assert!(result);
    }

    #[test]
    fn matches_path_for_single_file_or_directory_name() {
        let dir = "inv";
        let valid_test_dir = "valid/";
        let mut ignore = Matcher::new(MatchOptions::default());
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
        let mut ignore = Matcher::new(MatchOptions {
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
