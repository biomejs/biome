use crate::configuration::diagnostics::InvalidIgnorePattern;
use crate::{ConfigurationDiagnostic, WorkspaceError};
use globset::GlobSet;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

/// A data structure to use when there's need to match a string or a path a against
/// a unix shell style patterns
#[derive(Debug)]
pub struct Matcher {
    git_ignore: Option<Gitignore>,
    glob: GlobSet,
    /// Whether the string was already checked
    already_checked: RwLock<HashMap<String, bool>>,
}

impl Matcher {
    pub fn empty() -> Self {
        Self {
            git_ignore: None,
            glob: GlobSet::empty(),
            already_checked: RwLock::new(HashMap::default()),
        }
    }

    /// Creates a new Matcher with given options.
    ///
    /// Check [glob website](https://docs.rs/glob/latest/glob/struct.MatchOptions.html) for [MatchOptions]
    pub fn new(glob: GlobSet) -> Self {
        Self {
            glob,
            git_ignore: None,
            already_checked: RwLock::new(HashMap::default()),
        }
    }

    pub fn add_gitignore_matches(
        &mut self,
        path: PathBuf,
        matches: &[String],
    ) -> Result<(), WorkspaceError> {
        let mut gitignore_builder = GitignoreBuilder::new(path.clone());

        for the_match in matches {
            gitignore_builder
                .add_line(Some(path.clone()), the_match)
                .map_err(|err| {
                    WorkspaceError::Configuration(ConfigurationDiagnostic::InvalidIgnorePattern(
                        InvalidIgnorePattern {
                            message: err.to_string(),
                        },
                    ))
                })?;
        }
        let gitignore = gitignore_builder.build().map_err(|err| {
            WorkspaceError::Configuration(ConfigurationDiagnostic::InvalidIgnorePattern(
                InvalidIgnorePattern {
                    message: err.to_string(),
                },
            ))
        })?;
        self.git_ignore = Some(gitignore);
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
        if self.glob.is_match(source) {
            already_ignored.insert(source.to_string(), true);
            return true;
        }
        already_ignored.insert(source.to_string(), false);
        false
    }

    /// If no globs haven't been stored, the function returns [true]
    pub fn is_empty(&self) -> bool {
        self.glob.is_empty()
            && self
                .git_ignore
                .as_ref()
                .map(|ignore| ignore.is_empty())
                .unwrap_or(true)
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
            let matches = self.glob.is_match(source)
                || self
                    .git_ignore
                    .as_ref()
                    .map(|ignore| {
                        ignore
                            .matched(source_as_string, source.is_dir())
                            .is_ignore()
                    })
                    .unwrap_or_default();

            already_checked.insert(source_as_string.to_string(), matches);

            matches
        } else {
            false
        }
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
                .add(Glob::new(dir).unwrap())
                .add(Glob::new(valid_test_dir).unwrap())
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
        let dir = env::current_dir().unwrap().join("src/workspace.rs");
        let ignore = Matcher::new(
            GlobSetBuilder::new()
                .add(Glob::new(&dir.display().to_string()).unwrap())
                .build()
                .unwrap(),
        );
        let path = env::current_dir().unwrap().join("src/workspace.rs");
        let result = ignore.matches(path.to_str().unwrap());

        assert!(result);
    }
}
