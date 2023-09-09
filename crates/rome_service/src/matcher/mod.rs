pub mod pattern;

use biome_console::markup;
use biome_diagnostics::Diagnostic;
pub use pattern::{MatchOptions, Pattern, PatternError};
use std::collections::HashMap;
use std::path::Path;
use std::sync::RwLock;

/// A data structure to use when there's need to match a string or a path a against
/// a unix shell style patterns
#[derive(Debug)]
pub struct Matcher {
    patterns: Vec<Pattern>,
    options: MatchOptions,
    already_ignored: RwLock<HashMap<String, bool>>,
}

impl Matcher {
    /// Creates a new Matcher with given options.
    ///
    /// Check [glob website](https://docs.rs/glob/latest/glob/struct.MatchOptions.html) for [MatchOptions]
    pub fn new(options: MatchOptions) -> Self {
        Self {
            patterns: Vec::new(),
            options,
            already_ignored: RwLock::new(HashMap::default()),
        }
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
        let mut already_ignored = self.already_ignored.write().unwrap();
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

    /// It matches the given path against the stored patterns
    ///
    /// It returns [true] if there's a lest a match
    pub fn matches_path(&self, source: &Path) -> bool {
        let mut already_ignored = self.already_ignored.write().unwrap();
        let source_as_string = source.to_str();
        if let Some(source_as_string) = source_as_string {
            if let Some(matches) = already_ignored.get(source_as_string) {
                return *matches;
            }
        }
        let matches = {
            for pattern in &self.patterns {
                let matches = if pattern.matches_path_with(source, self.options) {
                    true
                } else {
                    // Here we cover cases where the user specifies single files inside the patterns.
                    // The pattern library doesn't support single files, we here we just do a check
                    // on contains
                    source_as_string.map_or(false, |source| source.contains(pattern.as_str()))
                };

                if matches {
                    return true;
                }
            }

            false
        };

        if let Some(source_as_string) = source_as_string {
            already_ignored.insert(source_as_string.to_string(), matches);
        }

        matches
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
