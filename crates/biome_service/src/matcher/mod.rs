use globset::GlobSet;
use std::collections::HashMap;
use std::path::Path;
use std::sync::RwLock;

/// A data structure to use when there's need to match a string or a path a against
/// a unix shell style patterns
#[derive(Debug)]
pub struct Matcher {
    glob: GlobSet,
    /// Whether the string was already checked
    already_checked: RwLock<HashMap<String, bool>>,
}

impl Matcher {
    /// Creates a new Matcher with given options.
    ///
    /// Check [glob website](https://docs.rs/glob/latest/glob/struct.MatchOptions.html) for [MatchOptions]
    pub fn new(glob: GlobSet) -> Self {
        Self {
            glob,
            already_checked: RwLock::new(HashMap::default()),
        }
    }

    /// It matches the given string against the stored patterns.
    ///
    /// It returns [true] if there's at least a match
    pub fn matches(&self, source: &str) -> bool {
        let mut already_ignored = self.already_checked.write().unwrap();
        if let Some(matches) = already_ignored.get(source) {
            return *matches;
        }
        if self.glob.is_match(source) {
            already_ignored.insert(source.to_string(), true);
            return true;
        }
        already_ignored.insert(source.to_string(), false);
        false
    }

    /// It matches the given path against the stored patterns
    ///
    /// It returns [true] if there's a lest a match
    pub fn matches_path(&self, source: &Path) -> bool {
        let mut already_checked = self.already_checked.write().unwrap();
        let source_as_string = source.to_str();
        if let Some(source_as_string) = source_as_string {
            if let Some(matches) = already_checked.get(source_as_string) {
                return *matches;
            }
        }
        let matches = self.glob.is_match(source);

        if let Some(source_as_string) = source_as_string {
            already_checked.insert(source_as_string.to_string(), matches);
        }

        matches
    }
}

#[cfg(test)]
mod test {
    use crate::matcher::Matcher;
    use globset::{Glob, GlobSetBuilder};
    use std::env;

    #[test]
    fn matches() {
        let current = env::current_dir().unwrap();
        let dir = format!("{}/**/*.rs", current.display());
        let ignore = Matcher::new(
            GlobSetBuilder::new()
                .add(Glob::new(&dir).unwrap())
                .build()
                .unwrap(),
        );
        let path = env::current_dir().unwrap().join("src/workspace.rs");
        let result = ignore.matches(path.to_str().unwrap());

        assert!(result);
    }

    #[test]
    fn matches_path() {
        let current = env::current_dir().unwrap();
        let dir = format!("{}/**/*.rs", current.display());
        let ignore = Matcher::new(
            GlobSetBuilder::new()
                .add(Glob::new(&dir).unwrap())
                .build()
                .unwrap(),
        );
        let path = env::current_dir().unwrap().join("src/workspace.rs");
        let result = ignore.matches_path(path.as_path());

        assert!(result);
    }

    #[test]
    fn matches_path_for_single_file_or_directory_name() {
        let dir = "inv";
        let valid_test_dir = "**/valid";
        let ignore = Matcher::new(
            GlobSetBuilder::new()
                .add(Glob::new(&dir).unwrap())
                .add(Glob::new(&valid_test_dir).unwrap())
                .build()
                .unwrap(),
        );

        let path = env::current_dir().unwrap().join("tests").join("invalid");
        let result = ignore.matches_path(path.as_path());

        assert!(!result);

        let path = env::current_dir().unwrap().join("tests").join("valid");
        let result = ignore.matches_path(path.as_path());

        assert!(result);
    }

    #[test]
    fn matches_single_path() {
        let dir = "**.rs";
        let ignore = Matcher::new(
            GlobSetBuilder::new()
                .add(Glob::new(&dir).unwrap())
                .build()
                .unwrap(),
        );
        let path = env::current_dir().unwrap().join("src/workspace.rs");
        let result = ignore.matches(path.to_str().unwrap());

        assert!(result);
    }
}
