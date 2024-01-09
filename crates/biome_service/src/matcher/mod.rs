use crate::configuration::diagnostics::InvalidIgnorePattern;
use crate::{ConfigurationDiagnostic, WorkspaceError};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

/// A data structure to use when there's need to match a string or a path a against
/// a unix shell style patterns
#[derive(Debug)]
pub struct Matcher {
    git_ignore: Option<Gitignore>,
    /// The globs
    ignore: Gitignore,
    /// Whether the string was already checked
    already_checked: RwLock<HashMap<String, bool>>,
}

impl TryFrom<&str> for Matcher {
    type Error = WorkspaceError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut builder = GitignoreBuilder::new("");
        builder
            .add_line(
                Some(PathBuf::new()),
                value.strip_prefix("./").unwrap_or(value),
            )
            .map_err(|error| {
                WorkspaceError::Configuration(ConfigurationDiagnostic::new_invalid_ignore_pattern(
                    value.to_string(),
                    error.to_string(),
                ))
            })?;

        let ignore = builder.build().map_err(|error| {
            WorkspaceError::Configuration(ConfigurationDiagnostic::InvalidIgnorePattern(
                InvalidIgnorePattern {
                    message: error.to_string(),
                },
            ))
        })?;

        Ok(Matcher::new(ignore))
    }
}

impl Matcher {
    pub fn empty() -> Self {
        Self {
            git_ignore: None,
            ignore: Gitignore::empty(),
            already_checked: RwLock::new(HashMap::default()),
        }
    }

    /// Creates a new Matcher with given options.
    ///
    /// Check [glob website](https://docs.rs/glob/latest/glob/struct.MatchOptions.html) for [MatchOptions]
    pub fn new(ignore: Gitignore) -> Self {
        Self {
            ignore,
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

    /// If no globs haven't been stored, the function returns [true]
    pub fn is_empty(&self) -> bool {
        self.ignore.is_empty()
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
            let matches = self
                .ignore
                .matched_path_or_any_parents(source_as_string, source.is_dir())
                .is_ignore()
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
    use ignore::gitignore::GitignoreBuilder;
    use std::path::Path;

    #[test]
    fn matches() {
        let dir = "src/**/*.rs";
        let ignore = Matcher::try_from(dir).unwrap();
        let path = Path::new("src/workspace.rs");
        let result = ignore.matches_path(path);

        assert!(result);
    }

    #[test]
    fn matches_path_for_single_file_or_directory_name() {
        let dir = "inv";
        let valid_test_dir = "**/valid";
        let ignore = Matcher::new(
            GitignoreBuilder::new("")
                .add_line(None, dir)
                .unwrap()
                .add_line(None, valid_test_dir)
                .unwrap()
                .build()
                .unwrap(),
        );

        let path = Path::new("tests").join("invalid");
        let result = ignore.matches_path(path.as_path());

        assert!(!result);

        let path = Path::new("tests").join("valid");
        let result = ignore.matches_path(path.as_path());

        assert!(result);
    }

    #[test]
    fn matches_single_path() {
        let dir = Path::new("src/workspace.rs");
        let ignore = Matcher::try_from(dir.to_str().unwrap()).unwrap();
        let path = Path::new("src/workspace.rs");
        let result = ignore.matches_path(path);

        assert!(result);
    }

    #[test]
    fn matches_single_path_2() {
        let ignore = Matcher::try_from("test/").unwrap();
        let result = ignore.matches_path(Path::new("test/workspace.rs"));

        assert!(result);
    }
}
